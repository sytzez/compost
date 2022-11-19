use std::ops::AddAssign;
use crate::ast::parser::Parser;
use crate::error::CResult;
use crate::lex::tokenizer::LeveledToken;
use crate::sem::scope::ReferencePath;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Type {
    Trait(ReferencePath),
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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum RawType {
    Int,
    String,
}

impl Parser for Type {
    fn matches(tokens: &[LeveledToken]) -> bool {
        true
    }

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self> {
        // TODO: actually implement
        position.add_assign(1);

        Ok(Type::Void)
    }
}
