use std::cell::RefCell;

use crate::ast::expression::{BinaryOp, Expression, FriendlyField};
use crate::ast::raw_value::RawValue;
use crate::error::{error, CResult};
use crate::sem::class::Class;
use crate::sem::lett::Let;
use crate::sem::semantic_analyser::SemanticScope;
use crate::sem::strukt::Struct;
use crate::sem::trayt::Trait;
use crate::sem::typ::Type;
use std::rc::Rc;

/// A semantically analysed expression that can be evaluated.
#[derive(Clone)]
pub enum Evaluation {
    Let(LetEvaluation),
    Trait(TraitEvaluation),
    Literal(RawValue),
    Local(String),
    FriendlyField(FriendlyField),
    Zelf,
    // only for internal use
    ClassConstructor(Rc<Class>),
    StructConstructor(Rc<Struct>),
}

#[derive(Clone)]
pub struct LetEvaluation {
    pub lett: Rc<RefCell<Let>>,
    pub inputs: Vec<(String, Evaluation)>,
}

#[derive(Clone)]
pub struct TraitEvaluation {
    pub trayt: Rc<RefCell<Trait>>,
    pub subject: Box<Evaluation>,
    pub inputs: Vec<(String, Evaluation)>,
}

impl Evaluation {
    pub fn analyse(expr: &Expression, scope: &SemanticScope) -> CResult<Self> {
        let eval = match expr.clone() {
            Expression::Binary(call) => {
                let trait_path = match call.op {
                    BinaryOp::Add => "Op\\Add",
                    BinaryOp::Sub => "Op\\Sub",
                    BinaryOp::Mul => "Op\\Mul",
                    BinaryOp::Div => "Op\\Div",
                };

                let subject = Box::new(Evaluation::analyse(&call.lhs, scope)?);

                let inputs = [("rhs".into(), Evaluation::analyse(&call.rhs, scope)?)].into();

                // TODO: check lhs and rhs types.
                Evaluation::Trait(TraitEvaluation {
                    trayt: scope.context.traits.resolve(trait_path, "")?,
                    subject,
                    inputs,
                })
            }
            Expression::Def(call) => {
                let mut inputs = vec![];
                for (param_name, expr) in call.inputs.into_iter() {
                    let eval = Evaluation::analyse(&expr, scope)?;

                    inputs.push((param_name, eval));
                }

                // TODO: Check inputs match
                println!("Doing def {} in scope {}", call.name, scope.path);
                Evaluation::Trait(TraitEvaluation {
                    trayt: scope.context.traits.resolve(&call.name, scope.path)?,
                    subject: Box::new(Evaluation::analyse(&call.subject, scope)?),
                    inputs,
                })
            }
            Expression::Let(call) => {
                let mut inputs = vec![];
                for (param_name, expr) in call.inputs.into_iter() {
                    let eval = Evaluation::analyse(&expr, scope)?;

                    inputs.push((param_name, eval));
                }

                // TODO: check inputs match
                Evaluation::Let(LetEvaluation {
                    lett: scope.context.lets.resolve(&call.name, scope.path)?,
                    inputs,
                })
            }
            Expression::Literal(value) => Evaluation::Literal(value),
            Expression::Local(name) => Evaluation::Local(name),
            Expression::FriendlyField(ff) => Evaluation::FriendlyField(ff),
            Expression::Zelf => Evaluation::Zelf,
        };

        Ok(eval)
    }

    pub fn typ(&self, scope: &SemanticScope) -> CResult<Type> {
        let typ = match self {
            Evaluation::Let(call) => call.lett.borrow().output.clone(),
            Evaluation::Trait(call) => call.trayt.borrow().output.clone(),
            Evaluation::Literal(_raw_value) => todo!(),
            Evaluation::Local(name) => scope.locals.get(name).unwrap().clone(),
            Evaluation::FriendlyField(_ff) => todo!(),
            Evaluation::Zelf => match &scope.zelf {
                Some(typ) => typ.clone(),
                None => return error("There is no 'Self' in this scope".to_string(), 0),
            },
            Evaluation::ClassConstructor(class) => class.interface(),
            Evaluation::StructConstructor(strukt) => strukt.interface(),
        };

        Ok(typ)
    }
}
