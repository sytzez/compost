use std::ops::AddAssign;
use crate::ast::parser::Parser;
use crate::ast::typ::Type;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::parser::parse_local;

pub struct ClassStatement {
    pub dependencies: Vec<(String, Type)>,
}

impl Parser for ClassStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Class))
    }

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self> {
        let base_level = tokens[*position].1;
        position.add_assign(1);

        let mut statement = ClassStatement {
            dependencies: vec![],
        };

        while *position < tokens.len() {
            if tokens[*position].1 <= base_level {
                break;
            }

            statement.dependencies.push(parse_dependency(tokens, position)?)
        }

        Ok(statement)
    }
}

fn parse_dependency(tokens: &[LeveledToken], position: &mut usize) -> CResult<(String, Type)> {
    let base_level = tokens[*position].1;
    let name = parse_local(&tokens[*position], base_level)?;
    position.add_assign(1);

    let typ = Type::parse(tokens, position)?;

    Ok((name, typ))
}