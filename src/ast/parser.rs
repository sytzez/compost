use crate::error::CResult;
use crate::lex::token::Token;
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

pub trait Parser where Self: Sized {
    fn matches(tokens: &[LeveledToken]) -> bool;

    fn parse(tokens: &mut Tokens) -> CResult<Self>;

    fn parse_maybe(tokens: &mut Tokens) -> CResult<Option<Self>> {
        if Self::matches(tokens.remaining()) {
            Ok(Some(Self::parse(tokens)?))
        } else {
            Ok(None)
        }
    }
}

pub fn parse_global(tokens: &mut Tokens) -> CResult<String> {
    if let (Token::Global(name), _) = tokens.current() {
        tokens.step();
        Ok(name.clone())
    } else {
        tokens.error(format!("Expected global name, got {:?} ", tokens.current()))
    }
}

pub fn parse_local(tokens: &mut Tokens) -> CResult<String> {
    if let (Token::Local(name), _) = tokens.current() {
        tokens.step();
        Ok(name.clone())
    } else {
        tokens.error(format!("Expected local name, got {:?} ", tokens.current()))
    }
}
