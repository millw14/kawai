//! Native account storage
//!
//! In-memory and file-backed account database for the validator.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Account data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Balance in lamports
    pub lamports: u64,
    /// Account data
    pub data: Vec<u8>,
    /// Owner program
    pub owner: String,
    /// Is executable (program)
    pub executable: bool,
    /// Rent epoch
    pub rent_epoch: u64,
}

impl Account {
    /// Create a new empty account
    pub fn new(lamports: u64, owner: &str) -> Self {
        Self {
            lamports,
            data: Vec::new(),
            owner: owner.to_string(),
            executable: false,
            rent_epoch: 0,
        }
    }
    
    /// Create a new program account
    pub fn new_program(data: Vec<u8>, owner: &str) -> Self {
        Self {
            lamports: 1,
            data,
            owner: owner.to_string(),
            executable: true,
            rent_epoch: 0,
        }
    }
    
    /// Create system account
    pub fn system_account(lamports: u64) -> Self {
        Self::new(lamports, SYSTEM_PROGRAM_ID)
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new(0, SYSTEM_PROGRAM_ID)
    }
}

/// Account database
pub struct AccountsDB {
    /// In-memory accounts
    accounts: Arc<RwLock<HashMap<String, Account>>>,
    /// Ledger directory for persistence
    ledger_dir: Option<std::path::PathBuf>,
}

impl AccountsDB {
    /// Create a new in-memory accounts database
    pub fn new() -> Self {
        Self {
            accounts: Arc::new(RwLock::new(HashMap::new())),
            ledger_dir: None,
        }
    }
    
    /// Create with file backing
    pub fn with_ledger(ledger_dir: impl AsRef<Path>) -> Result<Self> {
        let ledger_dir = ledger_dir.as_ref().to_path_buf();
        fs::create_dir_all(&ledger_dir)?;
        
        let accounts_file = ledger_dir.join("accounts.json");
        let accounts = if accounts_file.exists() {
            let content = fs::read_to_string(&accounts_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };
        
        Ok(Self {
            accounts: Arc::new(RwLock::new(accounts)),
            ledger_dir: Some(ledger_dir),
        })
    }
    
    /// Get an account
    pub async fn get(&self, pubkey: &str) -> Option<Account> {
        let accounts = self.accounts.read().await;
        accounts.get(pubkey).cloned()
    }
    
    /// Get multiple accounts
    pub async fn get_multiple(&self, pubkeys: &[String]) -> Vec<Option<Account>> {
        let accounts = self.accounts.read().await;
        pubkeys.iter()
            .map(|pk| accounts.get(pk).cloned())
            .collect()
    }
    
    /// Store an account
    pub async fn store(&self, pubkey: &str, account: Account) {
        let mut accounts = self.accounts.write().await;
        accounts.insert(pubkey.to_string(), account);
    }
    
    /// Store multiple accounts
    pub async fn store_multiple(&self, updates: Vec<(String, Account)>) {
        let mut accounts = self.accounts.write().await;
        for (pubkey, account) in updates {
            accounts.insert(pubkey, account);
        }
    }
    
    /// Check if account exists
    pub async fn exists(&self, pubkey: &str) -> bool {
        let accounts = self.accounts.read().await;
        accounts.contains_key(pubkey)
    }
    
    /// Get account balance
    pub async fn get_balance(&self, pubkey: &str) -> u64 {
        self.get(pubkey).await.map(|a| a.lamports).unwrap_or(0)
    }
    
    /// Transfer lamports between accounts
    pub async fn transfer(&self, from: &str, to: &str, lamports: u64) -> Result<()> {
        let mut accounts = self.accounts.write().await;
        
        // Get source account
        let from_account = accounts.get_mut(from)
            .ok_or_else(|| Error::NotFound)?;
        
        if from_account.lamports < lamports {
            return Err(Error::StartFailed("Insufficient balance".to_string()));
        }
        
        from_account.lamports -= lamports;
        
        // Get or create destination account
        let to_account = accounts.entry(to.to_string())
            .or_insert_with(|| Account::system_account(0));
        
        to_account.lamports += lamports;
        
        Ok(())
    }
    
    /// Persist to disk
    pub async fn persist(&self) -> Result<()> {
        if let Some(ledger_dir) = &self.ledger_dir {
            let accounts = self.accounts.read().await;
            let accounts_file = ledger_dir.join("accounts.json");
            let content = serde_json::to_string_pretty(&*accounts)
                .map_err(|e| Error::StartFailed(e.to_string()))?;
            fs::write(accounts_file, content)?;
        }
        Ok(())
    }
    
    /// Clear all accounts
    pub async fn clear(&self) {
        let mut accounts = self.accounts.write().await;
        accounts.clear();
    }
    
    /// Get total number of accounts
    pub async fn len(&self) -> usize {
        let accounts = self.accounts.read().await;
        accounts.len()
    }
    
    /// Initialize genesis accounts
    pub async fn init_genesis(&self, mint_lamports: u64) {
        // Create mint account with initial supply
        let mint_pubkey = "So11111111111111111111111111111111111111112"; // Wrapped SOL mint
        self.store(mint_pubkey, Account::system_account(mint_lamports)).await;
        
        // Create faucet account
        let faucet_pubkey = "FaucetXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        self.store(faucet_pubkey, Account::system_account(mint_lamports / 2)).await;
    }
}

impl Default for AccountsDB {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_accounts_db() {
        let db = AccountsDB::new();
        
        // Store account
        db.store("test", Account::system_account(1000)).await;
        
        // Get account
        let account = db.get("test").await.unwrap();
        assert_eq!(account.lamports, 1000);
    }
    
    #[tokio::test]
    async fn test_transfer() {
        let db = AccountsDB::new();
        
        db.store("from", Account::system_account(1000)).await;
        db.store("to", Account::system_account(0)).await;
        
        db.transfer("from", "to", 500).await.unwrap();
        
        assert_eq!(db.get_balance("from").await, 500);
        assert_eq!(db.get_balance("to").await, 500);
    }
}

