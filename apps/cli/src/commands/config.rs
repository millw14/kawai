//! Configuration command handler

use crate::ConfigCommands;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KawaiConfig {
    pub default_network: String,
    pub custom_rpc_url: Option<String>,
}

impl KawaiConfig {
    pub fn load() -> Self {
        let path = config_path();
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self {
                default_network: "devnet".to_string(),
                custom_rpc_url: None,
            }
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kawai")
        .join("config.json")
}

pub async fn handle(cmd: ConfigCommands) {
    match cmd {
        ConfigCommands::Show => show_config().await,
        ConfigCommands::Network { network } => set_network(&network).await,
        ConfigCommands::Rpc { url } => set_rpc(&url).await,
    }
}

async fn show_config() {
    let config = KawaiConfig::load();
    
    println!("{} Kawai Configuration", "⚙️".bright_yellow());
    println!();
    println!("   {} {}", "Network:".bright_cyan(), config.default_network);
    if let Some(rpc) = &config.custom_rpc_url {
        println!("   {} {}", "Custom RPC:".bright_cyan(), rpc);
    }
    println!();
    println!("   {} {}", "Config file:".dimmed(), config_path().display());
    println!();
}

async fn set_network(network: &str) {
    let valid_networks = ["devnet", "testnet", "mainnet", "localhost"];
    
    if !valid_networks.contains(&network) {
        println!("{} Invalid network: {}", "❌".bright_red(), network);
        println!("   Valid options: {}", valid_networks.join(", "));
        return;
    }

    let mut config = KawaiConfig::load();
    config.default_network = network.to_string();
    
    match config.save() {
        Ok(()) => {
            println!("{} Default network set to: {}", "✅".bright_green(), network.bright_cyan());
        }
        Err(e) => {
            println!("{} Failed to save config: {}", "❌".bright_red(), e);
        }
    }
}

async fn set_rpc(url: &str) {
    let mut config = KawaiConfig::load();
    config.custom_rpc_url = Some(url.to_string());
    
    match config.save() {
        Ok(()) => {
            println!("{} Custom RPC URL set to: {}", "✅".bright_green(), url.bright_cyan());
        }
        Err(e) => {
            println!("{} Failed to save config: {}", "❌".bright_red(), e);
        }
    }
}

