use crate::ast::parser::{Parse};
use crate::ast::raw_value::RawValue;
use crate::error::CResult;
use crate::lex::token::{Kw, Lit, Op, Token};

use crate::ast::expr::match_call::MatchCall;

use crate::ast::expr::if_else_call::IfElseCall;
use crate::ast::Statement;
use crate::lex::tokens::Tokens;
use std::collections::HashMap;
use std::ops::Range;
use crate::ast::expr::let_call::LetCall;

#[derive(Clone, Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
    token_range: Range<usize>,
}

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
    IfElse(IfElseCall),
    Zelf,
    Void,
}

#[derive(Clone, Debug)]
pub struct BinaryCall {
    pub op: BinaryOp,
    pub lhs: Box<ExpressionStatement>,
    pub rhs: Box<ExpressionStatement>,
}

#[derive(Clone, Debug)]
pub struct UnaryCall {
    pub op: UnaryOp,
    pub subject: Box<ExpressionStatement>,
}

#[derive(Clone, Debug)]
pub struct DefCall {
    pub name: String,
    pub subject: Box<ExpressionStatement>,
    pub inputs: HashMap<String, ExpressionStatement>,
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

impl Parse for ExpressionStatement {
    fn matches(_tokens: &Tokens) -> bool {
        true
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        let token_start = tokens.position();

        // Parse first token
        tokens.expect("an expression");
        let mut expr = match tokens.token().clone() {
            Token::Kw(Kw::Zelf) => {
                tokens.step();

                Expression::Zelf
            }
            Token::Global(_) => {
                let call = LetCall::parse(tokens)?;

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
                    Lit::Boolean(value) => RawValue::Bool(value),
                })
            }
            Token::Op(Op::Dot) => {
                // We don't step so we can reevaluate the same dot in the next step.
                Expression::Zelf
            }
            Token::Op(Op::Sub) => {
                tokens.step();

                let expr = ExpressionStatement::parse(tokens)?;

                Expression::Unary(UnaryCall {
                    op: UnaryOp::Neg,
                    subject: Box::new(expr),
                })
            }
            Token::Kw(Kw::Match) => Expression::Match(MatchCall::parse(tokens)?),
            Token::Kw(Kw::If) => Expression::IfElse(IfElseCall::parse(tokens)?),
            Token::Op(Op::Question) => {
                tokens.step();
                let statement = ExpressionStatement {
                    expression: Expression::Void,
                    token_range: token_start..tokens.position(),
                };
                return Ok(statement);
            }
            _ => return tokens.unexpected_token_error(),
        };

        // Parse further operations
        while tokens.deeper_than_or_eq(base_level) {
            expr = match tokens.token().clone() {
                Token::Op(op) => match op {
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

                        let lhs = ExpressionStatement {
                            expression: expr,
                            token_range: token_start..tokens.position(),
                        };
                        let rhs = ExpressionStatement::parse(tokens)?;

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
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        })
                    }
                    Op::Dot => {
                        tokens.step();

                        tokens.expect("a trait name or a friendly field name");
                        match (expr, tokens.token().clone()) {
                            (Expression::Local(local_name), Token::Local(field_name)) => {
                                tokens.step();

                                Expression::FriendlyField(FriendlyField {
                                    local_name,
                                    field_name,
                                })
                            }
                            (expr, Token::Global(_)) => {
                                let call = LetCall::parse(tokens)?;

                                let subject = ExpressionStatement {
                                    expression: expr,
                                    token_range: token_start..tokens.position(),
                                };

                                Expression::Def(DefCall {
                                    name: call.name,
                                    subject: Box::new(subject),
                                    inputs: call.inputs,
                                })
                            }
                            _ => return tokens.unexpected_token_error(),
                        }
                    }
                    _ => break,
                },
                _ => break,
            }
        }

        let statement = ExpressionStatement {
            expression: expr,
            token_range: token_start..tokens.position(),
        };
        Ok(statement)
    }
}

impl Statement for ExpressionStatement {
    fn token_range(&self) -> &Range<usize> {
        &self.token_range
    }
}
