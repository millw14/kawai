//! RPC types

use serde::{Deserialize, Serialize};

/// RPC request
#[derive(Debug, Serialize)]
pub struct RpcRequest {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

impl RpcRequest {
    pub fn new(method: &str, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0",
            id: 1,
            method: method.to_string(),
            params,
        }
    }
}

/// RPC response
#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(default)]
    pub result: Option<T>,
    #[serde(default)]
    pub error: Option<RpcError>,
}

/// RPC error
#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// Account info response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountInfoResponse {
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
    #[serde(default)]
    pub data: AccountData,
}

/// Account data
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AccountData {
    #[default]
    Empty,
    Base64(String, String),
    Json(serde_json::Value),
}

/// Block info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockInfo {
    pub blockhash: String,
    pub parent_slot: u64,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
}

/// Epoch info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpochInfo {
    pub epoch: u64,
    pub slot_index: u64,
    pub slots_in_epoch: u64,
    pub absolute_slot: u64,
    pub block_height: u64,
    pub transaction_count: Option<u64>,
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Ok,
    Behind { slots: u64 },
    Unknown,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Ok => write!(f, "✅ Healthy"),
            HealthStatus::Behind { slots } => write!(f, "⚠️ Behind by {} slots", slots),
            HealthStatus::Unknown => write!(f, "❓ Unknown"),
        }
    }
}

/// Version info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionInfo {
    #[serde(rename = "solana-core")]
    pub solana_core: String,
    #[serde(rename = "feature-set")]
    pub feature_set: Option<u64>,
}

/// Transaction confirmation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
}

impl Default for Commitment {
    fn default() -> Self {
        Commitment::Confirmed
    }
}

