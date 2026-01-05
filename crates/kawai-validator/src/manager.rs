//! Validator manager - high-level API for managing local validators

use crate::backend::{Backend, CloudBackend, DockerBackend, ValidatorBackend, WslBackend};
use crate::config::{BackendPreference, ValidatorConfig};
use crate::error::{Error, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

/// High-level validator manager
pub struct Validator {
    config: ValidatorConfig,
    backend: Arc<dyn ValidatorBackend>,
    state: Arc<RwLock<ValidatorState>>,
}

#[derive(Debug, Default)]
struct ValidatorState {
    running: bool,
    start_time: Option<std::time::Instant>,
}

impl Validator {
    /// Create validator with auto-detected backend
    pub async fn auto() -> Result<Self> {
        Self::with_config(ValidatorConfig::default()).await
    }

    /// Create validator with custom configuration
    pub async fn with_config(config: ValidatorConfig) -> Result<Self> {
        let backend = Self::select_backend(&config).await?;
        
        Ok(Self {
            config,
            backend,
            state: Arc::new(RwLock::new(ValidatorState::default())),
        })
    }

    /// Create with Docker backend
    pub fn docker() -> Result<Self> {
        let config = ValidatorConfig::default().backend(BackendPreference::Docker);
        let backend: Arc<dyn ValidatorBackend> = Arc::new(DockerBackend::new(&config.docker_image));
        
        Ok(Self {
            config,
            backend,
            state: Arc::new(RwLock::new(ValidatorState::default())),
        })
    }

    /// Create with WSL backend
    pub fn wsl() -> Result<Self> {
        let config = ValidatorConfig::default().backend(BackendPreference::Wsl);
        let backend: Arc<dyn ValidatorBackend> = Arc::new(WslBackend::new());
        
        Ok(Self {
            config,
            backend,
            state: Arc::new(RwLock::new(ValidatorState::default())),
        })
    }

    /// Create with cloud backend (devnet)
    pub fn cloud() -> Result<Self> {
        let config = ValidatorConfig::default().backend(BackendPreference::Cloud);
        let backend: Arc<dyn ValidatorBackend> = Arc::new(CloudBackend::devnet());
        
        Ok(Self {
            config,
            backend,
            state: Arc::new(RwLock::new(ValidatorState::default())),
        })
    }

    /// Select the appropriate backend based on configuration
    async fn select_backend(config: &ValidatorConfig) -> Result<Arc<dyn ValidatorBackend>> {
        match config.backend {
            BackendPreference::Docker => {
                if Backend::docker_available().await {
                    Ok(Arc::new(DockerBackend::new(&config.docker_image)))
                } else {
                    Err(Error::DockerNotAvailable)
                }
            }
            BackendPreference::Wsl => {
                if Backend::wsl_available().await {
                    Ok(Arc::new(WslBackend::new()))
                } else {
                    Err(Error::WslNotAvailable)
                }
            }
            BackendPreference::Cloud => {
                Ok(Arc::new(CloudBackend::devnet()))
            }
            BackendPreference::Native => {
                // Native not yet implemented
                Err(Error::NoBackendAvailable)
            }
            BackendPreference::Auto => {
                // Try backends in order of preference
                if Backend::docker_available().await {
                    return Ok(Arc::new(DockerBackend::new(&config.docker_image)));
                }
                if Backend::wsl_available().await {
                    return Ok(Arc::new(WslBackend::new()));
                }
                // Fall back to cloud
                Ok(Arc::new(CloudBackend::devnet()))
            }
        }
    }

    /// Start the validator
    pub async fn start(&self) -> Result<()> {
        // Check if already running
        if self.is_running().await {
            return Err(Error::AlreadyRunning(self.config.rpc_port));
        }

        // Check port availability
        if !Backend::port_available(self.config.rpc_port).await {
            return Err(Error::PortInUse(self.config.rpc_port));
        }

        // Start the backend
        self.backend.start(&self.config).await?;

        // Update state
        let mut state = self.state.write().await;
        state.running = true;
        state.start_time = Some(std::time::Instant::now());

        Ok(())
    }

    /// Stop the validator
    pub async fn stop(&self) -> Result<()> {
        if !self.is_running().await {
            return Ok(()); // Already stopped
        }

        self.backend.stop().await?;

        // Update state
        let mut state = self.state.write().await;
        state.running = false;
        state.start_time = None;

        Ok(())
    }

    /// Check if validator is running
    pub async fn is_running(&self) -> bool {
        self.backend.is_running().await
    }

    /// Get the RPC URL
    pub fn rpc_url(&self) -> String {
        self.config.rpc_url()
    }

    /// Get the WebSocket URL
    pub fn ws_url(&self) -> String {
        self.config.ws_url()
    }

    /// Get the backend name
    pub fn backend_name(&self) -> &'static str {
        self.backend.name()
    }

    /// Get configuration
    pub fn config(&self) -> &ValidatorConfig {
        &self.config
    }

    /// Get uptime
    pub async fn uptime(&self) -> Option<std::time::Duration> {
        let state = self.state.read().await;
        state.start_time.map(|t| t.elapsed())
    }

    /// Request airdrop (convenience method)
    pub async fn airdrop(&self, pubkey: &str, lamports: u64) -> Result<String> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.rpc_url())
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "requestAirdrop",
                "params": [pubkey, lamports]
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        result["result"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::Network("Invalid airdrop response".to_string()))
    }

    /// Get current slot
    pub async fn slot(&self) -> Result<u64> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.rpc_url())
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSlot"
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        result["result"]
            .as_u64()
            .ok_or_else(|| Error::Network("Invalid slot response".to_string()))
    }

    /// Get validator logs (Docker backend only)
    pub async fn logs(&self, lines: usize) -> Result<String> {
        if self.backend.name() != "Docker" {
            return Err(Error::Config("Logs only available for Docker backend".to_string()));
        }

        let output = tokio::process::Command::new("docker")
            .args(["logs", "--tail", &lines.to_string(), "kawai-validator"])
            .output()
            .await?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl std::fmt::Debug for Validator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Validator")
            .field("rpc_url", &self.rpc_url())
            .field("backend", &self.backend.name())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auto_detect() {
        // This will use whatever backend is available
        let result = Validator::auto().await;
        // Should always succeed (cloud is always available)
        assert!(result.is_ok() || matches!(result, Err(Error::NoBackendAvailable)));
    }

    #[tokio::test]
    async fn test_cloud_backend() {
        let validator = Validator::cloud().unwrap();
        assert_eq!(validator.backend_name(), "Cloud");
    }
}

