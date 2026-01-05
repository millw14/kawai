//! Anchor error types

use thiserror::Error;

/// Result type for Anchor operations
pub type Result<T> = std::result::Result<T, Error>;

/// Anchor error type
#[derive(Error, Debug)]
pub enum Error {
    /// Project not found
    #[error("Anchor project not found at: {0}")]
    ProjectNotFound(String),

    /// Not an Anchor project
    #[error("Not an Anchor project (missing Anchor.toml)")]
    NotAnchorProject,

    /// Build error
    #[error("Build failed: {0}")]
    BuildFailed(String),

    /// Test error
    #[error("Tests failed: {0}")]
    TestFailed(String),

    /// Deploy error
    #[error("Deployment failed: {0}")]
    DeployFailed(String),

    /// IDL error
    #[error("IDL error: {0}")]
    IdlError(String),

    /// Config error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Build crate error
    #[error("Build error: {0}")]
    Build(#[from] kawai_build::Error),

    /// Validator error
    #[error("Validator error: {0}")]
    Validator(#[from] kawai_validator::Error),
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::ConfigError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::IdlError(err.to_string())
    }
}

