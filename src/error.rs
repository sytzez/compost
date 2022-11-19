#[derive(Debug, Eq, PartialEq)]
pub struct CompilationError {
    pub message: String,
    pub position: usize,
}

pub fn error<T>(message: String, position: usize) -> CResult<T> {
    Err(CompilationError { message, position })
}

pub type CResult<T> = Result<T, CompilationError>;
