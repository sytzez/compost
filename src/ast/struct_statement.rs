use crate::ast::parser::{parse_local, Parser};
use crate::ast::type_statement::RawType;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;
use std::borrow::Borrow;

/// The struct keyword and its fields.
pub struct StructStatement {
    pub fields: Vec<(String, RawType)>,
}

impl StructStatement {
    pub fn new() -> Self {
        Self { fields: vec![] }
    }
}

impl Parser for StructStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Struct))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut statement = StructStatement::new();

        while tokens.deeper_than(base_level) {
            let field = parse_field(tokens)?;

            statement.fields.push(field)
        }

        Ok(statement)
    }
}

fn parse_field(tokens: &mut Tokens) -> CResult<(String, RawType)> {
    let name = parse_local(tokens)?;

    let type_name = parse_local(tokens)?;

    let typ = match type_name.borrow() {
        "int" => RawType::Int,
        "string" => RawType::String,
        _ => return tokens.error(format!("Unknown struct field type {}", type_name)),
    };

    Ok((name, typ))
}
