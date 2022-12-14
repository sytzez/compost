use crate::error::ErrorContext::TokenRange;
use crate::error::{CResult, CompilationError, ErrorMessage};
use std::ops::Range;

pub(crate) mod abstract_syntax_tree;
pub(crate) mod class_statement;
pub(crate) mod def_statement;
pub(crate) mod expr;
pub(crate) mod expression;
pub(crate) mod let_statement;
pub(crate) mod module_statement;
pub(crate) mod parser;
pub(crate) mod raw_value;
pub(crate) mod struct_statement;
pub(crate) mod trait_statement;
pub(crate) mod type_statement;
pub(crate) mod using_statement;

pub(crate) trait Statement {
    fn token_range(&self) -> &Range<usize>;
    fn error<T>(&self, message: ErrorMessage) -> CResult<T> {
        Err(CompilationError {
            message,
            context: Some(TokenRange(self.token_range().clone())),
        })
    }
}
