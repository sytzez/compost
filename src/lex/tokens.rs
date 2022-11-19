use crate::error::{CompilationError, CResult};
use crate::lex::tokenizer::LeveledToken;

/// Provides semantic utility functions for traversing the tokens.
pub struct Tokens {
    pub tokens: Vec<LeveledToken>,
    pub position: usize,
}

impl Tokens {
    pub fn step(&mut self) {
        self.position += 1;
    }

    pub fn steps(&mut self, amount: usize) {
        self.position += amount;
    }

    pub fn still_more(&self) -> bool {
        self.position < self.tokens.len()
    }

    pub fn remaining(&self) -> &[LeveledToken] {
        &self.tokens[self.position..]
    }

    pub fn current(&self) -> &LeveledToken {
        &self.tokens[self.position]
    }

    pub fn level(&self) -> usize {
        self.tokens[self.position].1
    }

    pub fn error<T>(&self, message: String) -> CResult<T> {
        Err(CompilationError { message, position: self.position })
    }
}