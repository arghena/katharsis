use glob::{GlobError, PatternError};
use inquire::InquireError;
use thiserror::Error;
use tokio::task::JoinError;

/// Errors that can occur during configuration initialization or RSS feed generation.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// An I/O error occurred while reading or writing files.
    #[error("an I/O error has occurred")]
    Io(#[from] tokio::io::Error),

    /// An error occurred while displaying the interactive terminal prompt.
    #[error("error occurred while displaying the prompt")]
    Prompt(#[from] InquireError),

    /// An error occurred while deserializing the TOML configuration.
    #[error("error occurred while deserializing the TOML file")]
    TomlDeserialize(#[from] toml::de::Error),

    /// Failed to parse the glob pattern.
    #[error("failed to parse the pattern")]
    Pattern(#[from] PatternError),

    /// Unable to read a path during glob iteration.
    #[error("unable to read the path for iteration")]
    Iteration(#[from] GlobError),

    /// Failed to parse a date string.
    #[error("failed to parse the date")]
    Date(#[from] jiff::Error),

    /// A concurrent task failed to complete.
    #[error("a concurrent task failed to complete")]
    Task(#[from] JoinError),

    /// Failed to parse a CSS selector.
    #[error("failed to parse CSS selector `{0}`")]
    Selector(String),

    /// The specified configuration file does not exist.
    #[error("file does not exist")]
    FileNotFound,
}
