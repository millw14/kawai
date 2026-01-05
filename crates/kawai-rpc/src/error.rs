//! RPC error types

use thiserror::Error;

/// Result type for RPC operations
pub type Result<T> = std::result::Result<T, Error>;

/// RPC error type
#[derive(Error, Debug)]
pub enum Error {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Request error
    #[error("Request error: {0}")]
    Request(String),

    /// Response error
    #[error("Response error: {0}")]
    Response(String),

    /// Timeout error
    #[error("Request timed out after {0} seconds")]
    Timeout(u64),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),

    /// Rate limited
    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),

    /// Node unhealthy
    #[error("Node unhealthy: {0}")]
    NodeUnhealthy(String),

    /// Invalid response
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl From<solana_client::client_error::ClientError> for Error {
    fn from(err: solana_client::client_error::ClientError) -> Self {
        Error::Request(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout(30)
        } else if err.is_connect() {
            Error::Connection(err.to_string())
        } else {
            Error::Request(err.to_string())
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Parse(err.to_string())
    }
}

