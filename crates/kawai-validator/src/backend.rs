//! Validator backends for different execution environments

use crate::config::ValidatorConfig;
use crate::error::{Error, Result};
use std::process::Stdio;
use tokio::process::Command;

/// Backend trait for running validators
#[async_trait::async_trait]
pub trait ValidatorBackend: Send + Sync {
    /// Check if this backend is available
    async fn is_available(&self) -> bool;
    
    /// Start the validator
    async fn start(&self, config: &ValidatorConfig) -> Result<()>;
    
    /// Stop the validator
    async fn stop(&self) -> Result<()>;
    
    /// Check if validator is running
    async fn is_running(&self) -> bool;
    
    /// Get the backend name
    fn name(&self) -> &'static str;
}

/// Available backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Docker,
    Wsl,
    Cloud,
    Native,
}

impl Backend {
    /// Check if Docker is available
    pub async fn docker_available() -> bool {
        Command::new("docker")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Check if WSL is available
    pub async fn wsl_available() -> bool {
        Command::new("wsl")
            .arg("--status")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Check if a port is available
    pub async fn port_available(port: u16) -> bool {
        std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
    }

    /// Auto-detect the best available backend
    pub async fn auto_detect() -> Option<Backend> {
        // Try Docker first (most reliable)
        if Self::docker_available().await {
            return Some(Backend::Docker);
        }

        // Try WSL
        if Self::wsl_available().await {
            return Some(Backend::Wsl);
        }

        // Fall back to cloud
        Some(Backend::Cloud)
    }
}

/// Docker backend implementation
pub struct DockerBackend {
    container_name: String,
    image: String,
}

impl DockerBackend {
    pub fn new(image: &str) -> Self {
        Self {
            container_name: "kawai-validator".to_string(),
            image: image.to_string(),
        }
    }

    /// Pull the Docker image
    pub async fn pull_image(&self) -> Result<()> {
        let status = Command::new("docker")
            .args(["pull", &self.image])
            .status()
            .await?;

        if !status.success() {
            return Err(Error::DownloadFailed(format!(
                "Failed to pull Docker image: {}",
                self.image
            )));
        }

        Ok(())
    }

    /// Build the docker run command
    fn build_run_command(&self, config: &ValidatorConfig) -> Command {
        let mut cmd = Command::new("docker");
        cmd.args([
            "run",
            "-d",
            "--name",
            &self.container_name,
            "-p",
            &format!("{}:8899", config.rpc_port),
            "-p",
            &format!("{}:8900", config.ws_port),
            "-p",
            &format!("{}:9900", config.faucet_port),
        ]);

        // Add volume for ledger persistence
        if let Some(ledger_dir) = &config.ledger_dir {
            cmd.args([
                "-v",
                &format!("{}:/ledger", ledger_dir.display()),
            ]);
        }

        cmd.args([
            &self.image,
            "solana-test-validator",
            "--ledger",
            "/ledger",
            "--rpc-port",
            "8899",
            "--bind-address",
            "0.0.0.0",
        ]);

        if config.reset {
            cmd.arg("--reset");
        }

        if let Some(slots) = config.slots_per_epoch {
            cmd.args(["--slots-per-epoch", &slots.to_string()]);
        }

        cmd
    }
}

#[async_trait::async_trait]
impl ValidatorBackend for DockerBackend {
    async fn is_available(&self) -> bool {
        Backend::docker_available().await
    }

    async fn start(&self, config: &ValidatorConfig) -> Result<()> {
        // Check if already running
        if self.is_running().await {
            return Err(Error::AlreadyRunning(config.rpc_port));
        }

        // Remove any existing container
        let _ = Command::new("docker")
            .args(["rm", "-f", &self.container_name])
            .output()
            .await;

        // Start the container
        let mut cmd = self.build_run_command(config);
        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::StartFailed(stderr.to_string()));
        }

        // Wait for validator to be ready
        Self::wait_for_ready(config.rpc_port, 30).await?;

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let status = Command::new("docker")
            .args(["stop", &self.container_name])
            .status()
            .await?;

        if !status.success() {
            return Err(Error::StopFailed("Failed to stop container".to_string()));
        }

        // Remove the container
        let _ = Command::new("docker")
            .args(["rm", &self.container_name])
            .status()
            .await;

        Ok(())
    }

