use crate::ast::expression::ExpressionStatement;
use crate::ast::parser::Parse;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokens::Tokens;

/// An if else statement. Because Compost is an expression language, there always needs
/// to be an else clause, in order to always have an output value.
#[derive(Clone, Debug)]
pub struct IfElseCall {
    pub condition: Box<ExpressionStatement>,
    pub iff: Box<ExpressionStatement>,
    pub els: Box<ExpressionStatement>,
}

impl Parse for IfElseCall {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::If))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        tokens.step();

        let condition = ExpressionStatement::parse(tokens)?;

        if !matches!(tokens.token(), Token::Kw(Kw::Then)) {
            return tokens.unexpected_token_error();
        }
        tokens.step();

        let iff = ExpressionStatement::parse(tokens)?;

        if !matches!(tokens.token(), Token::Kw(Kw::Else)) {
            return tokens.unexpected_token_error();
        }
        tokens.step();

        let els = ExpressionStatement::parse(tokens)?;

        let call = IfElseCall {
            condition: Box::new(condition),
            iff: Box::new(iff),
            els: Box::new(els),
        };
        Ok(call)
    }
}
