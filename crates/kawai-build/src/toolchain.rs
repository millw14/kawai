//! Toolchain management for Solana development

use crate::error::{Error, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

/// Toolchain manager
pub struct Toolchain {
    /// Installation directory
    install_dir: PathBuf,
}

impl Toolchain {
    /// Create a new toolchain manager
    pub fn new() -> Self {
        Self {
            install_dir: Self::default_install_dir(),
        }
    }

    /// Get default installation directory
    pub fn default_install_dir() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kawai")
            .join("toolchain")
    }

    /// Check if Docker-based toolchain is available
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

    /// Check if WSL toolchain is available
    pub async fn wsl_available() -> bool {
        // Check WSL exists
        let wsl_ok = Command::new("wsl")
            .arg("--status")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false);

        if !wsl_ok {
            return false;
        }

        // Check if Solana tools are installed
        Command::new("wsl")
            .args(["which", "solana"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Install Solana tools in WSL
    pub async fn install_wsl_solana(&self, version: &str) -> Result<()> {
        println!("📦 Installing Solana {} in WSL...", version);

        let install_script = format!(
            r#"
            set -e
            curl -sSfL "https://release.solana.com/v{}/install" | sh
            export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
            echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
            solana --version
            "#,
            version
        );

        let status = Command::new("wsl")
            .args(["bash", "-c", &install_script])
            .status()
            .await?;

        if !status.success() {
            return Err(Error::ToolchainNotFound);
        }

        println!("✅ Solana {} installed in WSL", version);
        Ok(())
    }

    /// Install Anchor in WSL
    pub async fn install_wsl_anchor(&self, version: &str) -> Result<()> {
        println!("📦 Installing Anchor {} in WSL...", version);

        let install_script = format!(
            r#"
            set -e
            cargo install --git https://github.com/coral-xyz/anchor --tag v{} anchor-cli --locked
            anchor --version
            "#,
            version
        );

        let status = Command::new("wsl")
            .args(["bash", "-c", &install_script])
            .status()
            .await?;

        if !status.success() {
            return Err(Error::ToolchainNotFound);
        }

        println!("✅ Anchor {} installed in WSL", version);
        Ok(())
    }

    /// Pull Docker build image
    pub async fn pull_docker_image(&self, image: &str) -> Result<()> {
        println!("📦 Pulling Docker image: {}...", image);

        let status = Command::new("docker")
            .args(["pull", image])
            .status()
            .await?;

        if !status.success() {
            return Err(Error::DockerNotAvailable);
        }

        println!("✅ Docker image ready: {}", image);
        Ok(())
    }

    /// Get toolchain status
    pub async fn status(&self) -> ToolchainStatus {
        ToolchainStatus {
            docker: Self::docker_available().await,
            wsl: Self::wsl_available().await,
            wsl_solana: self.check_wsl_solana_version().await,
            wsl_anchor: self.check_wsl_anchor_version().await,
        }
    }

    /// Check Solana version in WSL
    async fn check_wsl_solana_version(&self) -> Option<String> {
        let output = Command::new("wsl")
            .args(["solana", "--version"])
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            Some(version.trim().to_string())
        } else {
            None
        }
    }

    /// Check Anchor version in WSL
    async fn check_wsl_anchor_version(&self) -> Option<String> {
        let output = Command::new("wsl")
            .args(["anchor", "--version"])
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            Some(version.trim().to_string())
        } else {
            None
        }
    }
}

impl Default for Toolchain {
    fn default() -> Self {
        Self::new()
    }
}

/// Toolchain status
#[derive(Debug, Clone)]
pub struct ToolchainStatus {
    /// Docker is available
    pub docker: bool,
    /// WSL is available
    pub wsl: bool,
    /// Solana version in WSL
    pub wsl_solana: Option<String>,
    /// Anchor version in WSL
    pub wsl_anchor: Option<String>,
}

impl ToolchainStatus {
    /// Check if any build backend is available
    pub fn has_backend(&self) -> bool {
        self.docker || (self.wsl && self.wsl_solana.is_some())
    }

    /// Get the best available backend
    pub fn best_backend(&self) -> Option<&'static str> {
        if self.docker {
            Some("Docker")
        } else if self.wsl && self.wsl_solana.is_some() {
            Some("WSL")
        } else {
            None
        }
    }
}

impl std::fmt::Display for ToolchainStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Toolchain Status:")?;
        writeln!(f, "  Docker: {}", if self.docker { "✅ Available" } else { "❌ Not found" })?;
        writeln!(f, "  WSL: {}", if self.wsl { "✅ Available" } else { "❌ Not found" })?;
        
        if let Some(v) = &self.wsl_solana {
            writeln!(f, "  Solana (WSL): ✅ {}", v)?;
        } else if self.wsl {
            writeln!(f, "  Solana (WSL): ❌ Not installed")?;
        }

        if let Some(v) = &self.wsl_anchor {
            writeln!(f, "  Anchor (WSL): ✅ {}", v)?;
        } else if self.wsl {
            writeln!(f, "  Anchor (WSL): ❌ Not installed")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_toolchain_status() {
        let toolchain = Toolchain::new();
        let status = toolchain.status().await;
        // Just verify it runs
        println!("{}", status);
    }
}

