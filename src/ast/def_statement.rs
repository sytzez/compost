use crate::ast::expression::ExpressionStatement;
use crate::ast::parser::{parse_global, Parse};
use crate::ast::Statement;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use std::ops::Range;

use crate::lex::tokens::Tokens;

/// A single def.
pub struct DefStatement {
    pub name: String,
    pub expr: ExpressionStatement,
    token_range: Range<usize>,
}

/// The defs keyword and its defs.
pub struct DefsStatement {
    pub defs: Vec<DefStatement>,
}

impl Parse for DefsStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::Defs))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut defs = vec![];

        while tokens.deeper_than(base_level) {
            defs.push(parse_def(tokens)?)
        }

        let statement = DefsStatement { defs };
        Ok(statement)
    }
}

fn parse_def(tokens: &mut Tokens) -> CResult<DefStatement> {
    let token_start = tokens.position();
    tokens.expect("trait name for definition (Starting with upper-case letter)");
    let name = parse_global(tokens)?;
    let expr = ExpressionStatement::parse(tokens)?;
    let statement = DefStatement {
        name,
        expr,
        token_range: token_start..tokens.position(),
    };
    Ok(statement)
}

impl Statement for DefStatement {
    fn token_range(&self) -> &Range<usize> {
        &self.token_range
    }
}
