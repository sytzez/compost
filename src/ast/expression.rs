use crate::ast::parser::{parse_global, Parser};
use crate::ast::raw_value::RawValue;
use crate::error::CResult;
use crate::lex::token::{Kw, Lit, Op, Token};

use crate::ast::expr::match_call::MatchCall;

use crate::lex::tokens::Tokens;
use std::collections::HashMap;

/// An expression within the abstract syntax tree.
#[derive(Clone, Debug)]
pub enum Expression {
    Binary(BinaryCall),
    Unary(UnaryCall),
    Let(LetCall),
    Def(DefCall),
    Literal(RawValue),
    Local(String),
    FriendlyField(FriendlyField),
    Match(MatchCall),
    Zelf,
    Void,
}

#[derive(Clone, Debug)]
pub struct BinaryCall {
    pub op: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct UnaryCall {
    pub op: UnaryOp,
    pub subject: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct LetCall {
    pub name: String,
    pub inputs: HashMap<String, Expression>,
}

#[derive(Clone, Debug)]
pub struct DefCall {
    pub name: String,
    pub subject: Box<Expression>,
    pub inputs: HashMap<String, Expression>,
}

// A reference to the protected field of another instance of the self struct
#[derive(Clone, Debug)]
pub struct FriendlyField {
    pub local_name: String,
    pub field_name: String,
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Gt,
    And,
    Or,
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

impl Parser for Expression {
    fn matches(_tokens: &Tokens) -> bool {
        true
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();

        // Parse first token
        let mut expr = match tokens.token().clone() {
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

                Expression::Local(name)
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

                Expression::Unary(UnaryCall {
                    op: UnaryOp::Neg,
                    subject: Box::new(expr),
                })
            }
            Token::Kw(Kw::Match) => Expression::Match(MatchCall::parse(tokens)?),
            Token::Op(Op::Question) => {
                tokens.step();
                return Ok(Expression::Void);
            }
            _ => return tokens.unexpected_token_error(),
        };

        // Parse further operations
        while tokens.deeper_than_or_eq(base_level) {
            expr = match tokens.token().clone() {
                Token::Op(op) => {
                    match op {
                        Op::Add
                        | Op::Sub
                        | Op::Mul
                        | Op::Div
                        | Op::Eq
                        | Op::Lt
                        | Op::Gt
                        | Op::And
                        | Op::Or => {
                            tokens.step();

                            let rhs = Expression::parse(tokens)?;

                            Expression::Binary(BinaryCall {
                                op: match op {
                                    Op::Add => BinaryOp::Add,
                                    Op::Sub => BinaryOp::Sub,
                                    Op::Mul => BinaryOp::Mul,
                                    Op::Div => BinaryOp::Div,
                                    Op::Eq => BinaryOp::Eq,
                                    Op::Lt => BinaryOp::Lt,
                                    Op::Gt => BinaryOp::Gt,
                                    Op::And => BinaryOp::And,
                                    Op::Or => BinaryOp::Or,
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
                                        name: call.name,
                                        subject: Box::new(expr),
                                        inputs: call.inputs,
                                    })
                                }
                                _ => return tokens.unexpected_token_error(),
                            }
                        }
                        _ => break,
                    }
                }
                _ => break,
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

    let call = LetCall { name, inputs };

    Ok(call)
}
