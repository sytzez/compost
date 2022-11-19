use crate::ast::parser::{parse_parameter, Parser};
use crate::ast::typ::Type;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

/// The class keyword and its dependencies.
pub struct ClassStatement {
    pub dependencies: Vec<(String, Type)>,
}

impl Parser for ClassStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Class))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut statement = ClassStatement {
            dependencies: vec![],
        };

        while tokens.deeper_than(base_level) {
            let dependency = parse_parameter(tokens)?;

            statement.dependencies.push(dependency)
        }

        Ok(statement)
    }
}
