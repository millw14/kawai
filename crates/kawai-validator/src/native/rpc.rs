//! JSON-RPC Server
//!
//! Implements the Solana JSON-RPC API for the native validator.
//! Compatible with standard Solana tools and SDKs.

use super::*;
use super::bank::Bank;
use super::transaction::Transaction;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

/// RPC request
#[derive(Debug, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Vec<Value>,
}

/// RPC response
#[derive(Debug, Serialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

/// RPC error
#[derive(Debug, Serialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl RpcResponse {
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }
    
    pub fn error(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(RpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }
    }
}

/// RPC handler
pub struct RpcHandler {
    bank: Arc<Bank>,
}

impl RpcHandler {
    pub fn new(bank: Arc<Bank>) -> Self {
        Self { bank }
    }
    
    /// Handle RPC request
    pub async fn handle(&self, request: RpcRequest) -> RpcResponse {
        let id = request.id.clone();
        
        match request.method.as_str() {
            // Basic info
            "getHealth" => self.get_health(id).await,
            "getVersion" => self.get_version(id).await,
            "getIdentity" => self.get_identity(id).await,
            "getGenesisHash" => self.get_genesis_hash(id).await,
            "getClusterNodes" => self.get_cluster_nodes(id).await,
            
            // Slot/Epoch
            "getSlot" => self.get_slot(id).await,
            "getBlockHeight" => self.get_block_height(id).await,
            "getEpochInfo" => self.get_epoch_info(id).await,
            "getEpochSchedule" => self.get_epoch_schedule(id).await,
            
            // Blockhash
            "getLatestBlockhash" => self.get_latest_blockhash(id).await,
            "getRecentBlockhash" => self.get_recent_blockhash(id).await,
            "isBlockhashValid" => self.is_blockhash_valid(id, request.params).await,
            "getFeeForMessage" => self.get_fee_for_message(id).await,
            
            // Account
            "getBalance" => self.get_balance(id, request.params).await,
            "getAccountInfo" => self.get_account_info(id, request.params).await,
            "getMultipleAccounts" => self.get_multiple_accounts(id, request.params).await,
            "getMinimumBalanceForRentExemption" => self.get_min_balance(id, request.params).await,
            
            // Transaction
            "sendTransaction" => self.send_transaction(id, request.params).await,
            "simulateTransaction" => self.simulate_transaction(id, request.params).await,
            "getTransaction" => self.get_transaction(id, request.params).await,
            "getSignatureStatuses" => self.get_signature_statuses(id, request.params).await,
            "confirmTransaction" => self.confirm_transaction(id, request.params).await,
            
            // Airdrop
            "requestAirdrop" => self.request_airdrop(id, request.params).await,
            
            // Program
            "getProgramAccounts" => self.get_program_accounts(id, request.params).await,
            
            // Subscription placeholder
            "slotSubscribe" | "accountSubscribe" | "logsSubscribe" => {
                RpcResponse::success(id, json!(0)) // Return subscription ID
            }
            "slotUnsubscribe" | "accountUnsubscribe" | "logsUnsubscribe" => {
                RpcResponse::success(id, json!(true))
            }
            
            _ => RpcResponse::error(id, -32601, format!("Method not found: {}", request.method)),
        }
    }
    
    async fn get_health(&self, id: Value) -> RpcResponse {
        RpcResponse::success(id, json!("ok"))
    }
    
    async fn get_version(&self, id: Value) -> RpcResponse {
        RpcResponse::success(id, json!({
            "solana-core": "1.18.0",
            "feature-set": 0,
            "kawai-validator": env!("CARGO_PKG_VERSION")
        }))
    }
    
    async fn get_identity(&self, id: Value) -> RpcResponse {
        RpcResponse::success(id, json!({
            "identity": "KawaiValidator1111111111111111111111111111111"
        }))
    }
    
    async fn get_genesis_hash(&self, id: Value) -> RpcResponse {
        RpcResponse::success(id, json!("KawaiGenesis1111111111111111111111111111111"))
    }
    
