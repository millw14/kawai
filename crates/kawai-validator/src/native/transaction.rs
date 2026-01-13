//! Transaction types and processing

use super::*;
use super::bpf::{AccountMeta, Instruction};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Transaction signature (64 bytes as base58)
pub type Signature = String;

/// A Solana transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Signatures
    pub signatures: Vec<Signature>,
    /// Message
    pub message: Message,
}

/// Transaction message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Header with signer info
    pub header: MessageHeader,
    /// Account keys
    pub account_keys: Vec<String>,
    /// Recent blockhash
    pub recent_blockhash: String,
    /// Instructions
    pub instructions: Vec<CompiledInstruction>,
}

/// Message header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

/// Compiled instruction (uses indices)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledInstruction {
    /// Index of program ID in account_keys
    pub program_id_index: u8,
    /// Account indices
    pub accounts: Vec<u8>,
    /// Data as base64
    pub data: String,
}

impl Transaction {
    /// Get first signature
    pub fn signature(&self) -> &str {
        self.signatures.first().map(|s| s.as_str()).unwrap_or("")
    }
    
    /// Expand compiled instructions to full instructions
    pub fn expand_instructions(&self) -> Vec<Instruction> {
        self.message.instructions.iter().map(|ci| {
            let program_id = self.message.account_keys
                .get(ci.program_id_index as usize)
                .cloned()
                .unwrap_or_default();
            
            let accounts = ci.accounts.iter().map(|&idx| {
                let pubkey = self.message.account_keys
                    .get(idx as usize)
                    .cloned()
                    .unwrap_or_default();
                
                let is_signer = (idx as u8) < self.message.header.num_required_signatures;
                let is_writable = !self.is_readonly(idx as usize);
                
                AccountMeta {
                    pubkey,
                    is_signer,
                    is_writable,
                }
            }).collect();
            
            let data = base64::decode(&ci.data).unwrap_or_default();
            
            Instruction {
                program_id,
                accounts,
                data,
            }
        }).collect()
    }
    
    /// Check if account at index is readonly
    fn is_readonly(&self, index: usize) -> bool {
        let header = &self.message.header;
        let num_signed = header.num_required_signatures as usize;
        let num_readonly_signed = header.num_readonly_signed_accounts as usize;
        let num_readonly_unsigned = header.num_readonly_unsigned_accounts as usize;
        
        if index < num_signed {
            // Signed accounts: readonly if in readonly_signed range
            index >= (num_signed - num_readonly_signed)
        } else {
            // Unsigned accounts: readonly if in readonly_unsigned range
            let unsigned_start = num_signed;
            let num_unsigned = self.message.account_keys.len() - num_signed;
            index >= (unsigned_start + num_unsigned - num_readonly_unsigned)
        }
    }
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    /// Slot
    pub slot: u64,
    /// Confirmations (None = finalized)
    pub confirmations: Option<u64>,
    /// Error (None = success)
    pub err: Option<String>,
    /// Status
    pub status: TransactionStatusValue,
}

/// Transaction status value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatusValue {
    Ok,
    Err(String),
}

/// Generate a unique transaction signature
pub fn generate_signature() -> Signature {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..64).map(|_| rng.gen()).collect();
    bs58::encode(bytes).into_string()
}

/// Generate a blockhash
pub fn generate_blockhash() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    bs58::encode(bytes).into_string()
}

/// Compute transaction hash
pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    bs58::encode(result).into_string()
}

mod base64 {
    use base64_crate::{Engine as _, engine::general_purpose::STANDARD};
    
    pub fn decode(input: &str) -> Result<Vec<u8>, base64_crate::DecodeError> {
        STANDARD.decode(input)
    }
    
    pub fn encode(input: &[u8]) -> String {
        STANDARD.encode(input)
    }
}

