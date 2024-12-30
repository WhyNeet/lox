use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("No file name provided.")]
    MissingFilename,
}
