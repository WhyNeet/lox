use std::fmt;

use error::InterpreterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserErrorKind {
    #[error("Expected `{0}`.")]
    TokenExpected(char),

    #[error("Expected expression.")]
    ExpressionExprected,

    #[error("Missing the left-hand expression operand.")]
    MissingLeftHandOperand,

    #[error("Expected identifier.")]
    IdentifierExpected,
}

#[derive(Debug)]
pub struct ParserError {
    kind: ParserErrorKind,
}

impl error::Error for ParserError {
    fn line(&self) -> Option<usize> {
        None
    }

    fn kind(&self) -> error::ErrorKind {
        error::ErrorKind::Comptime
    }
}

impl std::error::Error for ParserError {}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl ParserError {
    pub fn new(kind: ParserErrorKind) -> Self {
        Self { kind }
    }
}

pub type ParserResult<T> = Result<T, InterpreterError<ParserError>>;
