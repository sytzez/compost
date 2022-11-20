use crate::error::CResult;
use crate::lex::token::{next_token, Level, Next, Token};
use crate::lex::tokens::Tokens;

pub type LeveledToken = (Token, usize);

// Turns a string of raw code into a vector of tokens with levels.
pub fn tokenize(code: &str) -> CResult<Tokens> {
    let mut position: usize = 0;
    let mut level_stack = LevelStack::new();
    let mut leveled_tokens: Vec<LeveledToken> = vec![];
    let mut is_beginning_of_line = true;

    while position <= code.len() {
        let sized_token = next_token(&code[position..])?;

        assert!(
            sized_token.1 > 0,
            "Size must be larger than 0 to prevent an infinite loop"
        );
        position += sized_token.1;

        if let Some(token) = sized_token.0 {
            is_beginning_of_line = is_beginning_of_line && token == Token::Space;

            match token {
                Token::Space => {
                    if is_beginning_of_line {
                        level_stack.indent()
                    }
                }
                Token::Down(level) => level_stack.push(level),
                Token::Up(level) => level_stack.pop(&level),
                Token::Next(next) => {
                    level_stack.next(&next);

                    if next == Next::Line {
                        is_beginning_of_line = true;
                    }
                }
                Token::Eof => leveled_tokens.push((Token::Eof, 0)),
                _ => leveled_tokens.push((token, level_stack.level())),
            }
        }
    }

    let tokens = Tokens::new(leveled_tokens);

    Ok(tokens)
}

// Utility to keep track of the depth level of our code.
struct LevelStack {
    levels: Vec<Level>,
    indentation: usize,
}

impl LevelStack {
    fn new() -> Self {
        LevelStack {
            levels: vec![],
            indentation: 0,
        }
    }

    fn push(&mut self, level: Level) {
        self.levels.push(level)
    }

    fn pop(&mut self, level: &Level) {
        if let Some(popped_level) = self.levels.pop() {
            match level {
                Level::Paren => {
                    // Keep popping until we're at an opening parenthesis
                    if popped_level != Level::Paren {
                        self.pop(level)
                    }
                }
                Level::Colon => {
                    // If the last level wasn't a colon, push it back
                    if popped_level != Level::Colon {
                        self.push(popped_level)
                    }
                }
            }
        }
    }

    fn indent(&mut self) {
        self.indentation += 1
    }

    fn next(&mut self, next: &Next) {
        match next {
            Next::Line => {
                self.levels.retain(|level| level != &Level::Colon);

                self.indentation = 0;
            }
            Next::Comma => self.pop(&Level::Colon),
        }
    }

    fn level(&self) -> usize {
        self.levels.len() + self.indentation
    }
}

#[cfg(test)]
mod test {
    use crate::lex::token::{Kw, Token};
    use crate::lex::tokenizer::tokenize;

    #[test]
    fn test_levels() {
        let code = r#"
            mod Module
                class
                    value: Int
                traits
                    Value: Int
        "#;

        let tokens = tokenize(code).unwrap();

        let expected = vec![
            (Token::Kw(Kw::Mod), 12),
            (Token::Global("Module".into()), 12),
            (Token::Kw(Kw::Class), 16),
            (Token::Local("value".into()), 20),
            (Token::Global("Int".into()), 21),
            (Token::Kw(Kw::Traits), 16),
            (Token::Global("Value".into()), 20),
            (Token::Global("Int".into()), 21),
            (Token::Eof, 0),
        ];

        assert_eq!(tokens.remaining(), &expected)
    }
}
