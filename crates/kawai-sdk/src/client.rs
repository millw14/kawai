//! Main Kawai client

use crate::error::{Error, Result};
use crate::network::Network;
use crate::types::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use std::sync::Arc;

/// Main Kawai client for interacting with Solana
pub struct Kawai {
    /// RPC client
    rpc: Arc<RpcClient>,
    /// Network configuration
    network: Network,
    /// Custom RPC URL (if any)
    rpc_url: String,
}

impl Kawai {
    /// Create a new Kawai client with custom RPC URL
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc: Arc::new(RpcClient::new(rpc_url.to_string())),
            network: Network::Custom,
            rpc_url: rpc_url.to_string(),
        }
    }

    /// Connect to devnet
    pub async fn devnet() -> Result<Self> {
        Self::connect(Network::Devnet).await
    }

    /// Connect to mainnet
    pub async fn mainnet() -> Result<Self> {
        Self::connect(Network::Mainnet).await
    }

    /// Connect to testnet
    pub async fn testnet() -> Result<Self> {
        Self::connect(Network::Testnet).await
    }

    /// Connect to localhost
    pub async fn localhost() -> Result<Self> {
        Self::connect(Network::Localhost).await
    }

    /// Connect to a specific network
    pub async fn connect(network: Network) -> Result<Self> {
        let rpc_url = network.rpc_url().to_string();
        let rpc = RpcClient::new(rpc_url.clone());

        // Verify connection
        rpc.get_health().map_err(|e| Error::Rpc(e.to_string()))?;

        Ok(Self {
            rpc: Arc::new(rpc),
            network,
            rpc_url,
        })
    }

    /// Get current network
    pub fn network(&self) -> Network {
        self.network
    }

    /// Get RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// Get account balance
    pub async fn balance(&self, pubkey: &Pubkey) -> Result<Balance> {
        let lamports = self
            .rpc
            .get_balance(pubkey)
            .map_err(|e| Error::Rpc(e.to_string()))?;
        Ok(Balance::from_lamports(lamports))
    }

    /// Get account balance by string pubkey
    pub async fn balance_str(&self, pubkey: &str) -> Result<Balance> {
        let pk = Pubkey::from_str(pubkey)?;
        self.balance(&pk).await
    }

    /// Request airdrop (devnet/testnet only)
    pub async fn airdrop(&self, pubkey: &Pubkey, lamports: u64) -> Result<AirdropResult> {
        if !self.network.is_dev() {
            return Err(Error::Network(
                "Airdrop only available on devnet, testnet, or localhost".to_string(),
            ));
        }

        let signature = self
            .rpc
            .request_airdrop(pubkey, lamports)
            .map_err(|e| Error::Rpc(e.to_string()))?;

        // Wait for confirmation
        self.rpc
            .confirm_transaction(&signature)
            .map_err(|e| Error::Rpc(e.to_string()))?;

        Ok(AirdropResult {
            signature: signature.to_string(),
            amount: lamports,
        })
    }

    /// Transfer SOL
    pub async fn transfer(
        &self,
        from: &Keypair,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<TransactionResult> {
        // Check balance
        let balance = self.balance(&from.pubkey()).await?;
        if balance.lamports < lamports {
            return Err(Error::InsufficientBalance {
                have: balance.lamports,
                need: lamports,
            });
        }

        // Create instruction
        let instruction = system_instruction::transfer(&from.pubkey(), to, lamports);

        // Get recent blockhash
        let blockhash = self
            .rpc
            .get_latest_blockhash()
            .map_err(|e| Error::Rpc(e.to_string()))?;

        // Create and sign transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&from.pubkey()),
            &[from],
            blockhash,
        );

        // Send transaction
        let signature = self
            .rpc
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| Error::Transaction(e.to_string()))?;

        Ok(TransactionResult {
            signature: signature.to_string(),
            slot: None,
            status: TransactionStatus::Confirmed,
        })
    }

    /// Get current slot
    pub async fn slot(&self) -> Result<u64> {
        self.rpc
            .get_slot()
            .map_err(|e| Error::Rpc(e.to_string()))
    }

    /// Get network stats
    pub async fn stats(&self) -> Result<NetworkStats> {
        let slot = self.slot().await?;
        let epoch_info = self
            .rpc
            .get_epoch_info()
            .map_err(|e| Error::Rpc(e.to_string()))?;

        Ok(NetworkStats {
            slot,
            epoch: epoch_info.epoch,
            tps: None, // Would need performance samples
            block_height: epoch_info.block_height,
        })
    }

    /// Get account info
    pub async fn account(&self, pubkey: &Pubkey) -> Result<AccountInfo> {
        let account = self
            .rpc
            .get_account(pubkey)
            .map_err(|e| Error::Rpc(e.to_string()))?;

        Ok(AccountInfo {
            pubkey: pubkey.to_string(),
            balance: Balance::from_lamports(account.lamports),
            executable: account.executable,
            owner: account.owner.to_string(),
            rent_epoch: account.rent_epoch,
        })
    }
}

impl std::fmt::Debug for Kawai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Kawai")
            .field("network", &self.network)
            .field("rpc_url", &self.rpc_url)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires network
    async fn test_connect_devnet() {
        let kawai = Kawai::devnet().await.unwrap();
        assert_eq!(kawai.network(), Network::Devnet);
    }
}

