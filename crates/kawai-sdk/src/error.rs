//! Error types for Kawai SDK

use thiserror::Error;

/// Result type alias for Kawai operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for Kawai SDK
#[derive(Error, Debug)]
pub enum Error {
    /// RPC connection error
    #[error("RPC error: {0}")]
    Rpc(String),

    /// Wallet error
    #[error("Wallet error: {0}")]
    Wallet(String),

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Insufficient balance
    #[error("Insufficient balance: have {have} lamports, need {need} lamports")]
    InsufficientBalance { have: u64, need: u64 },

    /// Invalid pubkey
    #[error("Invalid pubkey: {0}")]
    InvalidPubkey(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Timeout
    #[error("Operation timed out after {0} seconds")]
    Timeout(u64),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<solana_client::client_error::ClientError> for Error {
    fn from(err: solana_client::client_error::ClientError) -> Self {
        Error::Rpc(err.to_string())
    }
}

impl From<solana_sdk::pubkey::ParsePubkeyError> for Error {
    fn from(err: solana_sdk::pubkey::ParsePubkeyError) -> Self {
        Error::InvalidPubkey(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

