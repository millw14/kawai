//! Native Validator Server
//!
//! The main validator server that runs entirely on Windows.
//! No Docker, no WSL, no external dependencies.

use super::*;
use super::accounts::AccountsDB;
use super::bank::Bank;
use super::config::{NativeValidatorConfig, FundedAccount};
use super::rpc::{RpcHandler, RpcRequest, RpcResponse};
use super::runtime::SlotTimer;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use warp::Filter;

/// The Native Solana Validator
/// 
/// Runs entirely on Windows without any external dependencies.
/// Provides a complete local development environment for Solana.
pub struct NativeValidator {
    config: NativeValidatorConfig,
    bank: Arc<Bank>,
    rpc_handler: Arc<RpcHandler>,
    slot_timer: Arc<SlotTimer>,
    shutdown_tx: broadcast::Sender<()>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl NativeValidator {
    /// Create a new validator with default config
    pub fn new() -> Result<Self> {
        Self::with_config(NativeValidatorConfig::default())
    }
    
    /// Create a new validator with custom config
    pub fn with_config(config: NativeValidatorConfig) -> Result<Self> {
        // Initialize ledger directory
        if config.reset && config.ledger_dir.exists() {
            fs::remove_dir_all(&config.ledger_dir)?;
        }
        fs::create_dir_all(&config.ledger_dir)?;
        
        // Initialize accounts database
        let accounts_db = Arc::new(AccountsDB::with_ledger(&config.ledger_dir)?);
        
        // Initialize bank
        let bank = Arc::new(Bank::new(accounts_db, config.slots_per_epoch));
        
        // Initialize RPC handler
        let rpc_handler = Arc::new(RpcHandler::new(bank.clone()));
        
        // Initialize slot timer
        let slot_timer = Arc::new(SlotTimer::new(bank.clone(), config.slot_time_ms));
        
        // Shutdown channel
        let (shutdown_tx, _) = broadcast::channel(1);
        
        Ok(Self {
            config,
            bank,
            rpc_handler,
            slot_timer,
            shutdown_tx,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }
    
    /// Start the validator
    pub async fn start(&self) -> Result<()> {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║                                                              ║");
        println!("║  🌸 Kawai Native Validator - Pure Windows Solana Runtime 🌸 ║");
        println!("║                                                              ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
        
        // Initialize genesis
        self.init_genesis().await?;
        
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Start slot timer
        let slot_timer = self.slot_timer.clone();
        let shutdown_rx = self.shutdown_tx.subscribe();
        tokio::spawn(async move {
            slot_timer.start(shutdown_rx).await;
        });
        
        // Build RPC routes
        let rpc_handler = self.rpc_handler.clone();
        
        let rpc_route = warp::post()
            .and(warp::body::json())
            .and(warp::any().map(move || rpc_handler.clone()))
            .and_then(|request: RpcRequest, handler: Arc<RpcHandler>| async move {
                let response = handler.handle(request).await;
                Ok::<_, warp::Rejection>(warp::reply::json(&response))
            });
        
        let health = warp::get()
            .and(warp::path("health"))
            .map(|| warp::reply::json(&"ok"));
        
        let routes = rpc_route.or(health);
        
        let addr: SocketAddr = format!(
            "{}:{}",
            self.config.rpc_bind_address,
            self.config.rpc_port
        ).parse().unwrap();
        
        println!("🚀 Starting validator...");
        println!("   RPC URL:    {}", self.config.rpc_url());
        println!("   WS URL:     {}", self.config.ws_url());
        println!("   Ledger:     {:?}", self.config.ledger_dir);
        println!("   Slot Time:  {}ms", self.config.slot_time_ms);
        println!();
        println!("✨ Validator is ready!");
        println!("   Use with: kawai config set --url {}", self.config.rpc_url());
        println!();
        
        // Start HTTP server
        let shutdown_rx = self.shutdown_tx.subscribe();
        let (_, server) = warp::serve(routes)
            .bind_with_graceful_shutdown(addr, async move {
                let mut rx = shutdown_rx;
                let _ = rx.recv().await;
            });
        
        server.await;
        
        Ok(())
    }
    
    /// Initialize genesis state
    async fn init_genesis(&self) -> Result<()> {
        println!("📦 Initializing genesis...");
        
        // Get accounts DB through bank
        // Initialize with mint
        let faucet = "Faucet11111111111111111111111111111111111111";
        self.bank.airdrop(faucet, self.config.mint_lamports).await?;
        
        // Fund configured accounts
        for funded in &self.config.funded_accounts {
            self.bank.airdrop(&funded.pubkey, funded.lamports).await?;
            println!("   Funded {} with {} SOL", 
                &funded.pubkey[..8],
                funded.lamports / LAMPORTS_PER_SOL
            );
        }
        
        // Load preloaded programs
        for program in &self.config.preloaded_programs {
            if program.so_path.exists() {
                let bytecode = fs::read(&program.so_path)?;
                self.bank.load_program(&program.program_id, bytecode).await?;
                println!("   Loaded program {}", &program.program_id[..8]);
            }
        }
        
        println!("   Genesis initialized with {} SOL supply", 
            self.config.mint_lamports / LAMPORTS_PER_SOL
        );
        println!();
        
        Ok(())
    }
    
    /// Stop the validator
    pub async fn stop(&self) {
        println!("🛑 Stopping validator...");
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        self.slot_timer.stop();
        let _ = self.shutdown_tx.send(());
    }
    
    /// Check if validator is running
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Get RPC URL
    pub fn rpc_url(&self) -> String {
        self.config.rpc_url()
    }
    
    /// Get current slot
    pub async fn slot(&self) -> u64 {
        self.bank.slot().await
    }
    
    /// Airdrop SOL to an account
    pub async fn airdrop(&self, pubkey: &str, lamports: u64) -> Result<String> {
        self.bank.airdrop(pubkey, lamports).await
    }
}

impl Default for NativeValidator {
    fn default() -> Self {
        Self::new().expect("Failed to create default validator")
    }
}

/// Quick start function for CLI
pub async fn run_validator(config: NativeValidatorConfig) -> Result<()> {
    let validator = NativeValidator::with_config(config)?;
    
    // Handle Ctrl+C
    let validator_ref = validator.running.clone();
    let shutdown = validator.shutdown_tx.clone();
    
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("\n🌸 Shutting down gracefully...");
        validator_ref.store(false, std::sync::atomic::Ordering::SeqCst);
        let _ = shutdown.send(());
    });
    
    validator.start().await
}

