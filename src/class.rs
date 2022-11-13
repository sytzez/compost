use crate::definition::Definition;
use crate::expression::Expression;
use crate::instance::Instance;
use crate::lett::Let;
use crate::scope::ReferencePath;
use crate::typ::{combine_types, Type};
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

    pub fn instantiate(
        self: &Rc<Self>,
        dependencies: HashMap<String, Rc<Instance>>,
    ) -> ClassInstance {
        ClassInstance {
            class: Rc::clone(self),
            dependencies,
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
            .map(|path| Type::Trait(path))
            .collect::<Vec<_>>();

        combine_types(types)
    }
}

pub struct ClassInstance {
    class: Rc<Class>,
    dependencies: HashMap<String, Rc<Instance>>,
}

impl ClassInstance {
    pub fn class(&self) -> &Rc<Class> {
        &self.class
    }

    pub fn dependency(&self, name: &str) -> &Rc<Instance> {
        self.dependencies
            .get(name)
            .expect(&format!("Dependency {} does not exist", name))
    }

    pub fn dependencies(&self) -> HashMap<String, Rc<Instance>> {
        self.dependencies.clone()
    }
}
