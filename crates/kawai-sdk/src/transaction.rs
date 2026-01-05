//! Transaction builder for Kawai SDK

use crate::error::{Error, Result};
use crate::types::TransactionResult;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::system_instruction;

/// Builder for constructing transactions
pub struct TransactionBuilder {
    instructions: Vec<Instruction>,
    signers: Vec<Keypair>,
    fee_payer: Option<Pubkey>,
    memo: Option<String>,
}

impl TransactionBuilder {
    /// Create a new transaction builder
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            signers: Vec::new(),
            fee_payer: None,
            memo: None,
        }
    }

    /// Add a transfer instruction
    pub fn transfer(mut self, from: &Pubkey, to: &Pubkey, lamports: u64) -> Self {
        let instruction = system_instruction::transfer(from, to, lamports);
        self.instructions.push(instruction);
        self
    }

    /// Add a memo
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }

    /// Set the fee payer
    pub fn fee_payer(mut self, payer: Pubkey) -> Self {
        self.fee_payer = Some(payer);
        self
    }

    /// Add a custom instruction
    pub fn instruction(mut self, instruction: Instruction) -> Self {
        self.instructions.push(instruction);
        self
    }

    /// Get the instructions
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    /// Check if the builder has any instructions
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Get the number of instructions
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Fluent transfer builder
pub struct TransferBuilder {
    from: Option<Pubkey>,
    to: Option<Pubkey>,
    amount: Option<u64>,
    memo: Option<String>,
}

impl TransferBuilder {
    /// Create a new transfer builder
    pub fn new() -> Self {
        Self {
            from: None,
            to: None,
            amount: None,
            memo: None,
        }
    }

    /// Set the sender
    pub fn from(mut self, pubkey: Pubkey) -> Self {
        self.from = Some(pubkey);
        self
    }

    /// Set the recipient
    pub fn to(mut self, pubkey: Pubkey) -> Self {
        self.to = Some(pubkey);
        self
    }

    /// Set the amount in lamports
    pub fn amount(mut self, lamports: u64) -> Self {
        self.amount = Some(lamports);
        self
    }

    /// Set the amount in SOL
    pub fn sol(mut self, sol: f64) -> Self {
        self.amount = Some((sol * 1_000_000_000.0) as u64);
        self
    }

    /// Add a memo
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }

    /// Validate the transfer parameters
    pub fn validate(&self) -> Result<()> {
        if self.from.is_none() {
            return Err(Error::Transaction("Missing 'from' address".to_string()));
        }
        if self.to.is_none() {
            return Err(Error::Transaction("Missing 'to' address".to_string()));
        }
        if self.amount.is_none() || self.amount == Some(0) {
            return Err(Error::Transaction("Amount must be greater than 0".to_string()));
        }
        Ok(())
    }

    /// Build the transfer instruction
    pub fn build(self) -> Result<Instruction> {
        self.validate()?;
        Ok(system_instruction::transfer(
            &self.from.unwrap(),
            &self.to.unwrap(),
            self.amount.unwrap(),
        ))
    }
}

impl Default for TransferBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_transaction_builder() {
        let from = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        
        let builder = TransactionBuilder::new()
            .transfer(&from, &to, 1_000_000_000)
            .memo("Test transfer");
        
        assert_eq!(builder.len(), 1);
        assert!(!builder.is_empty());
    }

    #[test]
    fn test_transfer_builder() {
        let from = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        
        let transfer = TransferBuilder::new()
            .from(from)
            .to(to)
            .sol(1.0)
            .build();
        
        assert!(transfer.is_ok());
    }

    #[test]
    fn test_transfer_builder_validation() {
        let builder = TransferBuilder::new();
        assert!(builder.validate().is_err());
    }
}

