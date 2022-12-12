use std::cell::RefCell;

use crate::ast::expression::{BinaryOp, Expression, FriendlyField, UnaryOp};
use crate::ast::raw_value::RawValue;
use crate::ast::type_statement::RawType;
use crate::error::ErrorMessage::NoResolution;
use crate::error::{error, CResult, ErrorMessage};
use crate::sem::class::Class;
use crate::sem::lett::Let;
use crate::sem::semantic_analyser::SemanticScope;
use crate::sem::strukt::Struct;
use crate::sem::table::Table;
use crate::sem::trayt::Trait;
use crate::sem::typ::{combine_types, Type};
use crate::sem::type_checking::check_types;
use crate::sem::type_coercion::{coerce_type, coerce_types};
use std::rc::Rc;

/// A semantically analysed expression that can be evaluated.
#[derive(Clone, Debug)]
pub enum Evaluation {
    Let(LetEvaluation),
    Trait(TraitEvaluation),
    Literal(RawValue),
    Local(String),
    FriendlyField(FriendlyField),
    Match(MatchEvaluation),
    Zelf,
    Void,
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

#[derive(Clone, Debug)]
pub struct MatchEvaluation {
    pub local_name: String,
    pub subject: Box<Evaluation>,
    pub branches: Vec<(Type, Box<Evaluation>)>,
}

impl Evaluation {
    pub fn analyse(expr: Expression, scope: &SemanticScope) -> CResult<Self> {
        let eval = match expr {
            Expression::Binary(call) => {
                let trait_path = match call.op {
                    BinaryOp::Add => "Op\\Add",
                    BinaryOp::Sub => "Op\\Sub",
                    BinaryOp::Mul => "Op\\Mul",
                    BinaryOp::Div => "Op\\Div",
                    BinaryOp::Eq => "Op\\Eq",
                    BinaryOp::Lt => "Op\\Lt",
                    BinaryOp::Gt => "Op\\Gt",
                    BinaryOp::And => "Op\\And",
                    BinaryOp::Or => "Op\\Or",
                };
                let trayt = scope.context.traits.resolve(trait_path, "")?;

                let mut lhs = Evaluation::analyse(*call.lhs, scope)?;
                let mut rhs = Evaluation::analyse(*call.rhs, scope)?;

                let left_type = lhs.typ(scope)?;
                let right_type = rhs.typ(scope)?;

                let left_is_raw = matches!(left_type, Type::Raw(_));
                let right_is_raw = matches!(right_type, Type::Raw(_));

                // If one is raw and the other is not, coerce one side into a struct.
                if left_is_raw && !right_is_raw {
                    coerce_type(&right_type, &mut lhs, scope)?;
                } else if right_is_raw && !left_is_raw {
                    coerce_type(&left_type, &mut rhs, scope)?;
                }

                let inputs = vec![("rhs".into(), rhs)];

                // check_types(&trayt.borrow().inputs, &inputs, scope)?;

                Evaluation::Trait(TraitEvaluation {
                    trayt,
                    subject: Box::new(lhs),
                    inputs,
                })
            }
            Expression::Unary(call) => {
                let trait_path = match call.op {
                    UnaryOp::Neg => "Op\\Neg",
                    UnaryOp::Not => "Op\\Not",
                };
                let trayt = scope.context.traits.resolve(trait_path, "")?;

                let subject = Evaluation::analyse(*call.subject, scope)?;

                Evaluation::Trait(TraitEvaluation {
                    trayt,
                    subject: Box::new(subject),
                    inputs: vec![],
                })
            }
            Expression::Def(call) => {
                let subject = Evaluation::analyse(*call.subject, scope)?;

                // Make a temporary trait table using only traits defined on the subject.
                let mut trait_name_table = Table::new("Trait");
                for trait_name in subject.typ(scope)?.callable_traits(scope).into_iter() {
                    trait_name_table.declare(&trait_name, trait_name.clone())?;
                }
                let trait_name = trait_name_table.resolve(&call.name, "")?;

                let trayt = scope.context.traits.resolve(&trait_name, "")?;

                // Resolve all 'Self' types within input types with the current subject.
                let subject_type = subject.typ(scope)?;
                let input_types = trayt
                    .borrow()
                    .inputs
                    .iter()
                    .map(|(name, typ)| (name.clone(), resolve_self_types(typ.clone(), &subject_type)))
                    .collect::<Vec<_>>();

                let mut inputs = vec![];
                for (param_name, expr) in call.inputs.into_iter() {
                    let eval = Evaluation::analyse(expr, scope)?;

                    inputs.push((param_name, eval));
                }

                coerce_types(&input_types, &mut inputs, scope)?;
                check_types(&input_types, &inputs, scope)?;

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
                    let eval = Evaluation::analyse(expr, scope)?;

                    inputs.push((param_name, eval));
                }

                coerce_types(&lett.borrow().inputs, &mut inputs, scope)?;
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
            Expression::Match(call) => {
                let mut branches = vec![];
                for (type_statement, expr) in call.branches {
                    let typ = Type::analyse(&type_statement, scope.context, scope.path)?;

                    // Add the matched local to the scope for this branch.
                    let mut branch_scope = scope.clone();
                    branch_scope
                        .locals
                        .insert(call.local_name.clone(), typ.clone());

                    let eval = Box::new(Evaluation::analyse(*expr, &branch_scope)?);

                    branches.push((typ, eval));
                }

                let match_eval = MatchEvaluation {
                    local_name: call.local_name,
                    subject: Box::new(Evaluation::analyse(*call.subject, scope)?),
                    branches,
                };

                // TODO: check if branches cover all possibilities by combining all branch types with & and checking
                // that it covers the complete subject type.

                Evaluation::Match(match_eval)
            }
            Expression::Zelf => Evaluation::Zelf,
            Expression::Void => Evaluation::Void,
        };
        Ok(eval)
    }

