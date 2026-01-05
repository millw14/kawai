//! # Kawai Wallet
//!
//! Native Windows wallet management for Solana.
//! Create, import, export, and manage Solana wallets securely.

pub mod error;
pub mod keypair;
pub mod manager;
pub mod mnemonic;

pub use error::{Error, Result};
pub use keypair::KawaiKeypair;
pub use manager::WalletManager;
pub use mnemonic::Mnemonic;

/// Wallet prelude
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::keypair::KawaiKeypair;
    pub use crate::manager::WalletManager;
    pub use crate::mnemonic::Mnemonic;
}

