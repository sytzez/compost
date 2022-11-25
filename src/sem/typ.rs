use crate::ast::type_statement::{RawType, TypeStatement};
use crate::error::{error, CResult, ErrorMessage};
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::trayt::{interface_type, Trait};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub enum Type {
    Trait(Rc<RefCell<Trait>>),
    Raw(RawType),
    And(Box<Type>, Box<Type>),
    Or(Box<Type>, Box<Type>),
    // Self, the class or struct the trait is defined on
    Zelf,
    // No traits, no interaction possible
    Void,
}

pub fn combine_types(types: Vec<Type>) -> Type {
    let mut combined = None;

    for typ in types {
        combined = match combined {
            None => Some(typ),
            Some(prev_type) => Some(Type::And(Box::new(prev_type), Box::new(typ))),
        }
    }

    match combined {
        Some(typ) => typ,
        None => Type::Void,
    }
}

impl Type {
    pub fn analyse(
        statement: &TypeStatement,
        context: &SemanticContext,
        path: &str,
    ) -> CResult<Self> {
        let typ = match statement {
            TypeStatement::Name(name) => {
                if let Ok(interface) = context.interfaces.resolve(name, path) {
                    interface_type(interface.as_ref())
                } else if let Ok(trayt) = context.traits.resolve(name, path) {
                    Type::Trait(trayt)
                } else {
                    return error(ErrorMessage::NoModuleOrTrait(name.clone()));
                }
            }
            TypeStatement::And(a, b) => Type::And(
                Box::new(Type::analyse(a, context, path)?),
                Box::new(Type::analyse(b, context, path)?),
            ),
            TypeStatement::Or(a, b) => Type::Or(
                Box::new(Type::analyse(a, context, path)?),
                Box::new(Type::analyse(b, context, path)?),
            ),
            TypeStatement::Zelf => Type::Zelf,
            TypeStatement::Void => Type::Void,
        };

        Ok(typ)
    }

    pub fn fits(&self, _other: &Type) -> bool {
        todo!()
    }
}
