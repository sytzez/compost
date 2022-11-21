use crate::ast::expression::Expression;
use crate::ast::parser::{parse_global, parse_in_out_types, Parser};
use crate::ast::type_statement::TypeStatement;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

/// A single let which is made up of a name, optional parameters, an output type and the expression.
pub struct LetStatement {
    pub name: String,
    pub parameters: Vec<(String, TypeStatement)>,
    pub output: TypeStatement,
    pub expr: Expression,
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

impl Parser for LetsStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Lets))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut statement = LetsStatement::new();

        while tokens.deeper_than(base_level) {
            let lett = parse_let(tokens)?;

            statement.lets.push(lett)
        }

        Ok(statement)
    }
}

fn parse_let(tokens: &mut Tokens) -> CResult<LetStatement> {
    let base_level = tokens.level();
    let name = parse_global(tokens)?;
    let (parameters, output) = parse_in_out_types(tokens, base_level)?;
    let expr = Expression::parse(tokens)?;
    let statement = LetStatement {
        name,
        parameters,
        output,
        expr,
    };
    Ok(statement)
}
