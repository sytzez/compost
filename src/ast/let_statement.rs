use std::ops::Range;
use crate::ast::expression::{Expression, ExpressionStatement};
use crate::ast::parser::{parse_global, parse_in_out_types, Parse};
use crate::ast::Statement;
use crate::ast::type_statement::TypeStatement;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};

use crate::lex::tokens::Tokens;

/// A single let which is made up of a name, optional parameters, an output type and the expression.
pub struct LetStatement {
    pub name: String,
    pub parameters: Vec<(String, TypeStatement)>,
    pub output: TypeStatement,
    pub expr: ExpressionStatement,
    token_range: Range<usize>,
}

/// The lets keywords and its lets.
pub struct LetsStatement {
    pub lets: Vec<LetStatement>,
}

impl LetsStatement {
    pub fn new() -> Self {
        Self { lets: vec![] }
    }
}

impl Parse for LetsStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::Lets))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut statement = LetsStatement::new();

        while tokens.deeper_than(base_level) {
            statement.lets.push(parse_let(tokens)?)
        }

        Ok(statement)
    }
}

fn parse_let(tokens: &mut Tokens) -> CResult<LetStatement> {
    let base_level = tokens.level();
    let token_start = tokens.position();

    tokens.expect("the name of a let (Starting with an upper-case letter)");
    let name = parse_global(tokens)?;
    let (parameters, output) = parse_in_out_types(tokens, base_level)?;
    let expr = ExpressionStatement::parse(tokens)?;

    let statement = LetStatement {
        name,
        parameters,
        output,
        expr,
        token_range: token_start..tokens.position(),
    };
    Ok(statement)
}

impl Statement for LetStatement {
    fn token_range(&self) -> &Range<usize> {
        &self.token_range
    }
}
