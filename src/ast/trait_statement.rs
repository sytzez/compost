use std::ops::AddAssign;
use crate::ast::parser::Parser;
use crate::ast::typ::Type;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;

pub struct TraitStatement {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub output: Type,
}

pub struct TraitsStatement {
    pub traits: Vec<TraitStatement>,
}

impl Parser for TraitsStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Traits))
    }

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self> {
        let base_level = tokens[*position].1;
        position.add_assign(1);

        let mut statement = TraitsStatement {
            traits: vec![],
        };

        while *position < tokens.len() {
            if tokens[*position].1 <= base_level {
                break;
            }

            todo!("Parse traits")
        }

        Ok(statement)
    }
}