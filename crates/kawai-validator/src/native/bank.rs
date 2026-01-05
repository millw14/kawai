//! Bank - Processes transactions and maintains state
//!
//! The bank is the core of the validator, managing:
//! - Transaction processing
//! - Slot progression
//! - State snapshots
//! - Blockhash management

use super::*;
use super::accounts::{Account, AccountsDB};
use super::bpf::BpfRuntime;
use super::transaction::{Transaction, TransactionStatus, TransactionStatusValue, generate_blockhash};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Bank state
pub struct Bank {
    /// Current slot
    slot: Arc<RwLock<u64>>,
    /// Current epoch
    epoch: Arc<RwLock<u64>>,
    /// Slots per epoch
    slots_per_epoch: u64,
    /// Block height
    block_height: Arc<RwLock<u64>>,
    /// Current blockhash
    blockhash: Arc<RwLock<String>>,
    /// Recent blockhashes (for validity)
    recent_blockhashes: Arc<RwLock<Vec<String>>>,
    /// Accounts database
    accounts_db: Arc<AccountsDB>,
    /// BPF runtime
    bpf_runtime: Arc<RwLock<BpfRuntime>>,
    /// Transaction history
    tx_history: Arc<RwLock<HashMap<String, TransactionStatus>>>,
    /// Transaction logs
    tx_logs: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl Bank {
    /// Create a new bank
    pub fn new(accounts_db: Arc<AccountsDB>, slots_per_epoch: u64) -> Self {
        let initial_blockhash = generate_blockhash();
        
        Self {
            slot: Arc::new(RwLock::new(0)),
            epoch: Arc::new(RwLock::new(0)),
            slots_per_epoch,
            block_height: Arc::new(RwLock::new(0)),
            blockhash: Arc::new(RwLock::new(initial_blockhash.clone())),
            recent_blockhashes: Arc::new(RwLock::new(vec![initial_blockhash])),
            accounts_db,
            bpf_runtime: Arc::new(RwLock::new(BpfRuntime::new())),
            tx_history: Arc::new(RwLock::new(HashMap::new())),
            tx_logs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Get current slot
    pub async fn slot(&self) -> u64 {
        *self.slot.read().await
    }
    
    /// Get current epoch
    pub async fn epoch(&self) -> u64 {
        *self.epoch.read().await
    }
    
    /// Get block height
    pub async fn block_height(&self) -> u64 {
        *self.block_height.read().await
    }
    
    /// Get current blockhash
    pub async fn blockhash(&self) -> String {
        self.blockhash.read().await.clone()
    }
    
    /// Get recent blockhashes
    pub async fn recent_blockhashes(&self) -> Vec<String> {
        self.recent_blockhashes.read().await.clone()
    }
    
    /// Advance to next slot
    pub async fn advance_slot(&self) {
        let mut slot = self.slot.write().await;
        *slot += 1;
        
        // Check epoch
        if *slot % self.slots_per_epoch == 0 {
            let mut epoch = self.epoch.write().await;
            *epoch += 1;
        }
        
        // Update block height
        let mut block_height = self.block_height.write().await;
        *block_height += 1;
        
        // Generate new blockhash
        let new_hash = generate_blockhash();
        let mut blockhash = self.blockhash.write().await;
        *blockhash = new_hash.clone();
        
        // Keep recent blockhashes (max 150)
        let mut recent = self.recent_blockhashes.write().await;
        recent.push(new_hash);
        if recent.len() > 150 {
            recent.remove(0);
        }
    }
    
    /// Check if blockhash is valid
    pub async fn is_blockhash_valid(&self, hash: &str) -> bool {
        let recent = self.recent_blockhashes.read().await;
        recent.contains(&hash.to_string())
    }
    
    /// Load a program into the runtime
    pub async fn load_program(&self, program_id: &str, bytecode: Vec<u8>) -> Result<()> {
        let mut runtime = self.bpf_runtime.write().await;
        runtime.load_program(program_id, bytecode.clone())?;
        
        // Store program account
        self.accounts_db.store(
            program_id,
            Account::new_program(bytecode, BPF_LOADER_ID)
        ).await;
        
        Ok(())
    }
    
    /// Process a transaction
    pub async fn process_transaction(&self, tx: Transaction) -> Result<String> {
        let signature = tx.signature().to_string();
        let slot = self.slot().await;
        
        // Validate blockhash
        if !self.is_blockhash_valid(&tx.message.recent_blockhash).await {
            let status = TransactionStatus {
                slot,
                confirmations: Some(0),
                err: Some("Blockhash not found".to_string()),
                status: TransactionStatusValue::Err("Blockhash not found".to_string()),
            };
            self.tx_history.write().await.insert(signature.clone(), status);
            return Err(Error::StartFailed("Blockhash not found".to_string()));
        }
        
        // Expand instructions
        let instructions = tx.expand_instructions();
        let mut all_logs = Vec::new();
        
        // Execute each instruction
        let runtime = self.bpf_runtime.read().await;
        for instruction in instructions {
            let result = runtime.execute(&instruction, &self.accounts_db).await;
            all_logs.extend(result.logs);
            
            if !result.success {
                let error_msg = result.error.unwrap_or("Unknown error".to_string());
                let status = TransactionStatus {
                    slot,
                    confirmations: Some(0),
                    err: Some(error_msg.clone()),
                    status: TransactionStatusValue::Err(error_msg),
                };
                self.tx_history.write().await.insert(signature.clone(), status);
                self.tx_logs.write().await.insert(signature.clone(), all_logs);
                return Err(Error::StartFailed("Transaction failed".to_string()));
            }
        }
        
        // Success
        let status = TransactionStatus {
            slot,
            confirmations: None, // Finalized
            err: None,
            status: TransactionStatusValue::Ok,
        };
        
        self.tx_history.write().await.insert(signature.clone(), status);
        self.tx_logs.write().await.insert(signature.clone(), all_logs);
        
        // Persist accounts
        self.accounts_db.persist().await?;
        
        Ok(signature)
    }
    
    /// Get transaction status
    pub async fn get_transaction_status(&self, signature: &str) -> Option<TransactionStatus> {
        self.tx_history.read().await.get(signature).cloned()
    }
    
    /// Get transaction logs
    pub async fn get_transaction_logs(&self, signature: &str) -> Option<Vec<String>> {
        self.tx_logs.read().await.get(signature).cloned()
    }
    
    /// Get account
    pub async fn get_account(&self, pubkey: &str) -> Option<Account> {
        self.accounts_db.get(pubkey).await
    }
    
    /// Get balance
    pub async fn get_balance(&self, pubkey: &str) -> u64 {
        self.accounts_db.get_balance(pubkey).await
    }
    
    /// Airdrop SOL to an account
    pub async fn airdrop(&self, pubkey: &str, lamports: u64) -> Result<String> {
        // Get or create account
        let account = self.accounts_db.get(pubkey).await
            .unwrap_or_else(|| Account::system_account(0));
        
        // Add lamports
        let new_account = Account {
            lamports: account.lamports + lamports,
            ..account
        };
        
        self.accounts_db.store(pubkey, new_account).await;
        self.accounts_db.persist().await?;
        
        // Generate fake signature for the airdrop
        Ok(super::transaction::generate_signature())
    }
    
    /// Get epoch info
    pub async fn get_epoch_info(&self) -> EpochInfo {
        let slot = self.slot().await;
        let epoch = self.epoch().await;
        let block_height = self.block_height().await;
        
        EpochInfo {
            epoch,
            slot_index: slot % self.slots_per_epoch,
            slots_in_epoch: self.slots_per_epoch,
            absolute_slot: slot,
            block_height,
            transaction_count: self.tx_history.read().await.len() as u64,
        }
    }
    
    /// Get cluster time
    pub fn get_unix_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Epoch information
#[derive(Debug, Clone)]
pub struct EpochInfo {
    pub epoch: u64,
    pub slot_index: u64,
    pub slots_in_epoch: u64,
    pub absolute_slot: u64,
    pub block_height: u64,
    pub transaction_count: u64,
}

