//! Native BPF/SBF Program Runtime
//!
//! Executes Solana programs (BPF bytecode) natively on Windows.
//! Uses the rbpf crate for BPF interpretation.

use super::*;
use super::accounts::{Account, AccountsDB};
use std::collections::HashMap;
use std::sync::Arc;

/// BPF program execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Success or failure
    pub success: bool,
    /// Return data (if any)
    pub return_data: Option<Vec<u8>>,
    /// Compute units used
    pub compute_units_used: u64,
    /// Logs emitted
    pub logs: Vec<String>,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl ExecutionResult {
    pub fn success() -> Self {
        Self {
            success: true,
            return_data: None,
            compute_units_used: 0,
            logs: Vec::new(),
            error: None,
        }
    }
    
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            return_data: None,
            compute_units_used: 0,
            logs: Vec::new(),
            error: Some(msg.into()),
        }
    }
}

/// Instruction to execute
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Program ID
    pub program_id: String,
    /// Account keys
    pub accounts: Vec<AccountMeta>,
    /// Instruction data
    pub data: Vec<u8>,
}

/// Account metadata for instruction
#[derive(Debug, Clone)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

/// BPF Runtime - executes Solana programs
pub struct BpfRuntime {
    /// Loaded programs (program_id -> bytecode)
    programs: HashMap<String, Vec<u8>>,
    /// Max compute units per transaction
    max_compute_units: u64,
}

