use std::ops::Range;
use crate::ast::parser::{parse_parameter, Parse};
use crate::ast::Statement;
use crate::ast::type_statement::TypeStatement;

use crate::error::CResult;
use crate::lex::token::{Kw, Token};

use crate::lex::tokens::Tokens;

/// The class keyword and its dependencies.
pub struct ClassStatement {
    pub dependencies: Vec<(String, TypeStatement)>,
    pub token_range: Range<usize>,
}

impl Parse for ClassStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::Class))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        let token_start = tokens.position();
        tokens.step();

        let mut dependencies = vec![];
        while tokens.deeper_than(base_level) {
            let dependency = parse_parameter(tokens)?;
            dependencies.push(dependency)
        }

        let statement = ClassStatement {
            dependencies,
            token_range: token_start..tokens.position(),
        };
        Ok(statement)
    }
}

impl Statement for ClassStatement {
    fn token_range(&self) -> &Range<usize> {
        &self.token_range
    }
}