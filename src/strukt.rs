use crate::definition::Definition;
use crate::expression::Expression;
use crate::scope::ReferencePath;
use std::collections::HashMap;
use std::rc::Rc;
use crate::Instance;
use crate::lett::Let;
use crate::raw_value::RawValue;
use crate::typ::{combine_types, RawType, Type};

// A struct has a set of fields which are of raw types, and a set of trait definitions.
pub struct Struct {
    pub fields: HashMap<String, RawType>,
    pub definitions: HashMap<ReferencePath, Definition>,
}

impl Struct {
    pub fn new() -> Self {
        Struct {
            fields: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn instantiate(self: &Rc<Self>, values: HashMap<String, RawValue>) -> StructInstance {
        StructInstance {
            strukt: Rc::clone(self),
            values,
        }
    }

    pub fn add_field(&mut self, name: String, typ: RawType) {
        self.fields.insert(name, typ);
    }

    pub fn add_definition(&mut self, trait_path: ReferencePath, definition: Definition) {
        self.definitions.insert(trait_path, definition);
    }

    pub fn constructor(self: &Rc<Self>) -> Let {
        let inputs = self
            .fields
            .iter()
            .map(|(name, typ)| (name.clone(), Type::Raw(*typ)))
            .collect::<HashMap<_, _>>();

        Let {
            inputs,
            outputs: [(String::new(), self.interface())].into(),
            expression: Expression::ConstructStruct(Rc::clone(self)),
        }
    }

    pub fn interface(&self) -> Type {
        let types = self
            .definitions
            .keys()
            .cloned()
            .map(|path| Type::Trait(path))
            .collect::<Vec<_>>();

        combine_types(types)
    }
}

pub struct StructInstance {
    strukt: Rc<Struct>,
    values: HashMap<String, RawValue>,
}

impl StructInstance {
    pub fn strukt(&self) -> &Rc<Struct> {
        &self.strukt
    }

    pub fn value(&self, name: &str) -> &RawValue {
        self.values
            .get(name)
            .expect(&format!("Field {} does not exist", name))
    }

    pub fn values(&self) -> HashMap<String, Rc<Instance>> {
        self.values
            .iter()
            .map(|(name, value)| (name.clone(), Rc::new(Instance::Raw(value.clone()))))
            .collect()
    }
}
