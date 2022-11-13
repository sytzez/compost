#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Down(Level),
    Up(Level),
    Next(Next),
    Eof,
    Kw(Kw),
    Local(String),
    Global(String),
    Op(Op),
    Lit(Lit),
    Space,
}

// Keyword
#[derive(Eq, PartialEq, Debug)]
pub enum Kw {
    Mod,
    Class,
    Struct,
    Traits,
    Defs,
    Lets,
    Zelf,
}

// Operator
#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    Dot,
    Add,
    Sub,
    Div,
    Mul,
    Eq,
    Lt,
    Gt,
}

// Literal
#[derive(Eq, PartialEq, Debug)]
pub enum Lit {
    String(String),
    Number(usize),
}

#[derive(Eq, PartialEq, Debug)]
pub enum RawType {
    String,
    Int,
    UInt,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub enum Level {
    Colon,
    Paren,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Next {
    Comma,
    Line,
}

type SizedToken = (Option<Token>, usize);

pub fn next_token(code: &str) -> SizedToken {
    let char = match code.chars().next() {
        Some(c) => c,
        None => return (Some(Token::Eof), 1),
    };

    match char {
        ' ' => (Some(Token::Space), 1),
        '#' => (None, comment_size(code)),
        '(' => (Some(Token::Down(Level::Paren)), 1),
        ')' => (Some(Token::Up(Level::Paren)), 1),
        ':' => (Some(Token::Down(Level::Colon)), 1),
        '\n' | '\r' => (Some(Token::Next(Next::Line)), 1),
        ',' => (Some(Token::Next(Next::Comma)), 1),
        '+' => (Some(Token::Op(Op::Add)), 1),
        '-' => (Some(Token::Op(Op::Sub)), 1),
        '*' => (Some(Token::Op(Op::Mul)), 1),
        '/' => (Some(Token::Op(Op::Div)), 1),
        '<' => (Some(Token::Op(Op::Lt)), 1),
        '>' => (Some(Token::Op(Op::Gt)), 1),
        '=' => (Some(Token::Op(Op::Eq)), 1),
        '.' => (Some(Token::Op(Op::Dot)), 1),
        'a'..='z' => next_local_token(code),
        'A'..='Z' | '\\' => next_global_token(code),
        '0'..='9' => next_number_token(code),
        '\'' => next_string_token(code),
        _ => panic!("Unexpected character: {}", char),
    }
}

fn comment_size(code: &str) -> usize {
    match code.find("\n") {
        Some(position) => position,
        None => code.len(),
    }
}

fn next_local_token(code: &str) -> SizedToken {
    let mut size = 0;

    for char in code.chars() {
        if !char.is_alphanumeric() {
            break;
        }
        size += 1;
    }

    let str = &code[..size];

    let token = match str {
        "mod" => Token::Kw(Kw::Mod),
        "class" => Token::Kw(Kw::Class),
        "struct" => Token::Kw(Kw::Struct),
        "traits" => Token::Kw(Kw::Traits),
        "defs" => Token::Kw(Kw::Defs),
        "lets" => Token::Kw(Kw::Lets),
        _ => Token::Local(str.to_string()),
    };

    (Some(token), size)
}

fn next_global_token(code: &str) -> SizedToken {
    let mut size = 0;

    for char in code.chars() {
        if !(char.is_alphanumeric() || char == '\\') {
            break;
        }
        size += 1;
    }

    let str = &code[..size];

    let token = match str {
        "Self" => Token::Kw(Kw::Zelf),
        _ => Token::Global(str.to_string()),
    };

    (Some(token), size)
}

fn next_number_token(code: &str) -> SizedToken {
    let mut size = 0;

    for char in code.chars() {
        if !char.is_digit(10) {
            break;
        }
        size += 1;
    }

    let number = (&code[..size]).to_string().parse().unwrap();

    (Some(Token::Lit(Lit::Number(number))), size)
}

fn next_string_token(code: &str) -> SizedToken {
    let mut size = 2;

    for char in code.chars().skip(1) {
        if char == '\'' {
            break;
        }
        size += 1;
    }

    let string = (&code[1..(size - 1)]).to_string();

    (Some(Token::Lit(Lit::String(string))), size)
}

#[cfg(test)]
mod test {
    use crate::token::{next_token, Kw, Level, Lit, Next, Op, Token};

    #[test]
    fn test_operators() {
        assert_eq!(next_token("+ 1"), (Some(Token::Op(Op::Add)), 1));
        assert_eq!(next_token("- 1"), (Some(Token::Op(Op::Sub)), 1));
        assert_eq!(next_token("/ 1"), (Some(Token::Op(Op::Div)), 1));
        assert_eq!(next_token("* 1"), (Some(Token::Op(Op::Mul)), 1));
        assert_eq!(next_token(".Add"), (Some(Token::Op(Op::Dot)), 1));
        assert_eq!(next_token("< 1"), (Some(Token::Op(Op::Lt)), 1));
        assert_eq!(next_token("> 1"), (Some(Token::Op(Op::Gt)), 1));
        assert_eq!(next_token("= 1"), (Some(Token::Op(Op::Eq)), 1));
    }

    #[test]
    fn test_comments() {
        assert_eq!(next_token("# A comment\n1 + 1"), (None, 11));
        assert_eq!(next_token("# A comment\n# Next comment\n1 + 1"), (None, 11));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(next_token(" 1 + 1"), (Some(Token::Space), 1));
        assert_eq!(next_token(""), (Some(Token::Eof), 1));
    }

    #[test]
    fn test_levels() {
        assert_eq!(next_token("\n1 + 1"), (Some(Token::Next(Next::Line)), 1));
        assert_eq!(next_token(",1 + 1"), (Some(Token::Next(Next::Comma)), 1));
        assert_eq!(next_token(": 1 + 1"), (Some(Token::Down(Level::Colon)), 1));
        assert_eq!(next_token("(1 + 1)"), (Some(Token::Down(Level::Paren)), 1));
        assert_eq!(next_token(")1 + 1"), (Some(Token::Up(Level::Paren)), 1));
    }

    #[test]
    fn test_literals() {
        assert_eq!(next_token("123 "), (Some(Token::Lit(Lit::Number(123))), 3));
        assert_eq!(
            next_token("'Bla' "),
            (Some(Token::Lit(Lit::String("Bla".into()))), 5)
        );
    }

    #[test]
    fn test_keywords() {
        assert_eq!(next_token("mod "), (Some(Token::Kw(Kw::Mod)), 3));
        assert_eq!(next_token("class "), (Some(Token::Kw(Kw::Class)), 5));
        assert_eq!(next_token("struct "), (Some(Token::Kw(Kw::Struct)), 6));
        assert_eq!(next_token("traits "), (Some(Token::Kw(Kw::Traits)), 6));
        assert_eq!(next_token("defs "), (Some(Token::Kw(Kw::Defs)), 4));
        assert_eq!(next_token("lets "), (Some(Token::Kw(Kw::Lets)), 4));
        assert_eq!(next_token("Self "), (Some(Token::Kw(Kw::Zelf)), 4));
    }

    #[test]
    fn test_local() {
        assert_eq!(
            next_token("local "),
            (Some(Token::Local("local".into())), 5)
        );
        assert_eq!(
            next_token("aLocalField "),
            (Some(Token::Local("aLocalField".into())), 11)
        );
    }

    #[test]
    fn test_global() {
        assert_eq!(
            next_token("Global "),
            (Some(Token::Global("Global".into())), 6)
        );
        assert_eq!(
            next_token("AGlobalField "),
            (Some(Token::Global("AGlobalField".into())), 12)
        );
        assert_eq!(
            next_token("\\Global "),
            (Some(Token::Global("\\Global".into())), 7)
        );
        assert_eq!(
            next_token("Module\\Global.Trait"),
            (Some(Token::Global("Module\\Global".into())), 13)
        );
    }
}
