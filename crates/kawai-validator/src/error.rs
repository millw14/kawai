//! Validator error types

use thiserror::Error;

/// Result type for validator operations
pub type Result<T> = std::result::Result<T, Error>;

/// Validator error type
#[derive(Error, Debug)]
pub enum Error {
    /// Validator not found
    #[error("Validator not found. Install with: kawai validator install")]
    NotFound,

    /// Validator already running
    #[error("Validator already running on port {0}")]
    AlreadyRunning(u16),

    /// Validator not running
    #[error("Validator not running")]
    NotRunning,

    /// Failed to start validator
    #[error("Failed to start validator: {0}")]
    StartFailed(String),

    /// Failed to stop validator
    #[error("Failed to stop validator: {0}")]
    StopFailed(String),

    /// Docker not available
    #[error("Docker not available. Install Docker Desktop or use cloud backend")]
    DockerNotAvailable,

    /// WSL not available
    #[error("WSL not available")]
    WslNotAvailable,

    /// No backend available
    #[error("No validator backend available. Install Docker Desktop or enable WSL2")]
    NoBackendAvailable,

    /// Port already in use
    #[error("Port {0} already in use")]
    PortInUse(u16),

    /// Download failed
    #[error("Download failed: {0}")]
    DownloadFailed(String),

    /// Installation failed
    #[error("Installation failed: {0}")]
    InstallFailed(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Timeout
    #[error("Operation timed out after {0} seconds")]
    Timeout(u64),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(err.to_string())
    }
}

