use crate::class::Class;
use crate::lett::Let;
use crate::strukt::Struct;
use crate::trayt::Trait;

// A module can contain classes, structs, traits, lets and other modules.
pub struct Module {
    pub classes: Vec<(String, Class)>,
    pub structs: Vec<(String, Struct)>,
    pub traits: Vec<(String, Trait)>,
    pub lets: Vec<(String, Let)>,
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