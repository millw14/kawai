//! # Kawai Build
//!
//! Solana program compiler for Windows. Supports multiple compilation backends:
//!
//! 1. **Docker Backend** - Uses Docker to run Linux build tools
//! 2. **Cloud Backend** - Remote compilation service
//! 3. **WSL Backend** - Uses WSL2 for compilation
//! 4. **Local Backend** - Future: native Windows LLVM/BPF toolchain
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kawai_build::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Build a Solana program
//!     let result = Builder::new()
//!         .project_dir("./my-program")
//!         .build()
//!         .await?;
//!
//!     println!("Built: {}", result.so_path.display());
//!     println!("Program ID: {}", result.program_id);
//!
//!     Ok(())
//! }
//! ```

pub mod backend;
pub mod builder;
pub mod config;
pub mod error;
pub mod project;
pub mod toolchain;

pub use builder::Builder;
pub use config::BuildConfig;
pub use error::{Error, Result};
pub use project::Project;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::backend::BuildBackend;
    pub use crate::builder::Builder;
    pub use crate::config::BuildConfig;
    pub use crate::error::{Error, Result};
    pub use crate::project::Project;
}

