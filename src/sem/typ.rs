use crate::ast::type_statement::{RawType, TypeStatement, TypeStatementType};
use crate::error::{CResult, ErrorMessage};
use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::trayt::{interface_type, Trait};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::ast::Statement;

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
        let typ = match &statement.typ {
            TypeStatementType::Name(name) => {
                if let Ok(interface) = context.interfaces.resolve(name, path) {
                    interface_type(interface.borrow().as_ref())
                } else if let Ok(trayt) = context.traits.resolve(name, path) {
                    Type::Trait(trayt)
                } else {
                    return statement.error(ErrorMessage::NoModuleOrTrait(name.clone()));
                }
            }
            TypeStatementType::AtName(name) => {
                if let Ok(trayt) = context.traits.resolve(name, path) {
                    Type::Trait(trayt)
                } else {
                    return statement.error(ErrorMessage::NoTrait(name.clone()));
                }
            }
            TypeStatementType::And(a, b) => Type::And(
                Box::new(Type::analyse(a, context, path)?),
                Box::new(Type::analyse(b, context, path)?),
            ),
            TypeStatementType::Or(a, b) => Type::Or(
                Box::new(Type::analyse(a, context, path)?),
                Box::new(Type::analyse(b, context, path)?),
            ),
            TypeStatementType::Zelf => Type::Zelf,
            TypeStatementType::Void => Type::Void,
        };

        Ok(typ)
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
            Type::Or(a, b) => a
                .callable_traits(scope)
                .union(&b.callable_traits(scope))
                .cloned()
                .collect(),
            Type::Raw(raw_type) => match raw_type {
                RawType::Int => [
                    "Op\\Add", "Op\\Sub", "Op\\Mul", "Op\\Div", "Op\\Neg", "Op\\Eq", "Op\\Lt",
                    "Op\\Gt", "String",
                ]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
                RawType::String => ["Op\\Add", "Op\\Eq", "String"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                RawType::Bool => ["Op\\Eq", "Op\\And", "Op\\Or"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            },
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Trait(t) => write!(f, "{}", t.borrow().full_name),
            Type::Raw(t) => match t {
                RawType::Int => write!(f, "int"),
                RawType::String => write!(f, "string"),
                RawType::Bool => write!(f, "bool"),
            },
            Type::And(a, b) => write!(f, "{} & {}", a, b),
            Type::Or(a, b) => write!(f, "{} | {}", a, b),
            Type::Zelf => write!(f, "Self"),
            Type::Void => write!(f, "?"),
        }
    }
}
