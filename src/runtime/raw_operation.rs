use crate::ast::raw_value::RawValue;
use crate::runtime::instance::Instance;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

/// Performs an operation on a raw value.
pub fn raw_operation(
    value: &RawValue,
    trayt: &str,
    inputs: HashMap<String, Rc<Instance>>,
) -> RawValue {
    match trayt {
        "Op\\Add" => add(value, &rhs(inputs)),
        "Op\\Sub" => sub(value, &rhs(inputs)),
        "Op\\Mul" => mul(value, &rhs(inputs)),
        "Op\\Div" => div(value, &rhs(inputs)),
        "Op\\Neg" => neg(value),
        "String" => to_string(value),
        _ => panic!("No such raw trait: {}", trayt),
    }
}

fn rhs(inputs: HashMap<String, Rc<Instance>>) -> RawValue {
    if let Instance::Raw(value) = inputs.get("rhs").expect("No RHS given").borrow() {
        value.clone()
    } else {
        panic!("RHS is not a raw value")
    }
}

fn int(value: &RawValue) -> i64 {
    if let RawValue::Int(value) = value {
        *value
    } else {
        panic!("Value is not an int")
    }
}

fn string(value: &RawValue) -> &str {
    if let RawValue::String(value) = value {
        value
    } else {
        panic!("Value is not a string")
    }
}

fn add(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(*value + int(rhs)),
        RawValue::String(value) => RawValue::String(value.to_string() + string(rhs)),
    }
}

fn sub(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(*value - int(rhs)),
        RawValue::String(_) => panic!("Subtraction not supported by string"),
    }
}

fn mul(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(*value * int(rhs)),
        RawValue::String(_) => panic!("Multiplication not supported by string"),
    }
}

fn div(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(*value / int(rhs)),
        RawValue::String(_) => panic!("Division not supported by string"),
    }
}

fn neg(value: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(-*value),
        RawValue::String(_) => panic!("Negation not supported by string"),
    }
}

fn to_string(value: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::String(value.to_string()),
        RawValue::String(value) => RawValue::String(value.to_string()),
    }
}
