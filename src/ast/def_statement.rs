use crate::ast::expression::Expression;
use crate::ast::parser::Parser;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;

pub struct DefStatement {
    pub name: String,
    pub expr: Expression,
}

pub struct DefsStatement {
    pub defs: Vec<DefStatement>,
}

impl Parser for DefsStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Defs))
    }

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self> {
        let base_level = tokens[*position].1;
        position.add_assign(1);

        let mut statement = DefsStatement {
            defs: vec![],
        };

        while *position < tokens.len() {
            if tokens[*position].1 <= base_level {
                break;
            }

            todo!("Parse defs")
        }

        Ok(statement)
    }
}