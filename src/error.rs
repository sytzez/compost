use crate::lex::token::Token;
use crate::lex::tokenizer::get_position_of_token;

/// An error during compilation.
#[derive(Debug, Eq, PartialEq)]
pub struct CompilationError {
    pub message: ErrorMessage,
    pub context: Option<ErrorContext>,
}

impl CompilationError {
    pub fn to_string(&self, std_code: &str, all_code: &str) -> String {
        let message = String::from(&self.message);

        match &self.context {
            Some(context) => {
                let position = context.get_position_in_code(all_code);
                let (line, col) = get_line_and_col_number(std_code, all_code, position);
                format!("{} at line {} col {}", message, line, col)
            }
            None => message,
        }
    }
}

/// A compilation error message.
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorMessage {
    UnexpectedChar(String),
    UnexpectedToken(Token),
    NoSelf,
    NoResolution(&'static str, String),
    DoubleDeclaration(&'static str, String),
    NoModuleOrTrait(String),
    DuplicateClass(String),
    DuplicateStruct(String),
    ClassAndStruct(String),
    UnknownRawType(String),
}

impl From<&ErrorMessage> for String {
    fn from(message: &ErrorMessage) -> Self {
        match message {
            ErrorMessage::UnexpectedChar(char) => format!("Unexpected character {}", char),
            ErrorMessage::UnexpectedToken(token) => format!("Unexpected token {:?}", token),
            ErrorMessage::NoSelf => "Can't use 'Self' in global scope".to_string(),
            ErrorMessage::NoResolution(typ, name) => {
                format!("No resolution for {} '{}'", typ, name)
            }
            ErrorMessage::DoubleDeclaration(typ, name) => {
                format!("{} '{}' was declared twice", typ, name)
            }
            ErrorMessage::NoModuleOrTrait(name) => format!("'{}' is not a module or a trait", name),
            ErrorMessage::DuplicateClass(name) => format!(
                "Unexpected class in module '{}' which already has a class",
                name
            ),
            ErrorMessage::DuplicateStruct(name) => format!(
                "Unexpected struct in module '{}' which already has a struct",
                name
            ),
            ErrorMessage::ClassAndStruct(name) => format!(
                "Module '{}' has both a class and a struct. A module can only have one of the two.",
                name
            ),
            ErrorMessage::UnknownRawType(typ) => format!("Unknown raw type '{}'", typ),
        }
    }
}

/// Where the error occurred.
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorContext {
    Character(usize),
    Token(usize),
    Syntax,
}

impl ErrorContext {
    pub fn get_position_in_code(&self, code: &str) -> usize {
        match self {
            ErrorContext::Character(position) => *position,
            ErrorContext::Token(token_number) => get_position_of_token(code, *token_number),
            ErrorContext::Syntax => todo!(),
        }
    }
}

pub fn get_line_and_col_number(std_code: &str, all_code: &str, position: usize) -> (usize, usize) {
    let lines_in_std_code = std_code.chars().filter(|char| *char == '\n').count();
    let mut line = 1;
    let mut col = 1;
    let mut chars = all_code.chars();

    for _ in 0..position {
        let char = chars.next().unwrap();
        if char == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line - lines_in_std_code, col)
}

pub fn error<T>(message: ErrorMessage) -> CResult<T> {
    Err(CompilationError {
        message,
        context: None,
    })
}

pub type CResult<T> = Result<T, CompilationError>;
