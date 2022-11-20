use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::type_statement::{RawType, TypeStatement};
use crate::error::{CResult};
use crate::sem::scope::path;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::trayt::Trait;

#[derive(Eq, PartialEq, Clone)]
pub enum Type {
    Trait(Rc<RefCell<Option<Trait>>>),
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
    pub fn analyse(statement: &TypeStatement, context: &SemanticContext) -> CResult<Self> {
        let typ = match statement {
            TypeStatement::Name(name) => {
                // Will be a type or an interface
                // TODO: mod interfaces
                Type::Trait(context.traits.resolve(&path(name))?)
            }
            TypeStatement::And(a, b) => {
                Type::And(
                    Box::new(Type::analyse(a, context)?),
                    Box::new(Type::analyse(b, context)?),
                )
            }
            TypeStatement::Or(a, b) => {
                Type::Or(
                    Box::new(Type::analyse(a, context)?),
                    Box::new(Type::analyse(b, context)?),
                )
            }
            TypeStatement::Zelf => {
                if let Some(typ) = &context.zelf {
                    typ.clone()
                } else {
                    Type::Zelf
                }
            },
            TypeStatement::Void => Type::Void
        };

        Ok(typ)
    }

    pub fn fits(&self, _other: &Type) -> bool {
        todo!()
    }
}