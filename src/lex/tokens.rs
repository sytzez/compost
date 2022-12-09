use crate::error::{CResult, CompilationError, ErrorContext, ErrorMessage};
use crate::lex::token::Token;
use crate::lex::tokenizer::LeveledToken;

/// Provides utility functions that help traversing the tokens.
pub struct Tokens {
    tokens: Vec<LeveledToken>,
    position: usize,
}

impl Tokens {
    /// Advance to the next token.
    pub fn step(&mut self) {
        self.position += 1;
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

    /// The current token.
    pub fn token(&self) -> &Token {
        &self.tokens[self.position].0
    }

    /// Returns the current token and step to the next.
    pub fn token_and_step(&mut self) -> &Token {
        self.position += 1;
        &self.tokens[self.position - 1].0
    }

    /// The current level.
    pub fn level(&self) -> usize {
        self.tokens[self.position].1
    }

    /// Create an error at the current position.
    pub fn error<T>(&self, message: ErrorMessage) -> CResult<T> {
        Err(CompilationError {
            message,
            context: Some(ErrorContext::Token(self.position)),
        })
    }

    /// Create an unexpected token error at the current position
    pub fn unexpected_token_error<T>(&self) -> CResult<T> {
        Err(CompilationError {
            message: ErrorMessage::UnexpectedToken(self.token().clone()),
            context: Some(ErrorContext::Token(self.position)),
        })
    }
}

impl From<Vec<LeveledToken>> for Tokens {
    fn from(tokens: Vec<LeveledToken>) -> Self {
        Tokens {
            tokens,
            position: 0,
        }
    }
}
