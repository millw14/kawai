//! Native validator configuration

use super::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the native Windows validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeValidatorConfig {
    /// RPC bind address
    pub rpc_bind_address: String,
    
    /// RPC port
    pub rpc_port: u16,
    
    /// WebSocket port
    pub ws_port: u16,
    
    /// Faucet port
    pub faucet_port: u16,
    
    /// Ledger directory
    pub ledger_dir: PathBuf,
    
    /// Reset ledger on start
    pub reset: bool,
    
    /// Slots per epoch
    pub slots_per_epoch: u64,
    
    /// Ticks per slot
    pub ticks_per_slot: u64,
    
    /// Slot time in milliseconds
    pub slot_time_ms: u64,
    
    /// Initial mint amount (for faucet)
    pub mint_lamports: u64,
    
    /// Max transaction size
    pub max_transaction_size: usize,
    
    /// Enable transaction logging
    pub log_transactions: bool,
    
    /// Programs to preload
    pub preloaded_programs: Vec<PreloadedProgram>,
    
    /// Accounts to fund on startup
    pub funded_accounts: Vec<FundedAccount>,
}

impl Default for NativeValidatorConfig {
    fn default() -> Self {
        Self {
            rpc_bind_address: "127.0.0.1".to_string(),
            rpc_port: 8899,
            ws_port: 8900,
            faucet_port: 9900,
            ledger_dir: dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("kawai")
                .join("ledger"),
            reset: true,
            slots_per_epoch: DEFAULT_SLOTS_PER_EPOCH,
            ticks_per_slot: DEFAULT_TICKS_PER_SLOT,
            slot_time_ms: 400, // 400ms per slot (fast for testing)
            mint_lamports: 500_000_000 * LAMPORTS_PER_SOL, // 500M SOL
            max_transaction_size: 1232,
            log_transactions: true,
            preloaded_programs: Vec::new(),
            funded_accounts: Vec::new(),
        }
    }
}

impl NativeValidatorConfig {
    /// Create a new config with defaults
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set RPC port
    pub fn rpc_port(mut self, port: u16) -> Self {
        self.rpc_port = port;
        self
    }
    
    /// Set ledger directory
    pub fn ledger_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.ledger_dir = path.into();
        self
    }
    
    /// Enable reset on start
    pub fn reset(mut self, reset: bool) -> Self {
        self.reset = reset;
        self
    }
    
    /// Set slot time
    pub fn slot_time_ms(mut self, ms: u64) -> Self {
        self.slot_time_ms = ms;
        self
    }
    
    /// Add a preloaded program
    pub fn preload_program(mut self, program: PreloadedProgram) -> Self {
        self.preloaded_programs.push(program);
        self
    }
    
    /// Add a funded account
    pub fn fund_account(mut self, account: FundedAccount) -> Self {
        self.funded_accounts.push(account);
        self
    }
    
    /// Get RPC URL
    pub fn rpc_url(&self) -> String {
        format!("http://{}:{}", self.rpc_bind_address, self.rpc_port)
    }
    
    /// Get WebSocket URL
    pub fn ws_url(&self) -> String {
        format!("ws://{}:{}", self.rpc_bind_address, self.ws_port)
    }
}

/// Program to preload on validator start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreloadedProgram {
    /// Program ID
    pub program_id: String,
    /// Path to .so file
    pub so_path: PathBuf,
    /// Is upgradeable
    pub upgradeable: bool,
}

/// Account to fund on validator start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundedAccount {
    /// Account pubkey
    pub pubkey: String,
    /// Amount in lamports
    pub lamports: u64,
}

