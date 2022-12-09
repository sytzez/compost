use std::cell::RefCell;

use crate::ast::expression::{BinaryOp, Expression, FriendlyField};
use crate::ast::raw_value::RawValue;
use crate::error::ErrorMessage::NoResolution;
use crate::error::{error, CResult, ErrorMessage};
use crate::sem::class::Class;
use crate::sem::lett::Let;
use crate::sem::semantic_analyser::SemanticScope;
use crate::sem::strukt::Struct;
use crate::sem::table::Table;
use crate::sem::trayt::Trait;
use crate::sem::typ::Type;
use std::rc::Rc;
use crate::ast::type_statement::RawType;
use crate::sem::type_checking::check_types;

/// A semantically analysed expression that can be evaluated.
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct LetEvaluation {
    pub lett: Rc<RefCell<Let>>,
    pub inputs: Vec<(String, Evaluation)>,
}

#[derive(Clone, Debug)]
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
                let trayt = scope.context.traits.resolve(trait_path, "")?;

                let subject = Box::new(Evaluation::analyse(&call.lhs, scope)?);

                let inputs = vec![("rhs".into(), Evaluation::analyse(&call.rhs, scope)?)];

                // TODO: if subject is a raw literal, check if rhs is a literal of the same type.
                // TODO: OR, if one is a String or Int and other is string or int, do type coercion.
                // check_types(&trayt.borrow().inputs, &inputs, scope)?;

                Evaluation::Trait(TraitEvaluation { trayt, subject, inputs })
            }
            Expression::Def(call) => {
                let subject = Evaluation::analyse(&call.subject, scope)?;

                // Make a temporary trait table using only traits defined on the subject.
                let mut trait_name_table = Table::new("Trait");
                for trait_name in subject.typ(scope)?.callable_traits(scope).into_iter() {
                    trait_name_table.declare(&trait_name, trait_name.clone())?;
                }
                let trait_name = trait_name_table.resolve(&call.name, "")?;

                let trayt = scope.context.traits.resolve(&trait_name, "")?;

                let mut inputs = vec![];
                for (param_name, expr) in call.inputs.into_iter() {
                    let eval = Evaluation::analyse(&expr, scope)?;

                    inputs.push((param_name, eval));
                }

                check_types(&trayt.borrow().inputs, &inputs, scope)?;

                Evaluation::Trait(TraitEvaluation {
                    trayt,
                    subject: Box::new(subject),
                    inputs,
                })
            }
            Expression::Let(call) => {
                let lett = scope.context.lets.resolve(&call.name, scope.path)?;

                let mut inputs = vec![];
                for (param_name, expr) in call.inputs.into_iter() {
                    let eval = Evaluation::analyse(&expr, scope)?;

                    inputs.push((param_name, eval));
                }

                check_types(&lett.borrow().inputs, &inputs, scope)?;

                Evaluation::Let(LetEvaluation { lett, inputs })
            }
            Expression::Literal(value) => Evaluation::Literal(value),
            Expression::Local(name) => {
                if !scope.locals.contains_key(&name) {
                    return error(NoResolution("Local Variable", name));
                }

                Evaluation::Local(name)
            }
            Expression::FriendlyField(ff) => {
                let _local = match scope.locals.get(&ff.local_name) {
                    Some(local) => local,
                    None => return error(NoResolution("Local Variable", ff.local_name)),
                };

                // TODO: check if locals is a struct, if it is of the same type as self, if it has the friendly field

                Evaluation::FriendlyField(ff)
            }
            Expression::Zelf => Evaluation::Zelf,
        };

        Ok(eval)
    }

    pub fn typ(&self, scope: &SemanticScope) -> CResult<Type> {
        let typ = match self {
            Evaluation::Let(call) => call.lett.borrow().output.clone(),
            Evaluation::Trait(call) => {
                if let Type::Raw(raw_type) = call.subject.typ(scope)? {
                    if &call.trayt.borrow().full_name == "String" {
                        // If this is a cast to raw string, return string
                        Type::Raw(RawType::String)
                    } else {
                        // If this is an operation on a raw type, the return type is the same raw type.
                        Type::Raw(raw_type)
                    }

                } else {
                    call.trayt.borrow().output.clone()
                }
            },
            Evaluation::Literal(raw_value) => Type::Raw(raw_value.into()),
            Evaluation::Local(name) => scope.locals.get(name).unwrap().clone(),
            Evaluation::FriendlyField(ff) => scope.locals
                .get(&format!("{}.{}", ff.local_name, ff.field_name))
                .unwrap()
                .clone(),
            Evaluation::Zelf => match &scope.zelf {
                Some(typ) => typ.clone(),
                None => return error(ErrorMessage::NoSelf),
            },
            Evaluation::ClassConstructor(class) => class.interface(),
            Evaluation::StructConstructor(strukt) => strukt.interface(),
        };

        Ok(typ)
    }
}
