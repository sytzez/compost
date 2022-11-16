use crate::ast::raw_value::RawValue;
use crate::runtime::instance::Instance;
use crate::sem::strukt::Struct;
use std::collections::HashMap;
use std::rc::Rc;

pub struct StructInstance {
    strukt: Rc<Struct>,
    values: HashMap<String, RawValue>,
}

impl StructInstance {
    pub fn new(strukt: &Rc<Struct>, values: HashMap<String, RawValue>) -> Self {
        Self {
            strukt: Rc::clone(strukt),
            values,
        }
    }

    pub fn strukt(&self) -> &Rc<Struct> {
        &self.strukt
    }

    pub fn value(&self, name: &str) -> &RawValue {
        self.values
            .get(name)
            .unwrap_or_else(|| panic!("Field {} does not exist", name))
    }

    pub fn values(&self) -> HashMap<String, Rc<Instance>> {
        self.values
            .iter()
            .map(|(name, value)| (name.clone(), Rc::new(Instance::Raw(value.clone()))))
            .collect()
    }
}
