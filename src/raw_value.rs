use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use crate::instance::Instance;
use crate::scope::{path, ReferencePath};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RawValue {
    Int(i64),
    UInt(u64),
    String(String),
}

impl RawValue {
    pub fn call(&self, trait_path: &ReferencePath, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        if trait_path == &path("Op\\Add") {
            return self.add(inputs);
        }

        if trait_path == &path("Op\\Sub") {
            return self.sub(inputs);
        }

        panic!()
    }

    pub fn add(&self, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = Self::rhs(inputs);

        match self {
            RawValue::Int(value) => RawValue::Int(*value + rhs.int()),
            RawValue::UInt(value) => RawValue::UInt(*value + rhs.uint()),
            _ => panic!("todo"),
        }
    }

    pub fn sub(&self, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = Self::rhs(inputs);

        match self {
            RawValue::Int(value) => RawValue::Int(*value - rhs.int()),
            RawValue::UInt(value) => RawValue::UInt(*value - rhs.uint()),
            _ => panic!("todo"),
        }
    }

    fn int(&self) -> i64 {
        if let RawValue::Int(value) = self {
            *value
        } else {
            panic!("{:?} is not an Int", self);
        }
    }

    fn uint(&self) -> u64 {
        if let RawValue::UInt(value) = self {
            *value
        } else {
            panic!("{:?} is not an UInt", self);
        }
    }


    fn rhs(inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = inputs.get("rhs").expect("No rhs given");

        match rhs.borrow() {
            Instance::Raw(value) => value.clone(),
            _ => panic!(),
        }
    }
}
