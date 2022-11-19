use crate::ast::parser::Parser;
use crate::ast::typ::RawType;
use crate::error::CResult;
use crate::lex::tokenizer::LeveledToken;

pub struct StructStatement {
    pub fields: Vec<(String, RawType)>,
}

impl Parser for StructStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        todo!()
    }

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self> {
        todo!()
    }
}