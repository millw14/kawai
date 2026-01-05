//! Kawai RPC client

use crate::error::{Error, Result};
use crate::types::*;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use std::time::Duration;

/// Native Windows RPC client for Solana
pub struct RpcClient {
    url: String,
    http: reqwest::Client,
    timeout: Duration,
}

impl RpcClient {
    /// Create a new RPC client
    pub fn new(url: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            url: url.to_string(),
            http,
            timeout: Duration::from_secs(30),
        }
    }

    /// Create with custom timeout
    pub fn with_timeout(url: &str, timeout: Duration) -> Self {
        let http = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            url: url.to_string(),
            http,
            timeout,
        }
    }

    /// Get the RPC URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Send a raw RPC request
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let request = RpcRequest::new(method, params);

        let response = self
            .http
            .post(&self.url)
            .json(&request)
            .send()
            .await?;

        let rpc_response: RpcResponse<T> = response.json().await?;

        if let Some(error) = rpc_response.error {
            return Err(Error::Response(format!(
                "[{}] {}",
                error.code, error.message
            )));
        }

        rpc_response
            .result
            .ok_or_else(|| Error::InvalidResponse("No result in response".to_string()))
    }

    /// Check node health
    pub async fn health(&self) -> Result<HealthStatus> {
        let response = self
            .http
            .get(format!("{}/health", self.url.trim_end_matches('/')))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(HealthStatus::Ok)
        } else {
            let text = response.text().await.unwrap_or_default();
            if text.contains("behind") {
                // Parse slots behind if available
                Ok(HealthStatus::Behind { slots: 0 })
            } else {
                Ok(HealthStatus::Unknown)
            }
        }
    }

    /// Get account balance in lamports
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        let params = serde_json::json!([pubkey.to_string()]);
        let result: serde_json::Value = self.request("getBalance", params).await?;
        
        result["value"]
            .as_u64()
            .ok_or_else(|| Error::Parse("Invalid balance response".to_string()))
    }

    /// Get account info
    pub async fn get_account_info(&self, pubkey: &Pubkey) -> Result<Option<AccountInfoResponse>> {
        let params = serde_json::json!([
            pubkey.to_string(),
            { "encoding": "base64" }
        ]);
        
        let result: serde_json::Value = self.request("getAccountInfo", params).await?;
        
        if result["value"].is_null() {
            return Ok(None);
        }

        let info: AccountInfoResponse = serde_json::from_value(result["value"].clone())?;
        Ok(Some(info))
    }

    /// Get current slot
    pub async fn get_slot(&self) -> Result<u64> {
        let result: u64 = self.request("getSlot", serde_json::json!([])).await?;
        Ok(result)
    }

    /// Get epoch info
    pub async fn get_epoch_info(&self) -> Result<EpochInfo> {
        self.request("getEpochInfo", serde_json::json!([])).await
    }

    /// Get version
    pub async fn get_version(&self) -> Result<VersionInfo> {
        self.request("getVersion", serde_json::json!([])).await
    }

    /// Get latest blockhash
    pub async fn get_latest_blockhash(&self) -> Result<String> {
        let result: serde_json::Value = self
            .request("getLatestBlockhash", serde_json::json!([]))
            .await?;

        result["value"]["blockhash"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::Parse("Invalid blockhash response".to_string()))
    }

    /// Request airdrop
    pub async fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> Result<String> {
        let params = serde_json::json!([pubkey.to_string(), lamports]);
        self.request("requestAirdrop", params).await
    }

    /// Get minimum balance for rent exemption
    pub async fn get_minimum_balance_for_rent_exemption(&self, data_len: usize) -> Result<u64> {
        let params = serde_json::json!([data_len]);
        self.request("getMinimumBalanceForRentExemption", params).await
    }

    /// Get transaction count
    pub async fn get_transaction_count(&self) -> Result<u64> {
        self.request("getTransactionCount", serde_json::json!([])).await
    }
}

impl std::fmt::Debug for RpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcClient")
            .field("url", &self.url)
            .field("timeout", &self.timeout)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        let client = RpcClient::new("https://api.devnet.solana.com");
        assert!(client.url().contains("devnet"));
    }
}

