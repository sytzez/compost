use crate::ast::parser::{parse_global, Parser};
use crate::ast::raw_value::RawValue;
use crate::error::CResult;
use crate::lex::token::{Kw, Lit, Op, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;
use crate::sem::scope::{path, ReferencePath};
use std::collections::HashMap;

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
    // ConstructClass(Rc<Class>),
    // ConstructStruct(Rc<Struct>),
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

impl Parser for Expression {
    fn matches(_tokens: &[LeveledToken]) -> bool {
        true
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();

        // Needs cloning to prevent immutable borrow errors.
        let token = tokens.token().clone();

        // Parse first token
        let mut expr = match token {
            Token::Kw(Kw::Zelf) => {
                tokens.step();

                Expression::Zelf
            }
            Token::Global(_) => {
                let call = parse_let_call(tokens)?;

                Expression::Let(call)
            }
            Token::Local(name) => {
                tokens.step();

                Expression::Local(name.clone())
            }
            Token::Lit(lit) => {
                tokens.step();

                Expression::Literal(match lit {
                    Lit::String(value) => RawValue::String(value),
                    Lit::Number(value) => RawValue::Int(value as i64),
                })
            }
            Token::Op(Op::Dot) => {
                // We don't step so we can reevaluate the same dot in the next step.
                Expression::Zelf
            }
            Token::Op(Op::Sub) => {
                tokens.step();

                let expr = Expression::parse(tokens)?;

                Expression::Def(DefCall {
                    path: path("Op\\Neg"),
                    subject: Box::new(expr),
                    inputs: [].into(),
                })
            }
            _ => return tokens.error(format!("Unexpected token {:?}", tokens.token())),
        };

        // Parse further operations
        while tokens.deeper_than_or_eq(base_level) {
            // Needs cloning to prevent immutable borrow errors.
            let token = tokens.token().clone();

            expr = match token {
                Token::Op(op) => {
                    match op {
                        Op::Add | Op::Sub | Op::Mul | Op::Div => {
                            tokens.step();

                            let rhs = Expression::parse(tokens)?;

                            Expression::Binary(BinaryCall {
                                op: match op {
                                    Op::Add => BinaryOp::Add,
                                    Op::Sub => BinaryOp::Sub,
                                    Op::Mul => BinaryOp::Mul,
                                    Op::Div => BinaryOp::Div,
                                    _ => unreachable!(),
                                },
                                lhs: Box::new(expr),
                                rhs: Box::new(rhs),
                            })
                        }
                        Op::Dot => {
                            tokens.step();

                            // Needs cloning to prevent immutable borrow errors.
                            let token = tokens.token().clone();

                            match (expr, token) {
                                (Expression::Local(local_name), Token::Local(field_name)) => {
                                    tokens.step();

                                    Expression::FriendlyField(FriendlyField {
                                        local_name,
                                        field_name,
                                    })
                                }
                                (expr, Token::Global(_)) => {
                                    let call = parse_let_call(tokens)?;

                                    Expression::Def(DefCall {
                                        path: call.path,
                                        subject: Box::new(expr),
                                        inputs: call.inputs,
                                    })
                                }
                                _ => return tokens.error(
                                    "Dot operator must be followed by a trait or friendly field"
                                        .to_string(),
                                ),
                            }
                        }
                        _ => return tokens.error(format!("Unexpected operator {:?}", op)),
                    }
                }
                _ => return tokens.error(format!("Unexpected token {:?}", tokens.token())),
            }
        }

        Ok(expr)
    }
}

// e.g.: Module\Function(param1: ..., param2: ...)
// e.g.: Module\Constant
fn parse_let_call(tokens: &mut Tokens) -> CResult<LetCall> {
    let base_level = tokens.level();
    let name = parse_global(tokens)?;

    let mut inputs = HashMap::new();

    while tokens.deeper_than(base_level) {
        // Needs cloning to prevent immutable borrow errors.
        let token = tokens.token().clone();

        if let Token::Local(param_name) = token {
            tokens.step();

            let expr = Expression::parse(tokens)?;

            inputs.insert(param_name, expr);
        } else {
            break;
        }
    }

    let call = LetCall {
        path: path(&name),
        inputs,
    };

    Ok(call)
}

// impl Expression {
//     // TODO: this logic needs to go into the runtime
//     pub fn resolve(&self, scope: &LocalScope) -> Rc<Instance> {
//         match self {
//             Expression::Binary(call) => {
//                 let trait_path = match call.op {
//                     BinaryOp::Add => "Op\\Add",
//                     BinaryOp::Sub => "Op\\Sub",
//                     BinaryOp::Mul => "Op\\Mul",
//                     BinaryOp::Div => "Op\\Div",
//                 };
//                 let trait_path = path(trait_path);
//
//                 let lhs = call.lhs.resolve(scope);
//                 let rhs = call.rhs.resolve(scope);
//
//                 let inputs = [("rhs".to_string(), rhs)].into();
//
//                 lhs.call(&trait_path, inputs, scope.scope())
//             }
//             Expression::Let(call) => {
//                 println!("Call let {} ", call.path.join("\\"));
//
//                 let inputs = call
//                     .inputs
//                     .iter()
//                     .map(|(name, expression)| (name.clone(), expression.resolve(scope)))
//                     .collect::<HashMap<_, _>>();
//
//                 let lett = scope.scope().lett(&call.path);
//
//                 lett.resolve(inputs, scope.scope())
//             }
//             Expression::Def(call) => {
//                 println!("Call def {} ", call.path.join("\\"));
//
//                 let subject = call.subject.resolve(scope);
//
//                 let inputs = call
//                     .inputs
//                     .iter()
//                     .map(|(name, expression)| (name.clone(), expression.resolve(scope)))
//                     .collect::<HashMap<_, _>>();
//
//                 subject.call(&call.path, inputs, scope.scope())
//             }
//             Expression::Literal(value) => Rc::new(Instance::Raw(value.clone())),
//             Expression::Local(name) => Rc::clone(scope.local(name)),
//             Expression::FriendlyField(friendly_field) => {
//                 let local = scope.local(&friendly_field.local_name);
//
//                 match local.borrow() {
//                     Instance::Struct(struct_instance) => {
//                         let value = struct_instance.value(&friendly_field.field_name);
//
//                         Rc::new(Instance::Raw(value.clone()))
//                     }
//                     Instance::Raw(raw_value) => {
//                         let value = raw_value.call(&path(&friendly_field.field_name), [].into());
//
//                         Rc::new(Instance::Raw(value))
//                     }
//                     _ => panic!(
//                         "{0} in {0}.{1} is not a struct or raw value",
//                         friendly_field.local_name, friendly_field.field_name
//                     ),
//                 }
//             }
//             Expression::Zelf => match scope.zelf() {
//                 Some(z) => Rc::clone(z),
//                 None => panic!("No self in local scope"),
//             },
//             // Expression::ConstructStruct(strukt) => {
//             //     let values = strukt
//             //         .fields
//             //         .keys()
//             //         .map(|key| {
//             //             // TODO: check types
//             //
//             //             let instance = scope.local(key);
//             //             let raw = match instance.borrow() {
//             //                 Instance::Raw(value) => value.clone(),
//             //                 _ => panic!(),
//             //             };
//             //
//             //             (key.clone(), raw)
//             //         })
//             //         .collect::<HashMap<_, _>>();
//             //
//             //     let struct_instance = StructInstance::new(&strukt, values);
//             //
//             //     Rc::new(Instance::Struct(struct_instance))
//             // }
//             // Expression::ConstructClass(class) => {
//             //     let dependencies = class
//             //         .dependencies
//             //         .keys()
//             //         .map(|key| (key.clone(), Rc::clone(scope.local(key))))
//             //         .collect::<HashMap<_, _>>();
//             //
//             //     let class_instance = ClassInstance::new(&class, dependencies);
//             //
//             //     Rc::new(Instance::Class(class_instance))
//             // }
//         }
//     }
// }
