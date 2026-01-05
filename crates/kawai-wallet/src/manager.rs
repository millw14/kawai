//! Wallet manager for storing and managing multiple wallets

use crate::error::{Error, Result};
use crate::keypair::KawaiKeypair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Stored wallet metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Wallet name
    pub name: String,
    /// Public key
    pub pubkey: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Is this the default wallet
    pub is_default: bool,
}

/// Wallet manager for handling multiple wallets
pub struct WalletManager {
    /// Base directory for wallet storage
    base_dir: PathBuf,
    /// Loaded wallets
    wallets: HashMap<String, WalletInfo>,
    /// Default wallet name
    default_wallet: Option<String>,
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new() -> Result<Self> {
        let base_dir = Self::default_wallet_dir()?;
        Self::with_dir(base_dir)
    }

    /// Create with custom directory
    pub fn with_dir(base_dir: PathBuf) -> Result<Self> {
        // Create directory if it doesn't exist
        fs::create_dir_all(&base_dir)?;

        let mut manager = Self {
            base_dir,
            wallets: HashMap::new(),
            default_wallet: None,
        };

        // Load existing wallets
        manager.load_wallets()?;

        Ok(manager)
    }

    /// Get default wallet directory
    pub fn default_wallet_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory"
            )))?;
        Ok(config_dir.join("kawai").join("wallets"))
    }

    /// Load wallets from disk
    fn load_wallets(&mut self) -> Result<()> {
        let index_path = self.base_dir.join("wallets.json");
        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let data: WalletIndex = serde_json::from_str(&content)?;
            self.wallets = data.wallets;
            self.default_wallet = data.default_wallet;
        }
        Ok(())
    }

    /// Save wallet index to disk
    fn save_index(&self) -> Result<()> {
        let index_path = self.base_dir.join("wallets.json");
        let data = WalletIndex {
            wallets: self.wallets.clone(),
            default_wallet: self.default_wallet.clone(),
        };
        let content = serde_json::to_string_pretty(&data)?;
        fs::write(index_path, content)?;
        Ok(())
    }

    /// Create a new wallet
    pub fn create(&mut self, name: &str) -> Result<KawaiKeypair> {
        if self.wallets.contains_key(name) {
            return Err(Error::AlreadyExists(name.to_string()));
        }

        let keypair = KawaiKeypair::new().with_name(name);
        
        // Save keypair to file
        let key_path = self.base_dir.join(format!("{}.key", name));
        fs::write(&key_path, keypair.to_base58())?;

        // Add to index
        let info = WalletInfo {
            name: name.to_string(),
            pubkey: keypair.pubkey_string(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            is_default: self.wallets.is_empty(),
        };

        if info.is_default {
            self.default_wallet = Some(name.to_string());
        }

        self.wallets.insert(name.to_string(), info);
        self.save_index()?;

        Ok(keypair)
    }

    /// Import a wallet from base58 private key
    pub fn import(&mut self, name: &str, private_key: &str) -> Result<KawaiKeypair> {
        if self.wallets.contains_key(name) {
            return Err(Error::AlreadyExists(name.to_string()));
        }

        let keypair = KawaiKeypair::from_base58(private_key)?.with_name(name);

        // Save keypair to file
        let key_path = self.base_dir.join(format!("{}.key", name));
        fs::write(&key_path, keypair.to_base58())?;

        // Add to index
        let info = WalletInfo {
            name: name.to_string(),
            pubkey: keypair.pubkey_string(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            is_default: self.wallets.is_empty(),
        };

        if info.is_default {
            self.default_wallet = Some(name.to_string());
        }

        self.wallets.insert(name.to_string(), info);
        self.save_index()?;

        Ok(keypair)
    }

    /// Load a wallet by name
    pub fn load(&self, name: &str) -> Result<KawaiKeypair> {
        if !self.wallets.contains_key(name) {
            return Err(Error::NotFound(name.to_string()));
        }

        let key_path = self.base_dir.join(format!("{}.key", name));
        let content = fs::read_to_string(&key_path)?;
        KawaiKeypair::from_base58(content.trim())?.with_name(name);
        
        Ok(KawaiKeypair::from_base58(content.trim())?.with_name(name))
    }

    /// Load the default wallet
    pub fn load_default(&self) -> Result<KawaiKeypair> {
        let name = self.default_wallet
            .as_ref()
            .ok_or_else(|| Error::NotFound("No default wallet set".to_string()))?;
        self.load(name)
    }

    /// Delete a wallet
    pub fn delete(&mut self, name: &str) -> Result<()> {
        if !self.wallets.contains_key(name) {
            return Err(Error::NotFound(name.to_string()));
        }

        // Remove key file
        let key_path = self.base_dir.join(format!("{}.key", name));
        if key_path.exists() {
            fs::remove_file(&key_path)?;
        }

        // Remove from index
        self.wallets.remove(name);
        
        // Update default if needed
        if self.default_wallet.as_deref() == Some(name) {
            self.default_wallet = self.wallets.keys().next().cloned();
        }

        self.save_index()?;
        Ok(())
    }

    /// Set default wallet
    pub fn set_default(&mut self, name: &str) -> Result<()> {
        if !self.wallets.contains_key(name) {
            return Err(Error::NotFound(name.to_string()));
        }

        // Update is_default flags
        for (n, info) in self.wallets.iter_mut() {
            info.is_default = n == name;
        }

        self.default_wallet = Some(name.to_string());
        self.save_index()?;
        Ok(())
    }

    /// List all wallets
    pub fn list(&self) -> Vec<&WalletInfo> {
        self.wallets.values().collect()
    }

    /// Get wallet info
    pub fn get_info(&self, name: &str) -> Option<&WalletInfo> {
        self.wallets.get(name)
    }

    /// Check if a wallet exists
    pub fn exists(&self, name: &str) -> bool {
        self.wallets.contains_key(name)
    }

    /// Get the number of wallets
    pub fn count(&self) -> usize {
        self.wallets.len()
    }
}

impl Default for WalletManager {
    fn default() -> Self {
        Self::new().expect("Failed to create wallet manager")
    }
}

/// Serializable wallet index
#[derive(Debug, Serialize, Deserialize)]
struct WalletIndex {
    wallets: HashMap<String, WalletInfo>,
    default_wallet: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn temp_manager() -> WalletManager {
        let dir = env::temp_dir().join(format!("kawai_test_{}", rand::random::<u32>()));
        WalletManager::with_dir(dir).unwrap()
    }

    #[test]
    fn test_create_wallet() {
        let mut manager = temp_manager();
        let wallet = manager.create("test").unwrap();
        assert!(manager.exists("test"));
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_load_wallet() {
        let mut manager = temp_manager();
        let wallet1 = manager.create("test").unwrap();
        let wallet2 = manager.load("test").unwrap();
        assert_eq!(wallet1.pubkey(), wallet2.pubkey());
    }

    #[test]
    fn test_delete_wallet() {
        let mut manager = temp_manager();
        manager.create("test").unwrap();
        manager.delete("test").unwrap();
        assert!(!manager.exists("test"));
    }
}

