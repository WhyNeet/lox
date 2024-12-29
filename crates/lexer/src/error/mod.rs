use std::fmt;

use error::InterpreterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScannerErrorKind {
    #[error("Unterminated block-style comment.")]
    UnterminatedBlockComment,

    #[error("Unexpected character: `{0}`.")]
    UnexpectedCharacter(char),

    #[error("Unterminated string.")]
    UnterminatedString,
}

#[derive(Debug)]
pub struct ScannerError {
    kind: ScannerErrorKind,
    line: usize,
}

impl error::Error for ScannerError {
    fn line(&self) -> Option<usize> {
        Some(self.line)
    }
}

impl std::error::Error for ScannerError {}
impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl ScannerError {
    pub fn new(kind: ScannerErrorKind, line: usize) -> Self {
        Self { kind, line }
    }
}

pub type ScannerResult<T> = Result<T, InterpreterError<ScannerError>>;
