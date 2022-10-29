use crate::scope::ReferencePath;

#[derive(Eq, PartialEq, Hash)]
pub enum Type {
    Trait(ReferencePath),
    Raw(RawType),
    And(Box<Type>, Box<Type>),
    Or(Box<Type>, Box<Type>),
    // Self, the class or struct the trait is defined on
    Zelf,
    // No traits, no interaction possible
    Closed,
}

pub fn combine_types(types: Vec<Type>) -> Type {
    let mut combined = None;

    for typ in types {
        combined = match combined {
            None => Some(typ),
            Some(prev_type) => Some(
                Type::And(
                    Box::new(prev_type),
                    Box::new(typ)
                )
            ),
        }
    }

    match combined {
        Some(typ) => typ,
        None => Type::Closed,
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum RawType {
    Int,
    UInt,
    Float,
    String,
}