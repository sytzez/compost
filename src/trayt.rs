use crate::scope::ReferencePath;
use crate::typ::Type;

// A trait has input types and an output type. It can be defined on classes and structs.
#[derive(Eq, PartialEq, Hash)]
pub struct Trait {
    pub reference_path: ReferencePath,
    pub inputs: Vec<Type>,
    pub output: Type,
}
