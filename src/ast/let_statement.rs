use crate::ast::expression::Expression;
use crate::ast::parser::{Parser, ParseResult};
use crate::ast::typ::Type;
use crate::error::CResult;
use crate::lex::tokenizer::LeveledToken;

pub struct LetStatement {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub output: Type,
    pub expr: Expression,
}

pub struct LetsStatement {
    pub lets: Vec<LetStatement>,
}

impl Parser for LetsStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        todo!()
    }

    fn parse(tokens: &[LeveledToken]) -> CResult<ParseResult<Self>> {
        todo!()
    }
}