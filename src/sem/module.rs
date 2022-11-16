use crate::sem::class::Class;
use crate::sem::definition::Definition;
use crate::sem::lett::Let;
use crate::sem::strukt::Struct;
use crate::sem::trayt::Trait;

// A module can contain classes, structs, traits, lets and other modules.
pub struct Module {
    pub classes: Vec<(String, Class)>,
    pub structs: Vec<(String, Struct)>,
    pub traits: Vec<(String, Trait)>,
    pub lets: Vec<(String, Let)>,
    pub defs: Vec<(String, Definition)>,
    pub modules: Vec<(String, Box<Module>)>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            classes: vec![],
            structs: vec![],
            traits: vec![],
            lets: vec![],
            defs: vec![],
            modules: vec![],
        }
    }
}
