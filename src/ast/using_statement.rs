use crate::ast::parser::{parse_global, Parse};
use crate::error::CResult;
use crate::lex::token::{Kw, Op, Token};
use crate::lex::tokens::Tokens;

/// A single line of a 'using' statement
pub struct SingleUsingStatement {
    pub name: String,
    pub wildcard: bool,
}

/// All lines of a 'using' statement
pub struct UsingStatement {
    pub lines: Vec<SingleUsingStatement>,
}

impl Parse for UsingStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::Using))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let mut statement = UsingStatement { lines: vec![] };

        while tokens.deeper_than(base_level) {
            statement.lines.push(parse_single_using(tokens)?);
        }

        Ok(statement)
    }
}

fn parse_single_using(tokens: &mut Tokens) -> CResult<SingleUsingStatement> {
    let name = parse_global(tokens)?;
    let wildcard = parse_wildcard(tokens);
    let statement = SingleUsingStatement { name, wildcard };
    Ok(statement)
}

fn parse_wildcard(tokens: &mut Tokens) -> bool {
    if tokens.token() == &Token::Op(Op::Mul) {
        tokens.step();
        true
    } else {
        false
    }
}