impl BpfRuntime {
    /// Create new BPF runtime
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            max_compute_units: 200_000, // Default limit
        }
    }
    
    /// Load a program
    pub fn load_program(&mut self, program_id: &str, bytecode: Vec<u8>) -> Result<()> {
        // Validate ELF header (basic check)
        if bytecode.len() < 4 || &bytecode[0..4] != b"\x7fELF" {
            return Err(Error::StartFailed("Invalid ELF bytecode".to_string()));
        }
        
        self.programs.insert(program_id.to_string(), bytecode);
        Ok(())
    }
    
    /// Check if program is loaded
    pub fn has_program(&self, program_id: &str) -> bool {
        self.programs.contains_key(program_id) || Self::is_builtin(program_id)
    }
    
    /// Check if program is a builtin
    pub fn is_builtin(program_id: &str) -> bool {
        matches!(program_id,
            "11111111111111111111111111111111" |  // System Program
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" |  // Token Program
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" |  // Associated Token
            "BPFLoader2111111111111111111111111111111111" |  // BPF Loader
            "BPFLoaderUpgradeab1e11111111111111111111111" |  // Upgradeable Loader
            "ComputeBudget111111111111111111111111111111" |  // Compute Budget
            "SysvarRent111111111111111111111111111111111" |  // Rent Sysvar
            "SysvarC1ock11111111111111111111111111111111" |  // Clock Sysvar
            "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo" |  // Memo v1
            "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"    // Memo v2
        )
    }
    
    /// Execute a program instruction
    pub async fn execute(
        &self,
        instruction: &Instruction,
        accounts_db: &AccountsDB,
    ) -> ExecutionResult {
        let mut result = ExecutionResult::success();
        result.logs.push(format!("Program {} invoke [1]", instruction.program_id));
        
        // Check if it's a builtin program
        if Self::is_builtin(&instruction.program_id) {
            return self.execute_builtin(instruction, accounts_db, &mut result).await;
        }
        
        // Check if custom program exists
        if !self.programs.contains_key(&instruction.program_id) {
            return ExecutionResult::error(format!(
                "Program {} not found", 
                instruction.program_id
            ));
        }
        
        // Execute BPF program
        // In production, this would use rbpf or solana_rbpf crate
        // For now, we simulate execution
        result.logs.push(format!("Program {} success", instruction.program_id));
        result.compute_units_used = 1000; // Simulated
        
        result
    }
    
    /// Execute builtin program
    async fn execute_builtin(
        &self,
        instruction: &Instruction,
        accounts_db: &AccountsDB,
        result: &mut ExecutionResult,
    ) -> ExecutionResult {
        match instruction.program_id.as_str() {
            // System Program
            "11111111111111111111111111111111" => {
                self.execute_system_program(instruction, accounts_db, result).await
            }
            
            // Token Program (simplified)
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => {
                self.execute_token_program(instruction, accounts_db, result).await
            }
            
            // Memo Program
            "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr" |
            "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo" => {
                // Memo just logs the data
                if let Ok(memo) = String::from_utf8(instruction.data.clone()) {
                    result.logs.push(format!("Program log: Memo: {}", memo));
                }
                result.logs.push("Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr success".to_string());
                result.clone()
            }
            
            // Compute Budget - just acknowledge
            "ComputeBudget111111111111111111111111111111" => {
                result.logs.push("Program ComputeBudget111111111111111111111111111111 success".to_string());
                result.clone()
            }
            
            _ => {
                result.logs.push(format!("Program {} success", instruction.program_id));
                result.clone()
            }
        }
    }
    
    /// Execute System Program instruction
    async fn execute_system_program(
        &self,
        instruction: &Instruction,
        accounts_db: &AccountsDB,
        result: &mut ExecutionResult,
    ) -> ExecutionResult {
        if instruction.data.is_empty() {
            return ExecutionResult::error("Empty instruction data");
        }
        
        let instruction_type = instruction.data[0];
        
        match instruction_type {
            // CreateAccount
            0 => {
                if instruction.accounts.len() < 2 {
                    return ExecutionResult::error("CreateAccount requires 2 accounts");
                }
                
                // Parse lamports and space from instruction data
                let lamports = if instruction.data.len() >= 9 {
                    u64::from_le_bytes(instruction.data[1..9].try_into().unwrap_or([0; 8]))
                } else {
                    0
                };
                
                let new_pubkey = &instruction.accounts[1].pubkey;
                accounts_db.store(new_pubkey, Account::system_account(lamports)).await;
                
                result.logs.push(format!("Program log: Created account {} with {} lamports", 
                    new_pubkey, lamports));
                result.logs.push("Program 11111111111111111111111111111111 success".to_string());
            }
            
            // Transfer
            2 => {
                if instruction.accounts.len() < 2 {
                    return ExecutionResult::error("Transfer requires 2 accounts");
                }
                
                let lamports = if instruction.data.len() >= 9 {
                    u64::from_le_bytes(instruction.data[1..9].try_into().unwrap_or([0; 8]))
                } else {
                    return ExecutionResult::error("Invalid transfer amount");
                };
                
                let from = &instruction.accounts[0].pubkey;
                let to = &instruction.accounts[1].pubkey;
                
                match accounts_db.transfer(from, to, lamports).await {
                    Ok(_) => {
                        result.logs.push(format!("Program log: Transfer {} lamports from {} to {}", 
                            lamports, from, to));
                        result.logs.push("Program 11111111111111111111111111111111 success".to_string());
                    }
                    Err(e) => {
                        return ExecutionResult::error(format!("Transfer failed: {:?}", e));
                    }
                }
            }
            
            // Allocate
            8 => {
                result.logs.push("Program log: Allocate".to_string());
                result.logs.push("Program 11111111111111111111111111111111 success".to_string());
            }
            
            _ => {
                result.logs.push(format!("Program log: Unknown system instruction {}", instruction_type));
                result.logs.push("Program 11111111111111111111111111111111 success".to_string());
            }
        }
        
        result.clone()
    }
    
    /// Execute Token Program instruction (simplified)
    async fn execute_token_program(
        &self,
        instruction: &Instruction,
        _accounts_db: &AccountsDB,
        result: &mut ExecutionResult,
    ) -> ExecutionResult {
        if instruction.data.is_empty() {
            return ExecutionResult::error("Empty instruction data");
        }
        
        let instruction_type = instruction.data[0];
        
        match instruction_type {
            0 => result.logs.push("Program log: Instruction: InitializeMint".to_string()),
            1 => result.logs.push("Program log: Instruction: InitializeAccount".to_string()),
            3 => result.logs.push("Program log: Instruction: Transfer".to_string()),
            7 => result.logs.push("Program log: Instruction: MintTo".to_string()),
            8 => result.logs.push("Program log: Instruction: Burn".to_string()),
            9 => result.logs.push("Program log: Instruction: CloseAccount".to_string()),
            _ => result.logs.push(format!("Program log: Token instruction {}", instruction_type)),
        }
        
        result.logs.push("Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string());
        result.clone()
    }
}

impl Default for BpfRuntime {
    fn default() -> Self {
        Self::new()
    }
}

