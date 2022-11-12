use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use crate::class::ClassInstance;
use crate::definition::Definition;
use crate::path;
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

        let locals = inputs
            .into_iter()
            .chain(self.values())
            .collect();

        let local_scope = scope.local_scope(Some(Rc::clone(self)), locals);

        if let Some(definition) = self.definitions().get(trait_path) {
            // Use definition defined for class or struct.
            definition.expression.resolve(&local_scope)
        } else {
            // Use definition defined on original module of trait.
            scope.def(trait_path).expression.resolve(&local_scope)
        }
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
            Type::Void => true,
        }
    }

    pub fn to_string(self: &Rc<Self>, scope: &Scope) -> String {
        if let Instance::Raw(raw_value) = self.borrow() {
            match raw_value {
                RawValue::String(value) => value.clone(),
                RawValue::Int(value) => value.to_string(),
                RawValue::UInt(value) => value.to_string(),
            }
        } else {
            self.call(&path("String"), [].into(), scope)
                .to_string(scope)
        }
    }
}
