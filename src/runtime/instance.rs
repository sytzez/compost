use crate::ast::raw_value::RawValue;
use crate::ast::type_statement::RawType;
use crate::error::CResult;
use crate::runtime::class_instance::ClassInstance;
use crate::runtime::evaluate::evaluate;
use crate::runtime::raw_operation::raw_operation;
use crate::runtime::struct_instance::StructInstance;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::trayt::Trait;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::sem::typ::Type;

/// An instantiated class or struct, or a raw value.
#[derive(Debug)]
pub enum Instance {
    Class(ClassInstance),
    Struct(StructInstance),
    Raw(RawValue),
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
        };

        let evaluation = &definitions.iter().find(|(t, _)| t == &trayt).unwrap().1;

        inputs.extend(self.locals());

        evaluate(evaluation, inputs, Some(Rc::clone(self)))
    }

    fn locals(&self) -> HashMap<String, Rc<Instance>> {
        match self {
            Instance::Struct(instance) => instance.fields(),
            Instance::Class(instance) => instance.dependencies(),
            Instance::Raw(_) => unreachable!(),
        }
    }

    /// Whether the current instance satisfies the given type.
    pub fn satisfies_type(&self, typ: &Type) -> bool {
        match typ {
            Type::Trait(trayt) => {
                let trayt_name = &trayt.as_ref().borrow().full_name;
                match self {
                    Instance::Class(class) => {
                        class.class()
                            .definitions
                            .iter()
                            .any(|(trayt, _)| &trayt.as_ref().borrow().full_name == trayt_name)
                    },
                    Instance::Struct(strukt) => {
                        strukt.strukt()
                            .definitions
                            .iter()
                            .any(|(trayt, _)| &trayt.as_ref().borrow().full_name == trayt_name)
                    },
                    Instance::Raw(_) => false,
                }
            },
            Type::Raw(raw_type) => {
                if let Instance::Raw(val) = self {
                     match raw_type {
                         RawType::Int => matches!(val, RawValue::Int(_)),
                         RawType::String => matches!(val, RawValue::String(_)),
                     }
                } else {
                    false
                }
            }
            Type::And(a, b) => self.satisfies_type(a) && self.satisfies_type(b),
            Type::Or(a, b) => self.satisfies_type(a) || self.satisfies_type(b),
            Type::Zelf => todo!(),
            Type::Void => true,
        }
    }

    pub fn to_string(self: &Rc<Self>, context: &SemanticContext) -> CResult<String> {
        if let Instance::Raw(raw_value) = self.borrow() {
            let result = match raw_value {
                RawValue::String(value) => value.clone(),
                RawValue::Int(value) => value.to_string(),
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
