use crate::ast::raw_value::RawValue;
use crate::runtime::class_instance::ClassInstance;
use crate::runtime::instance::Instance;
use crate::runtime::struct_instance::StructInstance;
use crate::sem::class::Class;
use crate::sem::scope::{path, LocalScope, ReferencePath};
use crate::sem::strukt::Struct;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub enum Expression {
    Binary(BinaryCall),
    Let(LetCall),
    Def(DefCall),
    Literal(RawValue),
    Local(String),
    FriendlyField(FriendlyField),
    Zelf,
    // only for internal use
    ConstructClass(Rc<Class>),
    ConstructStruct(Rc<Struct>),
}

#[derive(Clone)]
pub struct BinaryCall {
    pub op: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone)]
pub struct LetCall {
    pub path: ReferencePath,
    pub inputs: HashMap<String, Expression>,
}

#[derive(Clone)]
pub struct DefCall {
    pub path: ReferencePath,
    pub subject: Box<Expression>,
    pub inputs: HashMap<String, Expression>,
}

// A reference to the protected field of another instance of the self struct
#[derive(Clone)]
pub struct FriendlyField {
    pub local_name: String,
    pub field_name: String,
}

#[derive(Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expression {
    pub fn resolve(&self, scope: &LocalScope) -> Rc<Instance> {
        match self {
            Expression::Binary(call) => {
                let trait_path = match call.op {
                    BinaryOp::Add => "Op\\Add",
                    BinaryOp::Sub => "Op\\Sub",
                    BinaryOp::Mul => "Op\\Mul",
                    BinaryOp::Div => "Op\\Div",
                };
                let trait_path = path(trait_path);

                let lhs = call.lhs.resolve(scope);
                let rhs = call.rhs.resolve(scope);

                let inputs = [("rhs".to_string(), rhs)].into();

                lhs.call(&trait_path, inputs, scope.scope())
            }
            Expression::Let(call) => {
                println!("Call let {} ", call.path.join("\\"));

                let inputs = call
                    .inputs
                    .iter()
                    .map(|(name, expression)| (name.clone(), expression.resolve(scope)))
                    .collect::<HashMap<_, _>>();

                let lett = scope.scope().lett(&call.path);

                lett.resolve(inputs, scope.scope())
            }
            Expression::Def(call) => {
                println!("Call def {} ", call.path.join("\\"));

                let subject = call.subject.resolve(scope);

                let inputs = call
                    .inputs
                    .iter()
                    .map(|(name, expression)| (name.clone(), expression.resolve(scope)))
                    .collect::<HashMap<_, _>>();

                subject.call(&call.path, inputs, scope.scope())
            }
            Expression::Literal(value) => Rc::new(Instance::Raw(value.clone())),
            Expression::Local(name) => Rc::clone(scope.local(name)),
            Expression::FriendlyField(friendly_field) => {
                let local = scope.local(&friendly_field.local_name);

                match local.borrow() {
                    Instance::Struct(struct_instance) => {
                        let value = struct_instance.value(&friendly_field.field_name);

                        Rc::new(Instance::Raw(value.clone()))
                    }
                    Instance::Raw(raw_value) => {
                        let value = raw_value.call(&path(&friendly_field.field_name), [].into());

                        Rc::new(Instance::Raw(value))
                    }
                    _ => panic!(
                        "{0} in {0}.{1} is not a struct or raw value",
                        friendly_field.local_name, friendly_field.field_name
                    ),
                }
            }
            Expression::Zelf => match scope.zelf() {
                Some(z) => Rc::clone(z),
                None => panic!("No self in local scope"),
            },
            Expression::ConstructStruct(strukt) => {
                let values = strukt
                    .fields
                    .keys()
                    .map(|key| {
                        // TODO: check types

                        let instance = scope.local(key);
                        let raw = match instance.borrow() {
                            Instance::Raw(value) => value.clone(),
                            _ => panic!(),
                        };

                        (key.clone(), raw)
                    })
                    .collect::<HashMap<_, _>>();

                let struct_instance = StructInstance::new(&strukt, values);

                Rc::new(Instance::Struct(struct_instance))
            }
            Expression::ConstructClass(class) => {
                let dependencies = class
                    .dependencies
                    .keys()
                    .map(|key| (key.clone(), Rc::clone(scope.local(key))))
                    .collect::<HashMap<_, _>>();

                let class_instance = ClassInstance::new(&class, dependencies);

                Rc::new(Instance::Class(class_instance))
            }
        }
    }
}