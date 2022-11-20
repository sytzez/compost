use std::cell::RefCell;

use std::rc::Rc;
use crate::ast::expression::{BinaryOp, Expression, FriendlyField};
use crate::ast::raw_value::RawValue;
use crate::error::CResult;
use crate::sem::class::Class;
use crate::sem::lett::Let;
use crate::sem::scope::{path};
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::strukt::Struct;
use crate::sem::trayt::Trait;
use crate::sem::typ::Type;

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
    pub lett: Rc<RefCell<Option<Let>>>,
    pub inputs: Vec<(String, Evaluation)>,
}

#[derive(Clone)]
pub struct TraitEvaluation {
    pub trayt: Rc<RefCell<Option<Trait>>>,
    pub subject: Box<Evaluation>,
    pub inputs: Vec<(String, Evaluation)>,
}

impl Evaluation {
    pub fn analyse(expr: &Expression, context: &SemanticContext) -> CResult<Self> {
        let eval = match expr.clone() {
            Expression::Binary(call) => {
                let trait_path = match call.op {
                    BinaryOp::Add => "Op\\Add",
                    BinaryOp::Sub => "Op\\Sub",
                    BinaryOp::Mul => "Op\\Mul",
                    BinaryOp::Div => "Op\\Div",
                };

                let inputs = [
                    ("rhs".into(), Evaluation::analyse(&call.rhs, context)?)
                ].into();

                let subject = Box::new(Evaluation::analyse(&call.lhs, context)?);

                // TODO: check lhs and rhs types.
                Evaluation::Trait(
                    TraitEvaluation {
                        trayt: context.traits.resolve(&path(&trait_path))?,
                        subject,
                        inputs,
                    }
                )
            }
            Expression::Def(call) => {
                let mut inputs = vec![];
                for (param_name, expr) in call.inputs.into_iter() {
                    let eval = Evaluation::analyse(&expr, context)?;

                    inputs.push((param_name, eval));
                }

                // TODO: Check inputs match
                Evaluation::Trait(
                    TraitEvaluation {
                        trayt: context.traits.resolve(&call.path)?,
                        subject: Box::new(Evaluation::analyse(&call.subject, context)?),
                        inputs,
                    }
                )
            }
            Expression::Let(call) => {
                let mut inputs = vec![];
                for (param_name, expr) in call.inputs.into_iter() {
                    let eval = Evaluation::analyse(&expr, context)?;

                    inputs.push((param_name, eval));
                }

                // TODO: check inputs match
                Evaluation::Let(
                    LetEvaluation {
                        lett: context.lets.resolve(&call.path)?,
                        inputs,
                    }
                )
            }
            Expression::Literal(value) => Evaluation::Literal(value),
            Expression::Local(name) => Evaluation::Local(name),
            Expression::FriendlyField(ff) => Evaluation::FriendlyField(ff),
            Expression::Zelf => Evaluation::Zelf,
        };

        Ok(eval)
    }

    pub fn typ(&self, context: &SemanticContext) -> Type {
        match self {
            Evaluation::Let(call) => call.lett.borrow().as_ref().unwrap().output.clone(),
            Evaluation::Trait(call) => call.trayt.borrow().as_ref().unwrap().output.clone(),
            Evaluation::Literal(_raw_value) => todo!(),
            Evaluation::Local(name) => context.locals.get(name).unwrap().clone(),
            Evaluation::FriendlyField(_ff) => todo!(),
            Evaluation::Zelf => context.zelf.as_ref().unwrap().clone(),
            Evaluation::ClassConstructor(class) => class.interface(),
            Evaluation::StructConstructor(strukt) => strukt.interface(),
        }
    }
}
