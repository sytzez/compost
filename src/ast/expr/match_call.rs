use crate::ast::expression::Expression;
use crate::ast::parser::{parse_local, Parse};
use crate::ast::type_statement::TypeStatement;
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokens::Tokens;

/// A match statement. It matches the subject's type to different branches.
#[derive(Clone, Debug)]
pub struct MatchCall {
    pub local_name: String,
    pub subject: Box<Expression>,
    pub branches: Vec<(TypeStatement, Box<Expression>)>,
}

impl Parse for MatchCall {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::Match))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let local_name = parse_local(tokens)?;
        let subject = Box::new(Expression::parse(tokens)?);
        let mut branches = vec![];

        while tokens.deeper_than(base_level) {
            let type_statement = TypeStatement::parse(tokens)?;
            let branch = Box::new(Expression::parse(tokens)?);

            branches.push((type_statement, branch));
        }

        let call = MatchCall {
            local_name,
            subject,
            branches,
        };
        Ok(call)
    }
}
