use crate::ast::raw_value::RawValue;
use crate::ast::type_statement::RawType;
use crate::error::CResult;
use crate::runtime::class_instance::ClassInstance;
use crate::runtime::evaluate::evaluate;
use crate::runtime::raw_operation::raw_operation;
use crate::runtime::struct_instance::StructInstance;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::trayt::Trait;
use crate::sem::typ::Type;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// An instantiated class or struct, or a raw value.
#[derive(Debug)]
pub enum Instance {
    Class(ClassInstance),
    Struct(StructInstance),
    Raw(RawValue),
    Void,
}

impl Instance {
    pub fn call(
        self: &Rc<Self>,
        trayt: Rc<RefCell<Trait>>,
        mut inputs: HashMap<String, Rc<Instance>>,
    ) -> Rc<Instance> {
        let definitions = match self.as_ref() {
            Instance::Struct(instance) => &instance.strukt().definitions,
            Instance::Class(instance) => &instance.class().definitions,
            Instance::Raw(value) => {
                let result = raw_operation(value, &trayt.as_ref().borrow().full_name, inputs);

                return Rc::new(Instance::Raw(result));
            }
            Instance::Void => panic!("Can't call trait on void"),
        };

        let evaluation = &definitions
            .iter()
            .find(|(t, _)| t == &trayt)
            .map(|(_, eval)| eval)
            .unwrap_or_else(|| {
                panic!(
                    "Couldn't find trait {} on instance",
                    trayt.as_ref().borrow().full_name
                )
            });

        inputs.extend(self.locals());

        evaluate(evaluation, inputs, Some(Rc::clone(self)))
    }

    fn locals(&self) -> HashMap<String, Rc<Instance>> {
        match self {
            Instance::Struct(instance) => instance.fields(),
            Instance::Class(instance) => instance.dependencies(),
            _ => unreachable!(),
        }
    }

    /// Whether the current instance satisfies the given type.
    pub fn satisfies_type(&self, typ: &Type, is_self: bool) -> bool {
        match typ {
            Type::Trait(trayt) => {
                let trayt_name = &trayt.as_ref().borrow().full_name;
                match self {
                    Instance::Class(class) => class
                        .class()
                        .definitions
                        .iter()
                        .any(|(trayt, _)| &trayt.as_ref().borrow().full_name == trayt_name),
                    Instance::Struct(strukt) => strukt
                        .strukt()
                        .definitions
                        .iter()
                        .any(|(trayt, _)| &trayt.as_ref().borrow().full_name == trayt_name),
                    _ => false,
                }
            }
            Type::Raw(raw_type) => {
                if let Instance::Raw(val) = self {
                    match raw_type {
                        RawType::Int => matches!(val, RawValue::Int(_)),
                        RawType::String => matches!(val, RawValue::String(_)),
                        RawType::Bool => matches!(val, RawValue::Bool(_)),
                    }
                } else {
                    false
                }
            }
            Type::And(a, b) => self.satisfies_type(a, is_self) && self.satisfies_type(b, is_self),
            Type::Or(a, b) => self.satisfies_type(a, is_self) || self.satisfies_type(b, is_self),
            Type::Zelf => is_self,
            Type::Void => true,
        }
    }

    /// Whether the current instance is of the same type as the given instance.
    /// This is used to match the 'Self' type in match statements during runtime.
    pub fn is_of_same_type(&self, other: &Self) -> bool {
        match self {
            Instance::Class(klass) => {
                if let Instance::Class(other) = other {
                    klass.class() == other.class()
                } else {
                    false
                }
            }
            Instance::Struct(strukt) => {
                if let Instance::Struct(other) = other {
                    strukt.strukt() == other.strukt()
                } else {
                    false
                }
            }
            Instance::Raw(raw_value) => {
                if let Instance::Raw(other) = other {
                    match raw_value {
                        RawValue::Int(_) => matches!(other, RawValue::Int(_)),
                        RawValue::String(_) => matches!(other, RawValue::String(_)),
                        RawValue::Bool(_) => matches!(other, RawValue::Bool(_)),
                    }
                } else {
                    false
                }
            }
            Instance::Void => matches!(other, Instance::Void),
        }
    }

    pub fn to_string(self: &Rc<Self>, context: &SemanticContext) -> CResult<String> {
        if let Instance::Raw(raw_value) = self.borrow() {
            let result = match raw_value {
                RawValue::String(value) => value.clone(),
                RawValue::Int(value) => value.to_string(),
                RawValue::Bool(value) => if *value { "true" } else { "false" }.to_string(),
            };

            return Ok(result);
        } else if let Instance::Struct(strukt) = self.borrow() {
            if strukt.strukt().fields.get(0) == Some(&("value".to_string(), RawType::String)) {
                if let RawValue::String(value) = strukt.field("value") {
                    return Ok(value.clone());
                } else {
                    unreachable!()
                }
            }
        }

        // Call the String trait on self recursively until hitting a String.
        let string_trait = context.traits.resolve("String", "")?;

        self.call(string_trait, [].into()).to_string(context)
    }
}