    /// Returns the type of the result of this evaluation.
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
                    let output = &call.trayt.borrow().output;

                    if matches!(output, Type::Zelf) {
                        // If the trait returns a Self type, the output type is identical to the subject's type.
                        call.subject.typ(scope)?
                    } else {
                        output.clone()
                    }
                }
            }
            Evaluation::Literal(raw_value) => Type::Raw(raw_value.into()),
            Evaluation::Local(name) => scope.locals.get(name).unwrap().clone(),
            Evaluation::FriendlyField(ff) => scope
                .locals
                .get(&format!("{}.{}", ff.local_name, ff.field_name))
                .unwrap()
                .clone(),
            Evaluation::Match(call) => {
                let mut types = vec![];
                for (typ, branch) in &call.branches {
                    // Add matched type to scope for each branch to determine the output type.
                    let mut scope = scope.clone();
                    scope.locals.insert(call.local_name.clone(), typ.clone());

                    types.push(branch.typ(&scope)?)
                }

                combine_types(types)
            }
            Evaluation::Zelf => match &scope.zelf {
                Some(typ) => typ.clone(),
                None => return error(ErrorMessage::NoSelf),
            },
            Evaluation::Void => Type::Void,
            Evaluation::ClassConstructor(class) => class.interface(),
            Evaluation::StructConstructor(strukt) => strukt.interface(),
        };

        // Substitute 'Self' for actual type if possible
        let typ = match &scope.zelf {
            Some(self_typ) => resolve_self_types(typ, self_typ),
            None => typ,
        };

        Ok(typ)
    }
}

/// Replace self types with specific type.
fn resolve_self_types(typ: Type, self_type: &Type) -> Type {
    match typ {
        Type::Zelf => self_type.clone(),
        Type::Or(a, b) => {
            Type::Or(
                Box::new(resolve_self_types(*a, self_type)),
                Box::new(resolve_self_types(*b, self_type)),
            )
        }
        Type::And(a, b) => {
            Type::And(
                Box::new(resolve_self_types(*a, self_type)),
                Box::new(resolve_self_types(*b, self_type)),
            )
        }
        _ => typ,
    }
}