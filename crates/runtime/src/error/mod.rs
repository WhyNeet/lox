use std::fmt;

use error::InterpreterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeErrorKind {
    #[error("Operand must be a number.")]
    ExpectedNumberOperand,

    #[error("Attempted to divide by zero.")]
    ZeroDivision,

    #[error("Variable with identifier `{0}` is already defined.")]
    VariableAlreadyDefined(String),

    #[error("Variable with identifier `{0}` is not defined.")]
    VariableNotDefined(String),

    #[error("`continue` statement used outside of a loop.")]
    ContinueNotWithinLoop,

    #[error("`break` statement used outside of a loop.")]
    BreakNotWithinLoop,

    #[error("Expression is not callable.")]
    ExpressionNotCallable,

    #[error("Invalid arguments count ({0}, expected {1}).")]
    InvalidArgumentCount(usize, usize),
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