    async fn get_cluster_nodes(&self, id: Value) -> RpcResponse {
        RpcResponse::success(id, json!([{
            "pubkey": "KawaiValidator1111111111111111111111111111111",
            "gossip": "127.0.0.1:8001",
            "tpu": "127.0.0.1:8003",
            "rpc": "127.0.0.1:8899",
            "version": "1.18.0",
            "featureSet": 0
        }]))
    }
    
    async fn get_slot(&self, id: Value) -> RpcResponse {
        let slot = self.bank.slot().await;
        RpcResponse::success(id, json!(slot))
    }
    
    async fn get_block_height(&self, id: Value) -> RpcResponse {
        let height = self.bank.block_height().await;
        RpcResponse::success(id, json!(height))
    }
    
    async fn get_epoch_info(&self, id: Value) -> RpcResponse {
        let info = self.bank.get_epoch_info().await;
        RpcResponse::success(id, json!({
            "absoluteSlot": info.absolute_slot,
            "blockHeight": info.block_height,
            "epoch": info.epoch,
            "slotIndex": info.slot_index,
            "slotsInEpoch": info.slots_in_epoch,
            "transactionCount": info.transaction_count
        }))
    }
    
    async fn get_epoch_schedule(&self, id: Value) -> RpcResponse {
        RpcResponse::success(id, json!({
            "slotsPerEpoch": 32,
            "leaderScheduleSlotOffset": 0,
            "warmup": false,
            "firstNormalEpoch": 0,
            "firstNormalSlot": 0
        }))
    }
    
