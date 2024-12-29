use colored::Colorize;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Comptime,
    Runtime,
}

pub struct InterpreterError<E: Error> {
    source: E,
}

pub trait Error: std::error::Error {
    fn line(&self) -> Option<usize>;
    fn kind(&self) -> ErrorKind;
}

impl<E> std::fmt::Display for InterpreterError<E>
where
    E: Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}{}: {}",
            if self.source.kind() == ErrorKind::Comptime {
                "compile-time"
            } else {
                "runtime"
            }
            .red(),
            "error".red(),
            self.source
                .line()
                .map(|line| format!("[:{line}]"))
                .unwrap_or(String::new())
                .red(),
            self.source
        )
    }
}

impl<E> InterpreterError<E>
where
    E: Error,
{
    pub fn new(source: E) -> Self {
        Self { source }
    }
}
