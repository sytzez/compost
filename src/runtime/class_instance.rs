use crate::runtime::instance::Instance;
use crate::sem::class::Class;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct ClassInstance {
    class: Rc<Class>,
    dependencies: HashMap<String, Rc<Instance>>,
}

impl ClassInstance {
    pub fn new(class: &Rc<Class>, dependencies: HashMap<String, Rc<Instance>>) -> Self {
        Self {
            class: Rc::clone(class),
            dependencies,
        }
    }

    pub fn class(&self) -> &Rc<Class> {
        &self.class
    }

    pub fn dependencies(&self) -> HashMap<String, Rc<Instance>> {
        self.dependencies.clone()
    }
}
