use crate::ast::expression::Expression;
use crate::ast::typ::{combine_types, Type};
use crate::runtime::instance::Instance;
use crate::sem::definition::Definition;
use crate::sem::lett::Let;
use crate::sem::scope::ReferencePath;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;

// A class has a set of dependencies of certain types, and a set of trait definitions.
pub struct Class {
    pub dependencies: HashMap<String, Type>,
    pub definitions: HashMap<ReferencePath, Definition>,
}

impl Class {
    pub fn new() -> Self {
        Class {
            dependencies: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, name: String, typ: Type) {
        self.dependencies.insert(name, typ);
    }

    pub fn add_definition(&mut self, trait_path: ReferencePath, definition: Definition) {
        self.definitions.insert(trait_path, definition);
    }

    pub fn constructor(self: &Rc<Self>) -> Let {
        Let {
            inputs: self.dependencies.clone(),
            outputs: [(String::new(), self.interface())].into(),
            expression: Expression::ConstructClass(Rc::clone(self)),
        }
    }

    pub fn interface(&self) -> Type {
        let types = self
            .definitions
            .keys()
            .cloned()
            .map(Type::Trait)
            .collect::<Vec<_>>();

        combine_types(types)
    }
}
