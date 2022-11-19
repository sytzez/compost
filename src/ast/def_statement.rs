use crate::ast::expression::Expression;
use crate::ast::parser::{parse_global, Parser};
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

/// A single def.
pub struct DefStatement {
    pub name: String,
    pub expr: Expression,
}

/// The defs keyword and its defs.
pub struct DefsStatement {
    pub defs: Vec<DefStatement>,
}

impl Parser for DefsStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Defs))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut statement = DefsStatement { defs: vec![] };

        while tokens.deeper_than(base_level) {
            let def = parse_def(tokens)?;

            statement.defs.push(def)
        }

        Ok(statement)
    }
}

fn parse_def(tokens: &mut Tokens) -> CResult<DefStatement> {
    let name = parse_global(tokens)?;
    let expr = Expression::parse(tokens)?;
    let statement = DefStatement { name, expr };
    Ok(statement)
}
