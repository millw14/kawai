//! # Kawai Validator
//!
//! Native Windows local Solana test validator management.
//! Provides multiple backends for running validators on Windows:
//!
//! 1. **Docker Backend** - Uses Docker Desktop (most compatible)
//! 2. **WSL Backend** - Uses WSL2 if available (transparent to user)
//! 3. **Cloud Backend** - Connects to remote test validators
//! 4. **Native Backend** - Future: fully native Windows validator
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kawai_validator::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Auto-detect best backend
//!     let validator = Validator::auto().await?;
//!     
//!     // Start validator
//!     validator.start().await?;
//!     
//!     // Get RPC URL
//!     println!("RPC: {}", validator.rpc_url());
//!     
//!     // Stop when done
//!     validator.stop().await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod backend;
pub mod config;
pub mod error;
pub mod manager;
pub mod process;

pub use error::{Error, Result};
pub use manager::Validator;
pub use config::ValidatorConfig;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::backend::Backend;
    pub use crate::config::ValidatorConfig;
    pub use crate::error::{Error, Result};
    pub use crate::manager::Validator;
}

/// Default RPC port for local validator
pub const DEFAULT_RPC_PORT: u16 = 8899;

/// Default WebSocket port
pub const DEFAULT_WS_PORT: u16 = 8900;

/// Default faucet port
pub const DEFAULT_FAUCET_PORT: u16 = 9900;

