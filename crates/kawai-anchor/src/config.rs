//! Anchor configuration handling

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Anchor.toml configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorConfig {
    /// Feature flags
    #[serde(default)]
    pub features: Features,

    /// Program configurations by cluster
    #[serde(default)]
    pub programs: HashMap<String, HashMap<String, String>>,

    /// Registry configuration
    #[serde(default)]
    pub registry: Registry,

    /// Provider configuration
    #[serde(default)]
    pub provider: Provider,

    /// Scripts
    #[serde(default)]
    pub scripts: HashMap<String, String>,

    /// Test configuration
    #[serde(default)]
    pub test: TestConfig,

    /// Workspace configuration
    #[serde(default)]
    pub workspace: Workspace,
}

impl Default for AnchorConfig {
    fn default() -> Self {
        Self {
            features: Features::default(),
            programs: HashMap::new(),
            registry: Registry::default(),
            provider: Provider::default(),
            scripts: HashMap::new(),
            test: TestConfig::default(),
            workspace: Workspace::default(),
        }
    }
}

impl AnchorConfig {
    /// Load from Anchor.toml
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let anchor_toml = path.as_ref().join("Anchor.toml");
        if !anchor_toml.exists() {
            return Err(Error::NotAnchorProject);
        }

        let content = fs::read_to_string(&anchor_toml)?;
        let config: AnchorConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save to Anchor.toml
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let anchor_toml = path.as_ref().join("Anchor.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::ConfigError(e.to_string()))?;
        fs::write(anchor_toml, content)?;
        Ok(())
    }

    /// Get program ID for a cluster
    pub fn program_id(&self, program: &str, cluster: &str) -> Option<&String> {
        self.programs
            .get(cluster)
            .and_then(|progs| progs.get(program))
    }

    /// Set program ID for a cluster
    pub fn set_program_id(&mut self, program: &str, cluster: &str, id: &str) {
        self.programs
            .entry(cluster.to_string())
            .or_default()
            .insert(program.to_string(), id.to_string());
    }

    /// Get all program names
    pub fn program_names(&self) -> Vec<String> {
        self.programs
            .values()
            .flat_map(|m| m.keys().cloned())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
}

/// Feature flags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Features {
    #[serde(default)]
    pub seeds: bool,
    #[serde(default = "default_true")]
    pub skip_lint: bool,
}

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub url: String,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            url: "https://api.apr.dev".to_string(),
        }
    }
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub cluster: String,
    pub wallet: String,
}

impl Default for Provider {
    fn default() -> Self {
        Self {
            cluster: "devnet".to_string(),
            wallet: "~/.config/solana/id.json".to_string(),
        }
    }
}

/// Test configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestConfig {
    #[serde(default)]
    pub startup_wait: u64,
    #[serde(default)]
    pub shutdown_wait: u64,
}

/// Workspace configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

fn default_true() -> bool {
    true
}

/// Create a new Anchor.toml for a project
pub fn create_anchor_toml(
    project_name: &str,
    program_id: &str,
) -> AnchorConfig {
    let mut config = AnchorConfig::default();

    // Add program to devnet
    config.set_program_id(project_name, "devnet", program_id);
    config.set_program_id(project_name, "localnet", program_id);

    // Set default scripts
    config.scripts.insert(
        "test".to_string(),
        "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts".to_string(),
    );

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AnchorConfig::default();
        assert_eq!(config.provider.cluster, "devnet");
    }

    #[test]
    fn test_program_id() {
        let mut config = AnchorConfig::default();
        config.set_program_id("my_program", "devnet", "11111111111111111111111111111111");
        
        assert_eq!(
            config.program_id("my_program", "devnet"),
            Some(&"11111111111111111111111111111111".to_string())
        );
    }
}

