use std::{collections::HashMap, borrow::Borrow};
use std::rc::Rc;
use std::string::String;
use crate::scope::path;
use crate::{scope::{ReferencePath, Scope}, expression::Expression};

pub struct Class {
    dependencies: HashMap<String, Type>,
    definitions: HashMap<ReferencePath, Definition>
}

pub struct ClassInstance {
    class: Rc<Class>,
    dependencies: HashMap<String, Instance>
}

pub struct Struct {
    pub fields: HashMap<String, RawType>,
    definitions: HashMap<ReferencePath, Definition>,
}

pub struct Definition {
    // Inputs and outputs are declared by the trait
    pub expression: Expression,
}

impl Struct {
    pub fn new() -> Self {
        Struct {
            fields: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn instantiate(self: &Rc<Self>, values: HashMap<String, RawValue>) -> StructInstance {
        StructInstance { strukt: Rc::clone(self) , values }
    }

    pub fn add_field(&mut self, name: String, typ: RawType) {
        self.fields.insert(name, typ);
    }

    pub fn add_definition(&mut self, trait_path: ReferencePath, definition: Definition) {
        self.definitions.insert(trait_path, definition);
    }

    pub fn constructor(self: &Rc<Self>) -> Let {
        let inputs = self.fields
            .iter()
            .map(|(name, typ)| {
                (name.clone(), Type::Raw(*typ))
            })
            .collect::<HashMap<_, _>>();

        Let {
            inputs,
            outputs: [(String::new(), self.interface())].into(),
            expression: Expression::ConstructStruct(Rc::clone(self)),
        }
    }

    pub fn interface(&self) -> Type {
        let types = self.definitions.keys()
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
    pub fn new(strukt: &Rc<Struct>) -> Self {
        Self {
            strukt: Rc::clone(strukt),
            values: HashMap::new(),
        }
    }

    pub fn strukt(&self) -> &Rc<Struct> {
        &self.strukt
    }

    pub fn set_value(&mut self, name: String, value: RawValue) {
        self.values.insert(name, value);
    }

    pub fn value(&self, name: &str) -> &RawValue {
        self.values.get(name).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Interface {
    pub traits: Vec<Trait>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Type {
    Trait(ReferencePath),
    Raw(RawType),
    And(Box<Type>, Box<Type>),
    Or(Box<Type>, Box<Type>),
    // Self, the class or struct the trait is defined on
    Zelf,
    // No traits, no interaction possible
    Closed,
}

pub fn combine_types(types: Vec<Type>) -> Type {
    let mut combined = None;

    for typ in types {
        combined = match combined {
            None => Some(typ),
            Some(prev_type) => Some(
                Type::And(
                    Box::new(prev_type),
                    Box::new(typ)
                )
            ),
        }
    }

    match combined {
        Some(typ) => typ,
        None => Type::Closed,
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum RawType {
    Int,
    UInt,
    Float,
    String,
}

#[derive(Clone)]
pub enum RawValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
}

impl RawValue {
    pub fn call(&self, trait_path: &ReferencePath, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        if trait_path == &path("Op.Add") {
            return self.add(inputs);
        }

        panic!()
    }

    pub fn add(&self, inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        let rhs = Self::rhs(inputs);

        match self {
            RawValue::Int(value) => RawValue::Int(*value + rhs.int()),
            _ => panic!("todo"),
        }
    }

    fn int(&self) -> i64 {
        match self {
            RawValue::Int(value) => *value,
            _ => panic!(),
        }
    }

    fn rhs(inputs: HashMap<String, Rc<Instance>>) -> RawValue {
        match inputs.get("rhs").unwrap().borrow() {
            Instance::Raw(value) => value.clone(),
            _ => panic!(),
        }
    }
}

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

#[derive(Eq, PartialEq, Hash)]
pub struct Trait {
    pub reference_path: ReferencePath,
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
}

pub struct Let {
    pub inputs: HashMap<String, Type>,
    pub outputs: HashMap<String, Type>,
    pub expression: Expression,
}

impl Let {
    pub fn resolve(&self, inputs: HashMap<String, Rc<Instance>>, scope: &Scope) -> Rc<Instance> {
        let local_scope = scope.local_scope(None, inputs);

        self.expression.resolve(&local_scope)
    }
}
