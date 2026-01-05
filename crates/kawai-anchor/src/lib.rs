//! # Kawai Anchor
//!
//! Anchor framework integration for Windows. Build, test, and deploy
//! Anchor programs without WSL.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kawai_anchor::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize new Anchor project
//!     Anchor::init("my-program").await?;
//!
//!     // Build the project
//!     let result = Anchor::build(".").await?;
//!     println!("Built: {}", result.program_id);
//!
//!     // Deploy to local validator
//!     let deployed = Anchor::deploy(".", "devnet").await?;
//!     println!("Deployed: {}", deployed.program_id);
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod idl;
pub mod project;
pub mod testing;

pub use config::AnchorConfig;
pub use error::{Error, Result};
pub use project::Anchor;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::config::AnchorConfig;
    pub use crate::error::{Error, Result};
    pub use crate::idl::Idl;
    pub use crate::project::Anchor;
}

