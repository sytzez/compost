use crate::ast::raw_value::RawValue;
use crate::runtime::instance::Instance;
use crate::sem::strukt::Struct;
use std::collections::HashMap;
use std::rc::Rc;

pub struct StructInstance {
    strukt: Rc<Struct>,
    fields: HashMap<String, RawValue>,
}

impl StructInstance {
    pub fn new(strukt: &Rc<Struct>, fields: HashMap<String, RawValue>) -> Self {
        Self {
            strukt: Rc::clone(strukt),
            fields,
        }
    }

    pub fn strukt(&self) -> &Rc<Struct> {
        &self.strukt
    }

    pub fn field(&self, name: &str) -> &RawValue {
        self.fields
            .get(name)
            .unwrap_or_else(|| panic!("Field {} does not exist", name))
    }

    pub fn fields(&self) -> HashMap<String, Rc<Instance>> {
        self.fields
            .iter()
            .map(|(name, value)| (name.clone(), Rc::new(Instance::Raw(value.clone()))))
            .collect()
    }
}
