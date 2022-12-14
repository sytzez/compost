use crate::ast::parser::{parse_local, Parse};
use crate::ast::type_statement::RawType;
use crate::error::{CResult, ErrorMessage};
use crate::lex::token::{Kw, Token};

use crate::lex::tokens::Tokens;
use std::borrow::Borrow;
use std::ops::Range;
use crate::ast::Statement;

/// The struct keyword and its fields.
pub struct StructStatement {
    pub fields: Vec<(String, RawType)>,
    token_range: Range<usize>,
}

impl Parse for StructStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::Struct))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        let token_start = tokens.position();
        tokens.step();

        let mut fields = vec![];

        while tokens.deeper_than(base_level) {
            fields.push(parse_field(tokens)?)
        }

        let statement = StructStatement {
            fields,
            token_range: token_start..tokens.position(),
        };
        Ok(statement)
    }
}

fn parse_field(tokens: &mut Tokens) -> CResult<(String, RawType)> {
    tokens.expect("a field name (Starting with a lower-case letter)");
    let name = parse_local(tokens)?;

    tokens.expect("int, string or bool");
    let type_name = parse_local(tokens)?;

    let typ = match type_name.borrow() {
        "int" => RawType::Int,
        "string" => RawType::String,
        "bool" => RawType::Bool,
        _ => return tokens.error(ErrorMessage::UnknownRawType(type_name.clone())),
    };

    Ok((name, typ))
}

impl Statement for StructStatement {
    fn token_range(&self) -> &Range<usize> {
        &self.token_range
    }
}