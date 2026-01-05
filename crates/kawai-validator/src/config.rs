//! Validator configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfig {
    /// RPC port
    pub rpc_port: u16,
    
    /// WebSocket port
    pub ws_port: u16,
    
    /// Faucet port
    pub faucet_port: u16,
    
    /// Ledger directory
    pub ledger_dir: Option<PathBuf>,
    
    /// Reset ledger on start
    pub reset: bool,
    
    /// Enable logging
    pub log_enabled: bool,
    
    /// Log file path
    pub log_file: Option<PathBuf>,
    
    /// Slots per epoch (for faster testing)
    pub slots_per_epoch: Option<u64>,
    
    /// Programs to deploy on start
    pub programs: Vec<ProgramConfig>,
    
    /// Accounts to clone from mainnet/devnet
    pub clone_accounts: Vec<CloneAccountConfig>,
    
    /// Backend preference
    pub backend: BackendPreference,
    
    /// Docker image (if using Docker backend)
    pub docker_image: String,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            rpc_port: crate::DEFAULT_RPC_PORT,
            ws_port: crate::DEFAULT_WS_PORT,
            faucet_port: crate::DEFAULT_FAUCET_PORT,
            ledger_dir: None,
            reset: true,
            log_enabled: true,
            log_file: None,
            slots_per_epoch: Some(32), // Fast epochs for testing
            programs: Vec::new(),
            clone_accounts: Vec::new(),
            backend: BackendPreference::Auto,
            docker_image: "solanalabs/solana:v1.18.0".to_string(),
        }
    }
}

impl ValidatorConfig {
    /// Create a new configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set RPC port
    pub fn rpc_port(mut self, port: u16) -> Self {
        self.rpc_port = port;
        self
    }

    /// Set WebSocket port
    pub fn ws_port(mut self, port: u16) -> Self {
        self.ws_port = port;
        self
    }

    /// Set ledger directory
    pub fn ledger_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.ledger_dir = Some(path.into());
        self
    }

    /// Enable ledger reset on start
    pub fn reset(mut self, reset: bool) -> Self {
        self.reset = reset;
        self
    }

    /// Add a program to deploy
    pub fn program(mut self, program: ProgramConfig) -> Self {
        self.programs.push(program);
        self
    }

    /// Add an account to clone
    pub fn clone_account(mut self, account: CloneAccountConfig) -> Self {
        self.clone_accounts.push(account);
        self
    }

    /// Set backend preference
    pub fn backend(mut self, backend: BackendPreference) -> Self {
        self.backend = backend;
        self
    }

    /// Get the RPC URL
    pub fn rpc_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.rpc_port)
    }

    /// Get the WebSocket URL
    pub fn ws_url(&self) -> String {
        format!("ws://127.0.0.1:{}", self.ws_port)
    }

    /// Get the default ledger directory
    pub fn default_ledger_dir() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kawai")
            .join("test-ledger")
    }
}

/// Program configuration for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramConfig {
    /// Program ID (pubkey)
    pub program_id: String,
    
    /// Path to .so file
    pub so_path: PathBuf,
    
    /// Upgradeable
    pub upgradeable: bool,
}

impl ProgramConfig {
    /// Create a new program config
    pub fn new(program_id: &str, so_path: impl Into<PathBuf>) -> Self {
        Self {
            program_id: program_id.to_string(),
            so_path: so_path.into(),
            upgradeable: false,
        }
    }
}

/// Account clone configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneAccountConfig {
    /// Account pubkey to clone
    pub pubkey: String,
    
    /// Source cluster (mainnet-beta, devnet, etc.)
    pub source: String,
}

impl CloneAccountConfig {
    /// Clone from mainnet
    pub fn from_mainnet(pubkey: &str) -> Self {
        Self {
            pubkey: pubkey.to_string(),
            source: "mainnet-beta".to_string(),
        }
    }

    /// Clone from devnet
    pub fn from_devnet(pubkey: &str) -> Self {
        Self {
            pubkey: pubkey.to_string(),
            source: "devnet".to_string(),
        }
    }
}

/// Backend preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BackendPreference {
    /// Auto-detect best available backend
    #[default]
    Auto,
    /// Use Docker
    Docker,
    /// Use WSL2
    Wsl,
    /// Use cloud/remote validator
    Cloud,
    /// Native Windows (future)
    Native,
}