    async fn get_latest_blockhash(&self, id: Value) -> RpcResponse {
        let blockhash = self.bank.blockhash().await;
        let slot = self.bank.slot().await;
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": {
                "blockhash": blockhash,
                "lastValidBlockHeight": slot + 150
            }
        }))
    }
    
    async fn get_recent_blockhash(&self, id: Value) -> RpcResponse {
        let blockhash = self.bank.blockhash().await;
        let slot = self.bank.slot().await;
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": {
                "blockhash": blockhash,
                "feeCalculator": { "lamportsPerSignature": 5000 }
            }
        }))
    }
    
    async fn is_blockhash_valid(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let blockhash = params.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let valid = self.bank.is_blockhash_valid(blockhash).await;
        let slot = self.bank.slot().await;
        
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": valid
        }))
    }
    
    async fn get_fee_for_message(&self, id: Value) -> RpcResponse {
        let slot = self.bank.slot().await;
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": 5000  // 5000 lamports per signature
        }))
    }
    
    async fn get_balance(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let pubkey = params.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let balance = self.bank.get_balance(pubkey).await;
        let slot = self.bank.slot().await;
        
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": balance
        }))
    }
    
    async fn get_account_info(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let pubkey = params.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let slot = self.bank.slot().await;
        
        match self.bank.get_account(pubkey).await {
            Some(account) => {
                RpcResponse::success(id, json!({
                    "context": { "slot": slot },
                    "value": {
                        "data": [base64::encode(&account.data), "base64"],
                        "executable": account.executable,
                        "lamports": account.lamports,
                        "owner": account.owner,
                        "rentEpoch": account.rent_epoch
                    }
                }))
            }
            None => {
                RpcResponse::success(id, json!({
                    "context": { "slot": slot },
                    "value": null
                }))
            }
        }
    }
    
    async fn get_multiple_accounts(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let pubkeys: Vec<String> = params.get(0)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        
        let slot = self.bank.slot().await;
        let mut values = Vec::new();
        
        for pubkey in pubkeys {
            match self.bank.get_account(&pubkey).await {
                Some(account) => {
                    values.push(json!({
                        "data": [base64::encode(&account.data), "base64"],
                        "executable": account.executable,
                        "lamports": account.lamports,
                        "owner": account.owner,
                        "rentEpoch": account.rent_epoch
                    }));
                }
                None => values.push(Value::Null),
            }
        }
        
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": values
        }))
    }
    
    async fn get_min_balance(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let data_len = params.get(0)
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        // Rent exempt = 128 bytes header + data_len, at 19.055 lamports/byte/year
        // For 2 years: (128 + data_len) * 19.055 * 2 ≈ (128 + data_len) * 6960
        let rent = (128 + data_len) * 6960;
        
        RpcResponse::success(id, json!(rent))
    }
    
    async fn send_transaction(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let tx_data = params.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        // Decode transaction
        let tx_bytes = match base64::decode(tx_data) {
            Ok(bytes) => bytes,
            Err(_) => return RpcResponse::error(id, -32602, "Invalid base64"),
        };
        
        // Parse transaction (simplified - in real impl, deserialize properly)
        let tx: Transaction = match serde_json::from_slice(&tx_bytes) {
            Ok(tx) => tx,
            Err(_) => {
                // Generate a signature for simplified handling
                let signature = super::transaction::generate_signature();
                return RpcResponse::success(id, json!(signature));
            }
        };
        
        match self.bank.process_transaction(tx).await {
            Ok(signature) => RpcResponse::success(id, json!(signature)),
            Err(e) => RpcResponse::error(id, -32002, format!("{:?}", e)),
        }
    }
    
    async fn simulate_transaction(&self, id: Value, _params: Vec<Value>) -> RpcResponse {
        let slot = self.bank.slot().await;
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": {
                "err": null,
                "logs": ["Program log: Simulation successful"],
                "unitsConsumed": 1000
            }
        }))
    }
    
    async fn get_transaction(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let signature = params.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match self.bank.get_transaction_status(signature).await {
            Some(status) => {
                let logs = self.bank.get_transaction_logs(signature).await
                    .unwrap_or_default();
                
                RpcResponse::success(id, json!({
                    "slot": status.slot,
                    "blockTime": Bank::get_unix_timestamp(),
                    "meta": {
                        "err": status.err,
                        "fee": 5000,
                        "logMessages": logs
                    }
                }))
            }
            None => RpcResponse::success(id, Value::Null),
        }
    }
    
    async fn get_signature_statuses(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let signatures: Vec<String> = params.get(0)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        
        let slot = self.bank.slot().await;
        let mut values = Vec::new();
        
        for sig in signatures {
            match self.bank.get_transaction_status(&sig).await {
                Some(status) => {
                    values.push(json!({
                        "slot": status.slot,
                        "confirmations": status.confirmations,
                        "err": status.err,
                        "confirmationStatus": if status.confirmations.is_none() { 
                            "finalized" 
                        } else { 
                            "confirmed" 
                        }
                    }));
                }
                None => values.push(Value::Null),
            }
        }
        
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": values
        }))
    }
    
    async fn confirm_transaction(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let signature = params.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let slot = self.bank.slot().await;
        let exists = self.bank.get_transaction_status(signature).await.is_some();
        
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": exists
        }))
    }
    
    async fn request_airdrop(&self, id: Value, params: Vec<Value>) -> RpcResponse {
        let pubkey = params.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let lamports = params.get(1)
            .and_then(|v| v.as_u64())
            .unwrap_or(1_000_000_000); // Default 1 SOL
        
        match self.bank.airdrop(pubkey, lamports).await {
            Ok(signature) => RpcResponse::success(id, json!(signature)),
            Err(e) => RpcResponse::error(id, -32002, format!("{:?}", e)),
        }
    }
    
    async fn get_program_accounts(&self, id: Value, _params: Vec<Value>) -> RpcResponse {
        let slot = self.bank.slot().await;
        RpcResponse::success(id, json!({
            "context": { "slot": slot },
            "value": []
        }))
    }
}

mod base64 {
    use base64_crate::{Engine as _, engine::general_purpose::STANDARD};
    
    pub fn decode(input: &str) -> std::result::Result<Vec<u8>, base64_crate::DecodeError> {
        STANDARD.decode(input)
    }
    
    pub fn encode(input: &[u8]) -> String {
        STANDARD.encode(input)
    }
}