    async fn is_running(&self) -> bool {
        Command::new("docker")
            .args(["inspect", "-f", "{{.State.Running}}", &self.container_name])
            .output()
            .await
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "true")
            .unwrap_or(false)
    }

    fn name(&self) -> &'static str {
        "Docker"
    }
}

impl DockerBackend {
    /// Wait for validator to be ready
    async fn wait_for_ready(port: u16, timeout_secs: u64) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("http://127.0.0.1:{}", port);

        for _ in 0..timeout_secs * 2 {
            if let Ok(response) = client
                .post(&url)
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "getHealth"
                }))
                .send()
                .await
            {
                if response.status().is_success() {
                    return Ok(());
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        Err(Error::Timeout(timeout_secs))
    }
}

/// WSL backend implementation
pub struct WslBackend {
    validator_path: Option<String>,
}

impl WslBackend {
    pub fn new() -> Self {
        Self {
            validator_path: None,
        }
    }

    /// Check if solana-test-validator is installed in WSL
    pub async fn check_solana_installed(&self) -> bool {
        Command::new("wsl")
            .args(["which", "solana-test-validator"])
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Install Solana in WSL
    pub async fn install_solana(&self) -> Result<()> {
        let script = r#"
            sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
            export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
        "#;

        let status = Command::new("wsl")
            .args(["bash", "-c", script])
            .status()
            .await?;

        if !status.success() {
            return Err(Error::InstallFailed("Failed to install Solana in WSL".to_string()));
        }

        Ok(())
    }
}

impl Default for WslBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ValidatorBackend for WslBackend {
    async fn is_available(&self) -> bool {
        Backend::wsl_available().await
    }

    async fn start(&self, config: &ValidatorConfig) -> Result<()> {
        if !self.check_solana_installed().await {
            return Err(Error::NotFound);
        }

        let ledger_dir = config
            .ledger_dir
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "/tmp/kawai-ledger".to_string());

        let mut args = vec![
            "solana-test-validator".to_string(),
            "--ledger".to_string(),
            ledger_dir,
            "--rpc-port".to_string(),
            config.rpc_port.to_string(),
        ];

        if config.reset {
            args.push("--reset".to_string());
        }

        let args_str = args.join(" ");
        
        // Start in background
        let _child = Command::new("wsl")
            .args(["bash", "-c", &format!("nohup {} &", args_str)])
            .spawn()?;

        // Wait for ready
        DockerBackend::wait_for_ready(config.rpc_port, 30).await?;

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let status = Command::new("wsl")
            .args(["pkill", "-f", "solana-test-validator"])
            .status()
            .await?;

        if !status.success() {
            // Process might not exist, which is fine
        }

        Ok(())
    }

    async fn is_running(&self) -> bool {
        Command::new("wsl")
            .args(["pgrep", "-f", "solana-test-validator"])
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn name(&self) -> &'static str {
        "WSL"
    }
}

/// Cloud backend - connects to remote test validators
pub struct CloudBackend {
    rpc_url: String,
}

impl CloudBackend {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
        }
    }

    /// Use Solana devnet as a test environment
    pub fn devnet() -> Self {
        Self::new("https://api.devnet.solana.com")
    }
}

#[async_trait::async_trait]
impl ValidatorBackend for CloudBackend {
    async fn is_available(&self) -> bool {
        // Cloud is always available (if internet works)
        reqwest::get(&format!("{}/health", self.rpc_url.trim_end_matches('/')))
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    async fn start(&self, _config: &ValidatorConfig) -> Result<()> {
        // Cloud validator is already running
        // Just verify it's accessible
        if !self.is_available().await {
            return Err(Error::Network("Cannot connect to cloud validator".to_string()));
        }
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        // Can't stop cloud validator
        Ok(())
    }

    async fn is_running(&self) -> bool {
        self.is_available().await
    }

    fn name(&self) -> &'static str {
        "Cloud"
    }
}

// Need to add async_trait
#[allow(dead_code)]
mod async_trait_impl {
    pub use async_trait::async_trait;
}

pub use async_trait::async_trait;

