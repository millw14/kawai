//! # Kawai Validator
//!
//! **Native Windows Solana Test Validator**
//! 
//! No Docker. No WSL. No Linux. Just Windows.
//!
//! This is a pure Rust implementation of a Solana test validator
//! that runs natively on Windows. It provides:
//!
//! - Full JSON-RPC API compatibility with standard Solana tools
//! - Built-in BPF program runtime
//! - Account storage with persistence
//! - Slot progression simulation
//! - Airdrop support for testing
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kawai_validator::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create and start native validator
//!     let config = NativeValidatorConfig::default()
//!         .rpc_port(8899)
//!         .slot_time_ms(400);
//!     
//!     run_validator(config).await
//! }
//! ```

// Core modules
pub mod error;
pub mod config;

// Native validator - Pure Windows implementation
pub mod native;

// Legacy compatibility modules (for fallback)
pub mod backend;
pub mod manager;
pub mod process;

// Re-exports
pub use error::{Error, Result};
pub use config::ValidatorConfig;
pub use native::{
    NativeValidator,
    NativeValidatorConfig,
    server::run_validator,
    LAMPORTS_PER_SOL,
};

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::native::{
        NativeValidator,
        NativeValidatorConfig,
        server::run_validator,
        LAMPORTS_PER_SOL,
    };
    // Legacy
    pub use crate::backend::Backend;
    pub use crate::config::ValidatorConfig;
    pub use crate::manager::Validator;
}

/// Default RPC port for local validator
pub const DEFAULT_RPC_PORT: u16 = 8899;

/// Default WebSocket port
pub const DEFAULT_WS_PORT: u16 = 8900;

/// Default faucet port
pub const DEFAULT_FAUCET_PORT: u16 = 9900;
