use std::fmt;

use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScannerErrorType {
    #[error("Unterminated block-style comment.")]
    UnterminatedBlockComment,

    #[error("Unexpected character: `{0}`.")]
    UnexpectedCharacter(char),

    #[error("Unterminated string.")]
    UnterminatedString,
}

#[derive(Debug)]
pub struct ScannerError {
    error: ScannerErrorType,
    line: Option<usize>,
}

impl std::error::Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}{}: {}",
            "error".red(),
            self.line
                .map(|line| format!("[:{line}]"))
                .unwrap_or(String::new())
                .red(),
            self.error
        )
    }
}

impl ScannerError {
    pub fn new(error: ScannerErrorType, line: Option<usize>) -> Self {
        Self { error, line }
    }
}

pub type ScannerResult<T> = Result<T, ScannerError>;
