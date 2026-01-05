//! Common types for Kawai SDK

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

/// Account balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Balance in lamports
    pub lamports: u64,
    /// Balance in SOL
    pub sol: f64,
}

impl Balance {
    /// Create from lamports
    pub fn from_lamports(lamports: u64) -> Self {
        Self {
            lamports,
            sol: lamports as f64 / 1_000_000_000.0,
        }
    }

    /// Create from SOL
    pub fn from_sol(sol: f64) -> Self {
        Self {
            lamports: (sol * 1_000_000_000.0) as u64,
            sol,
        }
    }
}

impl std::fmt::Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.9} SOL", self.sol)
    }
}

/// Transaction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    /// Transaction signature
    pub signature: String,
    /// Block slot
    pub slot: Option<u64>,
    /// Confirmation status
    pub status: TransactionStatus,
}

/// Transaction confirmation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction submitted but not confirmed
    Pending,
    /// Transaction confirmed
    Confirmed,
    /// Transaction finalized
    Finalized,
    /// Transaction failed
    Failed,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "⏳ Pending"),
            TransactionStatus::Confirmed => write!(f, "✅ Confirmed"),
            TransactionStatus::Finalized => write!(f, "🔒 Finalized"),
            TransactionStatus::Failed => write!(f, "❌ Failed"),
        }
    }
}

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account public key
    pub pubkey: String,
    /// Account balance
    pub balance: Balance,
    /// Is executable (program)
    pub executable: bool,
    /// Owner program
    pub owner: String,
    /// Rent epoch
    pub rent_epoch: u64,
}

/// Transfer parameters
#[derive(Debug, Clone)]
pub struct TransferParams {
    /// Source pubkey
    pub from: Pubkey,
    /// Destination pubkey
    pub to: Pubkey,
    /// Amount in lamports
    pub amount: u64,
    /// Optional memo
    pub memo: Option<String>,
}

/// Airdrop result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropResult {
    /// Airdrop signature
    pub signature: String,
    /// Amount received in lamports
    pub amount: u64,
}

impl std::fmt::Display for AirdropResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "🎁 Airdrop: {} SOL ({})",
            self.amount as f64 / 1_000_000_000.0,
            &self.signature[..8]
        )
    }
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Current slot
    pub slot: u64,
    /// Current epoch
    pub epoch: u64,
    /// Transactions per second
    pub tps: Option<f64>,
    /// Block height
    pub block_height: u64,
}

