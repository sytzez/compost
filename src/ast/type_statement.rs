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
        let typ = match tokens.token() {
            Token::Kw(Kw::Zelf) => TypeStatement::Zelf,
            Token::Global(name) => TypeStatement::Name(name.clone()),
            _ => return tokens.unexpected_token_error(),
        };

        tokens.step();

        // TODO: parse & and |

        Ok(typ)
    }
}
