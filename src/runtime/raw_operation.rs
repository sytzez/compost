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
        _ => unreachable!("No such raw trait: {}", trayt),
    }
}

fn rhs(inputs: HashMap<String, Rc<Instance>>) -> RawValue {
    if let Instance::Raw(value) = inputs.get("rhs").unwrap().borrow() {
        value.clone()
    } else {
        unreachable!()
    }
}

fn int(value: &RawValue) -> i64 {
    if let RawValue::Int(value) = value {
        *value
    } else {
        unreachable!()
    }
}

fn string(value: &RawValue) -> &str {
    if let RawValue::String(value) = value {
        value
    } else {
        unreachable!()
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
        RawValue::String(_) => unreachable!(),
    }
}

fn mul(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(*value * int(rhs)),
        RawValue::String(_) => unreachable!(),
    }
}

fn div(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(*value / int(rhs)),
        RawValue::String(_) => unreachable!(),
    }
}

fn neg(value: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(-*value),
        RawValue::String(_) => unreachable!(),
    }
}

fn to_string(value: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::String(value.to_string()),
        RawValue::String(_) => unreachable!(),
    }
}
