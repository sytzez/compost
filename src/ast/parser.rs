use crate::error::CResult;
use crate::lex::tokenizer::LeveledToken;

pub trait Parser where Self: Sized {
    fn matches(tokens: &[LeveledToken]) -> bool;

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self>;

    fn parse_maybe(tokens: &[LeveledToken], position: &mut usize) -> CResult<Option<Self>> {
        if Self::matches(&tokens[*position..]) {
            Ok(Some(Self::parse(tokens, position)?))
        } else {
            Ok(None)
        }
    }
}
