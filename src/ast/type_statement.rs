use crate::ast::parser::{parse_global, Parse};
use crate::ast::raw_value::RawValue;
use crate::ast::Statement;
use crate::error::CResult;
use crate::lex::token::{Kw, Op, Token};
use std::ops::Range;

use crate::lex::tokens::Tokens;

#[derive(Clone, Debug)]
pub struct TypeStatement {
    pub typ: TypeStatementType,
    token_range: Range<usize>,
}

#[derive(Clone, Debug)]
pub enum TypeStatementType {
    Name(String),
    AtName(String),
    And(Box<TypeStatement>, Box<TypeStatement>),
    Or(Box<TypeStatement>, Box<TypeStatement>),
    // Self, the class or struct the trait is defined on
    Zelf,
    // No traits, no interaction possible
    Void,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum RawType {
    Int,
    String,
    Bool,
}

impl Statement for TypeStatement {
    fn token_range(&self) -> &Range<usize> {
        &self.token_range
    }
}

impl Parse for TypeStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(
            tokens.token(),
            Token::Global(_) | Token::Kw(Kw::Zelf) | Token::Op(Op::At | Op::Question)
        )
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let token_start = tokens.position();

        tokens.expect("a trait name, @ followed by a trait name, a module name, 'Self' or '?'");
        let typ = match tokens.token_and_step() {
            Token::Kw(Kw::Zelf) => TypeStatementType::Zelf,
            Token::Global(name) => TypeStatementType::Name(name.clone()),
            Token::Op(Op::Question) => TypeStatementType::Void,
            Token::Op(Op::At) => TypeStatementType::AtName(parse_global(tokens)?),
            _ => return tokens.unexpected_token_error(),
        };

        let typ = match tokens.token().clone() {
            Token::Op(op @ (Op::And | Op::Or)) => {
                tokens.step();

                let lhs = Box::new(TypeStatement {
                    typ,
                    token_range: token_start..tokens.position(),
                });
                let rhs = Box::new(TypeStatement::parse(tokens)?);

                match op {
                    Op::And => TypeStatementType::And(lhs, rhs),
                    Op::Or => TypeStatementType::Or(lhs, rhs),
                    _ => unreachable!(),
                }
            }
            _ => typ,
        };

        let statement = TypeStatement {
            typ,
            token_range: token_start..tokens.position(),
        };
        Ok(statement)
    }
}

impl From<&RawValue> for RawType {
    fn from(value: &RawValue) -> Self {
        match value {
            RawValue::String(_) => RawType::String,
            RawValue::Int(_) => RawType::Int,
            RawValue::Bool(_) => RawType::Bool,
        }
    }
}
