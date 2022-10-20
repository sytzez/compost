use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use crate::class::{Instance, RawValue, Class, Struct, StructInstance};
use crate::scope::{create_reference_path, ReferencePath, Scope, LocalScope};

pub enum Expression {
    Binary(BinaryCall),
    Let(LetCall),
    Def(DefCall),
    Literal(RawValue),
    Local(String),
    Zelf,
    // only for internal use
    ConstructClass(Rc<Class>),
    ConstructStruct(Rc<Struct>),
}

pub struct BinaryCall {
    pub op: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

pub struct LetCall {
    pub path: ReferencePath,
    pub inputs: HashMap<String, Expression>,
}

pub struct DefCall {
    pub path: ReferencePath,
    pub inputs: HashMap<String, Expression>,
}

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
                    BinaryOp::Add => "Op.Add",
                    BinaryOp::Sub => "Op.Sub",
                    BinaryOp::Mul => "Op.Mul",
                    BinaryOp::Div => "Op.Div",
                };
                let trait_path = create_reference_path(trait_path);

                let lhs = call.lhs.resolve(scope);
                let rhs = call.rhs.resolve(scope);

                let inputs = [("rhs".to_string(), rhs)].into();

                lhs.call(&trait_path, inputs, scope.scope())
            },
            Expression::Let(call) => {
                let inputs = call.inputs
                    .iter()
                    .map(|(name, expression)| (name.clone(), expression.resolve(scope)))
                    .collect::<HashMap<_, _>>();

                let lett = scope.scope().lett(&call.path);

                lett.resolve(inputs, scope.scope())
            },
            Expression::Def(call) => {
                let inputs = call.inputs
                    .iter()
                    .map(|(name, expression)| (name.clone(), expression.resolve(scope)))
                    .collect::<HashMap<_, _>>();

                let zelf = match scope.zelf() {
                    Some(z) => z,
                    None => panic!("No self in local scope"),
                };

                zelf.call(&call.path, inputs, scope.scope())
            },
            Expression::Literal(value) => {
                Rc::new(Instance::Raw(value.clone()))
            },
            Expression::Local(name) => {
                Rc::clone(scope.local(name))
            },
            Expression::Zelf => {
                match scope.zelf() {
                    Some(z) => Rc::clone(z),
                    None => panic!("No self in local scope"),
                }
            },
            Expression::ConstructStruct(strukt) => {
                let values = strukt.fields
                    .keys()
                    .map(|key| {
                        let instance = scope.local(key);
                        let raw = match instance.borrow() {
                            Instance::Raw(value) => value.clone(),
                            _ => panic!(),
                        };

                        (key.clone(), raw)
                    })
                    .collect::<HashMap<_, _>>();
                    
                let struct_instance = StructInstance {
                    strukt: Rc::clone(strukt),
                    values,
                };

                Rc::new(Instance::Struct(struct_instance))
            },
            Expression::ConstructClass(class) => {
                panic!("todo")
            }
        }
    }
}
