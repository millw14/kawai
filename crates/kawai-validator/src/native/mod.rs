//! Native Windows Solana Validator
//!
//! A pure Rust implementation of a local Solana test validator
//! that runs natively on Windows without Docker, WSL, or any external tools.
//!
//! This is a simplified validator optimized for local development and testing.

pub mod accounts;
pub mod bank;
pub mod bpf;
pub mod config;
pub mod rpc;
pub mod runtime;
pub mod server;
pub mod transaction;

use crate::error::{Error, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use config::NativeValidatorConfig;
pub use server::NativeValidator;

/// Default slots per epoch for fast testing
pub const DEFAULT_SLOTS_PER_EPOCH: u64 = 32;

/// Default ticks per slot
pub const DEFAULT_TICKS_PER_SLOT: u64 = 64;

/// Lamports per SOL
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// System program ID
pub const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

/// Native loader program ID  
pub const NATIVE_LOADER_ID: &str = "NativeLoader1111111111111111111111111111111";

/// BPF loader program ID
pub const BPF_LOADER_ID: &str = "BPFLoader2111111111111111111111111111111111";

/// SPL Token program ID
pub const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

