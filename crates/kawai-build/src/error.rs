//! Build error types

use thiserror::Error;

/// Result type for build operations
pub type Result<T> = std::result::Result<T, Error>;

/// Build error type
#[derive(Error, Debug)]
pub enum Error {
    /// Project not found
    #[error("Project not found at: {0}")]
    ProjectNotFound(String),

    /// Invalid project
    #[error("Invalid project: {0}")]
    InvalidProject(String),

    /// Cargo.toml not found
    #[error("Cargo.toml not found")]
    CargoTomlNotFound,

    /// Build failed
    #[error("Build failed: {0}")]
    BuildFailed(String),

    /// Compilation error
    #[error("Compilation error:\n{0}")]
    CompilationError(String),

    /// Toolchain not found
    #[error("Toolchain not found. Install with: kawai build install-toolchain")]
    ToolchainNotFound,

    /// Docker not available
    #[error("Docker not available for compilation")]
    DockerNotAvailable,

    /// Cloud service error
    #[error("Cloud compilation service error: {0}")]
    CloudError(String),

    /// Upload failed
    #[error("Failed to upload project: {0}")]
    UploadFailed(String),

    /// Download failed
    #[error("Failed to download artifact: {0}")]
    DownloadFailed(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Timeout
    #[error("Build timed out after {0} seconds")]
    Timeout(u64),

    /// Unsupported platform
    #[error("Unsupported platform for native compilation")]
    UnsupportedPlatform,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::InvalidProject(err.to_string())
    }
}

