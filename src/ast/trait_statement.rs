use crate::ast::parser::{parse_global, parse_in_out_types, parse_parameter, Parser};
use crate::ast::type_statement::TypeStatement;
use crate::error::CResult;
use crate::lex::token::{Kw, Op, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

/// A single trait.
pub struct TraitStatement {
    pub name: String,
    pub parameters: Vec<(String, TypeStatement)>,
    pub output: TypeStatement,
}

/// The traits keyword and its traits.
pub struct TraitsStatement {
    pub traits: Vec<TraitStatement>,
}

impl TraitsStatement {
    pub fn new() -> Self {
        Self {
            traits: vec![],
        }
    }
}

impl Parser for TraitsStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Traits))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut statement = TraitsStatement::new();

        while tokens.deeper_than(base_level) {
            let trayt = parse_trait(tokens)?;

            statement.traits.push(trayt)
        }

        Ok(statement)
    }
}

fn parse_trait(tokens: &mut Tokens) -> CResult<TraitStatement> {
    let base_level = tokens.level();
    let name = parse_global(tokens)?;
    let (parameters, output) = parse_in_out_types(tokens, base_level)?;
    let statement = TraitStatement { name, parameters, output, };
    Ok(statement)
}