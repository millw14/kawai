//! # Kawai RPC
//!
//! Native Windows RPC client for Solana with enhanced features.
//! Optimized for Windows networking, no WSL required.

pub mod client;
pub mod error;
pub mod methods;
pub mod types;

pub use client::RpcClient;
pub use error::{Error, Result};

/// RPC prelude
pub mod prelude {
    pub use crate::client::RpcClient;
    pub use crate::error::{Error, Result};
    pub use crate::types::*;
}

