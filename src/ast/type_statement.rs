use crate::ast::parser::Parser;
use crate::error::CResult;
use crate::lex::token::{Kw, Op, Token};

use crate::lex::tokens::Tokens;

pub enum TypeStatement {
    Name(String),
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
}

impl Parser for TypeStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Global(_) | Token::Kw(Kw::Zelf))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let typ = match tokens.token() {
            Token::Kw(Kw::Zelf) => TypeStatement::Zelf,
            Token::Global(name) => TypeStatement::Name(name.clone()),
            Token::Op(Op::Question) => TypeStatement::Void,
            _ => return tokens.unexpected_token_error(),
        };

        tokens.step();

        let typ = match tokens.token().clone() {
            Token::Op(op @ (Op::And | Op::Or)) => {
                tokens.step();

                let lhs = Box::new(typ);
                let rhs = Box::new(TypeStatement::parse(tokens)?);

                match op {
                    Op::And => TypeStatement::And(lhs, rhs),
                    Op::Or => TypeStatement::Or(lhs, rhs),
                    _ => unreachable!(),
                }
            }
            _ => typ,
        };

        Ok(typ)
    }
}
