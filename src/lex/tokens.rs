use crate::error::{CResult, CompilationError};
use crate::lex::token::Token;
use crate::lex::tokenizer::LeveledToken;

/// Provides semantic utility functions for traversing the tokens.
pub struct Tokens {
    tokens: Vec<LeveledToken>,
    position: usize,
}

impl Tokens {
    pub fn new(tokens: Vec<LeveledToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Advance one step.
    pub fn step(&mut self) {
        self.position += 1;
    }

    /// Advance a number of steps.
    pub fn steps(&mut self, amount: usize) {
        self.position += amount;
    }

    /// Whether there are more tokens left.
    pub fn still_more(&self) -> bool {
        self.position < self.tokens.len()
    }

    /// Whether the current token is deeper than the given level.
    pub fn deeper_than(&self, level: usize) -> bool {
        self.still_more() && self.level() > level
    }

    pub fn deeper_than_or_eq(&self, level: usize) -> bool {
        self.still_more() && self.level() >= level
    }

    /// The remaining tokens.
    pub fn remaining(&self) -> &[LeveledToken] {
        &self.tokens[self.position..]
    }

    /// The current token and level.
    pub fn leveled_token(&self) -> &LeveledToken {
        &self.tokens[self.position]
    }

    /// The current token.
    pub fn token(&self) -> &Token {
        &self.tokens[self.position].0
    }

    /// The current level.
    pub fn level(&self) -> usize {
        self.tokens[self.position].1
    }

    /// Create an error at the current position.
    pub fn error<T>(&self, message: String) -> CResult<T> {
        Err(CompilationError {
            message,
            position: self.position,
        })
    }
}
