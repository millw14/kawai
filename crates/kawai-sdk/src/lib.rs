//! # Kawai SDK
//!
//! Native Windows SDK for Solana development. No WSL required.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kawai_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Connect to devnet
//!     let kawai = Kawai::devnet().await?;
//!
//!     // Create a new wallet
//!     let wallet = kawai.wallet().create()?;
//!     println!("🌸 New wallet: {}", wallet.pubkey());
//!
//!     // Request airdrop
//!     kawai.airdrop(&wallet, sol!(2.0)).await?;
//!
//!     // Check balance
//!     let balance = kawai.balance(&wallet).await?;
//!     println!("💰 Balance: {} SOL", balance);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod network;
pub mod transaction;
pub mod types;

// Re-export sub-crates
pub use kawai_rpc as rpc;
pub use kawai_wallet as wallet;

/// Prelude module - import everything you need
pub mod prelude {
    pub use crate::client::Kawai;
    pub use crate::error::{Error, Result};
    pub use crate::network::Network;
    pub use crate::transaction::TransactionBuilder;
    pub use crate::types::*;
    
    // Convenient macros
    pub use crate::sol;
    pub use crate::lamports;
    
    // Re-exports from solana-sdk
    pub use solana_sdk::pubkey::Pubkey;
    pub use solana_sdk::signature::Signature;
}

/// Convert SOL to lamports
#[macro_export]
macro_rules! sol {
    ($amount:expr) => {
        (($amount as f64) * 1_000_000_000.0) as u64
    };
}

/// Lamports constant (for clarity)
#[macro_export]
macro_rules! lamports {
    ($amount:expr) => {
        $amount as u64
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol_macro() {
        assert_eq!(sol!(1.0), 1_000_000_000);
        assert_eq!(sol!(0.5), 500_000_000);
        assert_eq!(sol!(2.5), 2_500_000_000);
    }

    #[test]
    fn test_lamports_macro() {
        assert_eq!(lamports!(1000), 1000);
        assert_eq!(lamports!(1_000_000_000), 1_000_000_000);
    }
}

