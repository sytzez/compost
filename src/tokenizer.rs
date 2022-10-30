use crate::token::{Level, Next, next_token, Token};

pub type LeveledToken = (Token, usize);

fn tokenize(code: &str) -> Vec<LeveledToken> {
    let mut position: usize = 0;
    let mut level_stack: Vec<Level> = vec![];
    let mut indentation = 0;
    let mut leveled_tokens: Vec<LeveledToken> = vec![];

    while position <= code.len() {
        let sized_token = next_token(&code[position..]);
        position += sized_token.1;

        if let Some(token) = sized_token.0 {
            match token {
                Token::Down(level) => level_stack.push(level),
                Token::Up(level) => {

                },
                Token::Next(Next::NewLine) => {
                    leveled_tokens.push((token, level_stack.len()))
                }
                _ => leveled_tokens.push((token, level_stack.len())),
            }
        }
    }

    leveled_tokens
}

#[cfg(test)]
mod test {
    use crate::token::{Kw, Token};
    use crate::tokenizer::tokenize;

    #[test]
    fn test_levels() {
        let code = r#"
            mod Module
                class
                    value: Int
                traits
                    Value: Int
                    Trait: (in: Self) -> Self
                defs
                    Value: value
                    Trait: Module(in.Value + value)
        "#;

        let leveled_tokens = tokenize(code);

        let expected = vec![
            (Token::Kw(Kw::Mod), 0),
            (Token::Global("Module".into()), 0),
            (Token::Kw(Kw::Class), 1),
            (Token::Local("value".into()), 2),
            (Token::Global("Int".into()), 3),
            (Token::Kw(Kw::Traits), 1),
            (Token::Global("Value".into()), 2),
            (Token::Global("Int".into()), 3),
            (Token::Global("Trait".into()), 2),
            (Token::Global("Trait".into()), 4),
        ];

        assert_eq!(leveled_tokens, expected)
    }
}
