//! Wallet error types

use thiserror::Error;

/// Result type for wallet operations
pub type Result<T> = std::result::Result<T, Error>;

/// Wallet error type
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid keypair
    #[error("Invalid keypair: {0}")]
    InvalidKeypair(String),

    /// Invalid mnemonic
    #[error("Invalid mnemonic: {0}")]
    InvalidMnemonic(String),

    /// Invalid private key
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// Wallet not found
    #[error("Wallet not found: {0}")]
    NotFound(String),

    /// Wallet already exists
    #[error("Wallet already exists: {0}")]
    AlreadyExists(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Encryption error
    #[error("Encryption error: {0}")]
    Encryption(String),

    /// Decryption error
    #[error("Decryption error: {0}")]
    Decryption(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

