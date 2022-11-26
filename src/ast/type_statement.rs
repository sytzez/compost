use crate::ast::parser::Parser;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
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
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Global(_) | Token::Kw(Kw::Zelf))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        // TODO: actually implement
        tokens.step();

        Ok(TypeStatement::Void)
    }
}
