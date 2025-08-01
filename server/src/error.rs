use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Invalid configuration file")]
    InvalidConfig,

    #[error("Invalid TOML")]
    InvalidTOML,

    #[error("Failed to create file: {0}")]
    FailedToCreateFile(String),

    #[error("Failed to write to file: {0}")]
    FailedToWriteToFile(String),
}
