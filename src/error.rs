#[derive(Debug, Eq, PartialEq)]
pub struct CompilationError {
    pub message: String,
}

pub fn error<T>(message: String) -> Result<T, CompilationError> {
    Err(CompilationError { message })
}
