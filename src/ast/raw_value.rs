use crate::runtime::instance::Instance;
use crate::sem::scope::ReferencePath;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RawValue {
    Int(i64),
    String(String),
}

impl RawValue {
    pub fn call(
        &self,
        trait_path: &ReferencePath,
        inputs: HashMap<String, Rc<Instance>>,
    ) -> RawValue {
        match trait_path.join("\\").borrow() {
            "Op\\Add" => self.add(inputs),
            "Op\\Sub" => self.sub(inputs),
            "Op\\Mul" => self.mul(inputs),
            "Op\\Neg" => self.neg(),
            "toString" => self.to_string(),
            _ => panic!("Unknown raw trait {} ", trait_path.join("\\")),
        }
    }

    fn add(&self, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = Self::rhs(inputs);

        match self {
            RawValue::Int(value) => RawValue::Int(*value + rhs.int()),
            RawValue::String(value) => RawValue::String(value.clone() + rhs.string()),
        }
    }

    fn sub(&self, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = Self::rhs(inputs);

        match self {
            RawValue::Int(value) => RawValue::Int(*value - rhs.int()),
            RawValue::String(_) => panic!("Can't subtract strings"),
        }
    }

    fn mul(&self, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = Self::rhs(inputs);

        match self {
            RawValue::Int(value) => RawValue::Int(*value * rhs.int()),
            RawValue::String(_) => panic!("Can't multiply strings"),
        }
    }

    fn neg(&self) -> RawValue {
        match self {
            RawValue::Int(value) => RawValue::Int(-*value),
            RawValue::String(_) => panic!("Can't negate strings"),
        }
    }

    fn to_string(&self) -> RawValue {
        let string = match self {
            RawValue::Int(value) => value.to_string(),
            RawValue::String(value) => value.clone(),
        };

        RawValue::String(string)
    }

    fn int(&self) -> i64 {
        if let RawValue::Int(value) = self {
            *value
        } else {
            panic!("{:?} is not an Int", self)
        }
    }

    fn string(&self) -> &str {
        if let RawValue::String(value) = self {
            value
        } else {
            panic!("{:?} is not a String", self)
        }
    }

    fn rhs(inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = inputs.get("rhs").expect("No rhs given");

        match rhs.borrow() {
            Instance::Raw(value) => value.clone(),
            _ => panic!("Right hand side of operation is not a raw type"),
        }
    }
}
