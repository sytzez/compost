use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use crate::class::ClassInstance;
use crate::definition::Definition;
use crate::raw_value::RawValue;
use crate::scope::{ReferencePath, Scope};
use crate::strukt::StructInstance;
use crate::typ::{RawType, Type};

pub enum Instance {
    Class(ClassInstance),
    Struct(StructInstance),
    Raw(RawValue),
}

impl Instance {
    pub fn has_trait(&self, trait_path: &ReferencePath) -> bool {
        self.definitions().contains_key(trait_path)
    }

    pub fn call(self: &Rc<Self>, trait_path: &ReferencePath, inputs: HashMap<String, Rc<Instance>>, scope: &Scope) -> Rc<Instance> {
        match self.borrow() {
            Instance::Raw(value) => return Rc::new(Instance::Raw(value.call(trait_path, inputs))),
            _ => (),
        };

        let definition = self.definitions().get(trait_path).expect("Trait not defined");

        let locals = inputs
            .into_iter()
            .chain(self.values())
            .collect();

        let local_scope = scope.local_scope(Some(Rc::clone(self)), locals);

        definition.expression.resolve(&local_scope)
    }

    fn definitions(&self) -> &HashMap<ReferencePath, Definition> {
        match self {
            Instance::Class(instance) => &instance.class.definitions,
            Instance::Struct(instance) => &instance.strukt.definitions,
            _ => panic!(),
        }
    }

    fn values(&self) -> HashMap<String, Rc<Instance>> {
        match self {
            Instance::Struct(instance) => instance
                .values
                .iter()
                .map(|(name, value)| (name.clone(), Rc::new(Instance::Raw(value.clone()))))
                .collect(),
            _ => panic!(),
        }
    }

    pub fn is_of_raw_type(&self, typ: &RawType) -> bool {
        match self {
            Instance::Raw(value) => {
                match typ {
                    RawType::Int => match value {
                        RawValue::Int(_) => true,
                        _ => false,
                    }
                    RawType::UInt => match value {
                        RawValue::UInt(_) => true,
                        _ => false,
                    }
                    RawType::Float => match value {
                        RawValue::Float(_) => true,
                        _ => false,
                    }
                    RawType::String => match value {
                        RawValue::String(_) => true,
                        _ => false,
                    }
                }
            }
            _ => false,
        }
    }

    pub fn is_of_type(&self, typ: &Type, is_self: bool) -> bool {
        match typ {
            Type::Or(left, right) => self.is_of_type(left, is_self) || self.is_of_type(right, is_self),
            Type::And(left, right) => self.is_of_type(left, is_self) && self.is_of_type(right, is_self),
            Type::Trait(path) => self.has_trait(path),
            Type::Raw(raw_type) => self.is_of_raw_type(raw_type),
            Type::Zelf => is_self,
            Type::Closed => true,
        }
    }
}
