use std::fmt;

use error::InterpreterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeErrorKind {
    #[error("Operand must be a number.")]
    ExpectedNumberOperand,

    #[error("Attempted to divide by zero.")]
    ZeroDivision,
}

#[derive(Debug)]
pub struct RuntimeError {
    kind: RuntimeErrorKind,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl std::error::Error for RuntimeError {}
impl error::Error for RuntimeError {
    fn line(&self) -> Option<usize> {
        None
    }

    fn kind(&self) -> error::ErrorKind {
        error::ErrorKind::Runtime
    }
}

pub type RuntimeResult<T> = Result<T, InterpreterError<RuntimeError>>;
