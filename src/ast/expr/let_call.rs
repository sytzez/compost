use std::collections::HashMap;
use crate::ast::expression::ExpressionStatement;
use crate::ast::parser::{Parse, parse_global};
use crate::error::CResult;
use crate::lex::token::Token;
use crate::lex::tokens::Tokens;

/// e.g.: Module\Function(param1: ..., param2: ...)
/// e.g.: Module\Constant
#[derive(Clone, Debug)]
pub struct LetCall {
    pub name: String,
    pub inputs: HashMap<String, ExpressionStatement>,
}

impl Parse for LetCall {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Global(_))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        let name = parse_global(tokens)?;

        let mut inputs = HashMap::new();

        while tokens.deeper_than(base_level) {
            if let Token::Local(param_name) = tokens.token().clone() {
                tokens.step();

                let expr = ExpressionStatement::parse(tokens)?;

                inputs.insert(param_name, expr);
            } else {
                break;
            }
        }

        let call = LetCall { name, inputs };

        Ok(call)
    }
}
