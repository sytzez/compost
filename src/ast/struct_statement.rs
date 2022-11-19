use crate::ast::parser::{parse_local, Parser};
use crate::ast::typ::RawType;
use crate::error::{CResult, error};
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;

pub struct StructStatement {
    pub fields: Vec<(String, RawType)>,
}

impl Parser for StructStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Struct))
    }

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self> {
        let base_level = tokens[*position].1;
        position.add_assign(1);

        let mut statement = StructStatement {
            fields: vec![],
        };

        while *position < tokens.len() {
            if tokens[*position].1 <= base_level {
                break;
            }

            statement.fields.push(parse_field(tokens, position)?)
        }

        Ok(statement)
    }
}

fn parse_field(tokens: &[LeveledToken], position: &mut usize) -> CResult<(String, Type)> {
    let name = parse_local(tokens, position)?;

    let type_name = parse_local(tokens, position)?;

    let typ = match type_name.borrow() {
        "int" => RawType::Int,
        "string" => RawType::String,
        _ => return error(format!("Unknown struct field type {}", type_name), *position),
    };

    Ok((name, typ))
}
