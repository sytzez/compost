#[derive(Debug, Eq, PartialEq)]
pub struct CompilationError {
    pub message: String,
}

pub fn error<T>(message: String) -> CResult<T> {
    Err(CompilationError { message })
}

pub type CResult<T> = Result<T, CompilationError>;
