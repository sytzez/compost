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
        "Op\\Eq" => eq(value, &rhs(inputs)),
        "Op\\Lt" => lt(value, &rhs(inputs)),
        "Op\\Gt" => gt(value, &rhs(inputs)),
        "Op\\And" => and(value, &rhs(inputs)),
        "Op\\Or" => or(value, &rhs(inputs)),
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

fn int(value: &RawValue) -> &i64 {
    if let RawValue::Int(value) = value {
        value
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

fn bool(value: &RawValue) -> &bool {
    if let RawValue::Bool(value) = value {
        value
    } else {
        panic!("Value is not a bool")
    }
}

fn add(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(value + int(rhs)),
        RawValue::String(value) => RawValue::String(value.to_string() + string(rhs)),
        RawValue::Bool(_) => panic!("Addition not supported by bool"),
    }
}

fn sub(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(value - int(rhs)),
        RawValue::String(_) => panic!("Subtraction not supported by string"),
        RawValue::Bool(_) => panic!("Subtraction not supported by bool"),
    }
}

fn mul(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(value * int(rhs)),
        RawValue::String(_) => panic!("Multiplication not supported by string"),
        RawValue::Bool(_) => panic!("Multiplication not supported by bool"),
    }
}

fn div(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(value / int(rhs)),
        RawValue::String(_) => panic!("Division not supported by string"),
        RawValue::Bool(_) => panic!("Division not supported by bool"),
    }
}

fn neg(value: &RawValue) -> RawValue {
    match value {
        RawValue::Int(value) => RawValue::Int(-*value),
        RawValue::String(_) => panic!("Negation not supported by string"),
        RawValue::Bool(_) => panic!("Negation not supported by bool"),
    }
}

fn eq(value: &RawValue, rhs: &RawValue) -> RawValue {
    let bool = match value {
        RawValue::Int(value) => value == int(rhs),
        RawValue::String(value) => value == string(rhs),
        RawValue::Bool(value) => value == bool(rhs),
    };
    RawValue::Bool(bool)
}

fn lt(value: &RawValue, rhs: &RawValue) -> RawValue {
    let bool = match value {
        RawValue::Int(value) => value < int(rhs),
        RawValue::String(value) => value.len() < string(rhs).len(),
        RawValue::Bool(_) => panic!("Less than not supported by bool"),
    };
    RawValue::Bool(bool)
}

fn gt(value: &RawValue, rhs: &RawValue) -> RawValue {
    let bool = match value {
        RawValue::Int(value) => value > int(rhs),
        RawValue::String(value) => value.len() > string(rhs).len(),
        RawValue::Bool(_) => panic!("Greater than not supported by bool"),
    };
    RawValue::Bool(bool)
}

fn and(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Bool(value) => RawValue::Bool(*value && *bool(rhs)),
        RawValue::Int(_) => panic!("And operation not supported by int"),
        RawValue::String(_) => panic!("And operation not supported by string"),
    }
}

fn or(value: &RawValue, rhs: &RawValue) -> RawValue {
    match value {
        RawValue::Bool(value) => RawValue::Bool(*value || *bool(rhs)),
        RawValue::Int(_) => panic!("And operation not supported by int"),
        RawValue::String(_) => panic!("And operation not supported by string"),
    }
}

fn to_string(value: &RawValue) -> RawValue {
    let string = match value {
        RawValue::Int(value) => value.to_string(),
        RawValue::String(value) => value.to_string(),
        RawValue::Bool(value) => if *value { "true" } else { "false" }.to_string(),
    };
    RawValue::String(string)
}
