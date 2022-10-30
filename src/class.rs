use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;
use crate::scope::ReferencePath;
use crate::definition::Definition;
use crate::instance::Instance;
use crate::typ::Type;

// A class has a set of dependencies of certain types, and a set of trait definitions.
pub struct Class {
    pub dependencies: HashMap<String, Type>,
    pub definitions: HashMap<ReferencePath, Definition>
}

pub struct ClassInstance {
    pub class: Rc<Class>,
    pub dependencies: HashMap<String, Instance>
}
