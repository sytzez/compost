use crate::ast::type_statement::{RawType, TypeStatement};
use crate::error::{error, CResult, ErrorMessage};
use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::trayt::{interface_type, Trait};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
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
                    interface_type(interface.borrow().as_ref())
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

    /// If the current type is suitable to be used where the given type is required.
    pub fn fits(&self, _other: &Type) -> bool {
        todo!()
    }

    /// Outputs a list of traits that can be called on an instance of this type.
    pub fn callable_traits(&self, scope: &SemanticScope) -> BTreeSet<String> {
        match self {
            Type::Void => [].into(),
            Type::Trait(trayt) => [trayt.borrow().full_name.clone()].into(),
            Type::And(a, b) => a
                .callable_traits(scope)
                .into_iter()
                .chain(b.callable_traits(scope).into_iter())
                .collect(),
            Type::Zelf => match &scope.zelf {
                None => [].into(),
                Some(Type::Zelf) => panic!("Recursion!"),
                Some(zelf) => zelf.callable_traits(scope),
            },
            Type::Or(_a, _b) => todo!("Do union operation"),
            Type::Raw(raw_type) => match raw_type {
                RawType::Int => [
                    "Op\\Add", "Op\\Sub", "Op\\Mul", "Op\\Div", "Op\\Neg", "String",
                ]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
                RawType::String => ["Op\\Add", "String"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            },
        }
    }
}
