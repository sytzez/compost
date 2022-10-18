use crate::class::{Class, Struct, Trait};

pub struct Module {
    pub classes: Vec<(String, Class)>,
    pub structs: Vec<(String, Struct)>,
    pub traits: Vec<(String, Trait)>,
    pub lets: Vec<(String, ())>,
    pub modules: Vec<(String, Box<Module>)>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            classes: vec![],
            structs: vec![],
            traits: vec![],
            lets: vec![],
            modules: vec![],
        }
    }
}