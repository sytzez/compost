use std::collections::HashMap;
use crate::sem::typ::Type;

// A trait has input types and an output type. It can be defined on classes and structs.
#[derive(Eq, PartialEq, Hash)]
pub struct Trait {
    pub inputs: HashMap<String, Type>,
    pub output: Type,
}
