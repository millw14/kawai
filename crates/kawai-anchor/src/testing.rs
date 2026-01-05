//! Testing utilities for Anchor programs

use crate::error::{Error, Result};
use kawai_validator::{Validator, ValidatorConfig};
use std::path::Path;
use std::time::Duration;

/// Test environment for Anchor programs
pub struct TestEnvironment {
    validator: Option<Validator>,
    rpc_url: String,
}

impl TestEnvironment {
    /// Create a new test environment
    pub async fn new() -> Result<Self> {
        let validator = Validator::auto().await
            .map_err(Error::from)?;

        Ok(Self {
            validator: Some(validator),
            rpc_url: "http://127.0.0.1:8899".to_string(),
        })
    }

    /// Create with custom validator config
    pub async fn with_config(config: ValidatorConfig) -> Result<Self> {
        let validator = Validator::with_config(config).await
            .map_err(Error::from)?;

        let rpc_url = validator.rpc_url();

        Ok(Self {
            validator: Some(validator),
            rpc_url,
        })
    }

    /// Start the test environment
    pub async fn start(&self) -> Result<()> {
        if let Some(validator) = &self.validator {
            validator.start().await
                .map_err(Error::from)?;
        }
        Ok(())
    }

    /// Stop the test environment
    pub async fn stop(&self) -> Result<()> {
        if let Some(validator) = &self.validator {
            validator.stop().await
                .map_err(Error::from)?;
        }
        Ok(())
    }

    /// Get the RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// Deploy a program for testing
    pub async fn deploy_program(&self, so_path: &Path) -> Result<String> {
        // TODO: Implement program deployment
        // This would use the BPF loader to deploy the program
        
        // For now, return a placeholder
        Ok("11111111111111111111111111111111".to_string())
    }

    /// Request an airdrop for testing
    pub async fn airdrop(&self, pubkey: &str, amount: u64) -> Result<()> {
        if let Some(validator) = &self.validator {
            validator.airdrop(pubkey, amount).await
                .map_err(Error::from)?;
        }
        Ok(())
    }
}

/// Test fixture for Anchor programs
pub struct TestFixture {
    /// Program ID
    pub program_id: String,
    /// Test accounts
    pub accounts: Vec<TestAccount>,
}

/// Test account
pub struct TestAccount {
    pub pubkey: String,
    pub keypair: Vec<u8>,
    pub balance: u64,
}

impl TestFixture {
    /// Create a new test fixture
    pub fn new(program_id: &str) -> Self {
        Self {
            program_id: program_id.to_string(),
            accounts: Vec::new(),
        }
    }

    /// Add a test account
    pub fn add_account(&mut self, pubkey: &str, keypair: Vec<u8>, balance: u64) {
        self.accounts.push(TestAccount {
            pubkey: pubkey.to_string(),
            keypair,
            balance,
        });
    }
}

/// Run tests with automatic setup/teardown
pub async fn run_test<F, Fut>(test_fn: F) -> Result<()>
where
    F: FnOnce(TestEnvironment) -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let env = TestEnvironment::new().await?;
    env.start().await?;

    let result = test_fn(env).await;

    // Environment will be stopped when dropped
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires validator
    async fn test_environment() {
        let env = TestEnvironment::new().await.unwrap();
        assert!(!env.rpc_url().is_empty());
    }
}

