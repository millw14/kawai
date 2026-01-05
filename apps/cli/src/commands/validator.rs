//! Validator command handlers
//!
//! Uses the native Windows validator by default - no Docker, no WSL!

use crate::ValidatorCommands;
use colored::*;
use kawai_validator::{
    NativeValidator,
    NativeValidatorConfig,
    LAMPORTS_PER_SOL,
};

pub async fn handle(cmd: ValidatorCommands) {
    match cmd {
        ValidatorCommands::Start { reset, port, backend } => {
            start_validator(reset, port, &backend).await;
        }
        ValidatorCommands::Stop => {
            stop_validator().await;
        }
        ValidatorCommands::Status => {
            validator_status().await;
        }
        ValidatorCommands::Logs { lines } => {
            show_logs(lines).await;
        }
        ValidatorCommands::Install { backend } => {
            install_info(&backend).await;
        }
    }
}

async fn start_validator(reset: bool, port: u16, backend: &str) {
    // Native is default - no external tools needed!
    if backend != "native" && backend != "auto" {
        println!("{} Note: Using native validator (no {} needed)", 
            "ℹ️".bright_cyan(), backend);
        println!();
    }
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║  🌸 Kawai Native Validator - Pure Windows Solana Runtime 🌸 ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", "No Docker. No WSL. No Linux. Just Windows.".bright_magenta());
    println!();
    
    // Build config
    let config = NativeValidatorConfig::default()
        .rpc_port(port)
        .reset(reset)
        .slot_time_ms(400); // Fast for dev
    
    println!("{} Starting native validator...", "🚀".bright_yellow());
    println!();
    println!("   {} {}", "RPC Port:".bright_cyan(), port);
    println!("   {} {}", "Reset:".bright_cyan(), reset);
    println!("   {} {:?}", "Ledger:".bright_cyan(), config.ledger_dir);
    println!();
    
    match NativeValidator::with_config(config) {
        Ok(validator) => {
            // Start validator (blocks until Ctrl+C)
            match validator.start().await {
                Ok(()) => {
                    println!("{} Validator stopped cleanly", "✅".bright_green());
                }
                Err(e) => {
                    println!("{} Validator error: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize validator: {}", "❌".bright_red(), e);
        }
    }
}

async fn stop_validator() {
    println!("{} Stopping validator...", "🛑".bright_yellow());
    println!();
    
    // Check if RPC is responding
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getHealth"
    });
    
    match client.post("http://127.0.0.1:8899")
        .json(&body)
        .send()
        .await 
    {
        Ok(response) if response.status().is_success() => {
            println!("{} Validator is running at http://127.0.0.1:8899", "⚠️".bright_yellow());
            println!();
            println!("   To stop: Press {} in the validator terminal", "Ctrl+C".bright_cyan());
            println!("   Or close the terminal window");
        }
        _ => {
            println!("{} No validator running at http://127.0.0.1:8899", "ℹ️".bright_cyan());
        }
    }
}

async fn validator_status() {
    println!("{} Validator Status", "📊".bright_yellow());
    println!();
    
    let client = reqwest::Client::new();
    
    // Check health
    let health_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getHealth"
    });
    
    match client.post("http://127.0.0.1:8899")
        .json(&health_body)
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await 
    {
        Ok(response) if response.status().is_success() => {
            println!("   {} {}", "Status:".bright_cyan(), "🟢 Running".bright_green());
            println!("   {} {}", "Type:".bright_cyan(), "Kawai Native (Pure Windows)");
            println!("   {} http://127.0.0.1:8899", "RPC:".bright_cyan());
            
            // Get slot
            let slot_body = serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSlot"
            });
            
            if let Ok(slot_response) = client.post("http://127.0.0.1:8899")
                .json(&slot_body)
                .send()
                .await
            {
                if let Ok(json) = slot_response.json::<serde_json::Value>().await {
                    if let Some(slot) = json.get("result") {
                        println!("   {} {}", "Slot:".bright_cyan(), slot);
                    }
                }
            }
            
            // Get version
            let version_body = serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getVersion"
            });
            
            if let Ok(version_response) = client.post("http://127.0.0.1:8899")
                .json(&version_body)
                .send()
                .await
            {
                if let Ok(json) = version_response.json::<serde_json::Value>().await {
                    if let Some(version) = json.get("result") {
                        if let Some(kawai_version) = version.get("kawai-validator") {
                            println!("   {} Kawai v{}", "Version:".bright_cyan(), kawai_version);
                        }
                    }
                }
            }
        }
        _ => {
            println!("   {} {}", "Status:".bright_cyan(), "🔴 Not running".bright_red());
            println!();
            println!("   Run '{}' to start", "kawai validator start".bright_green());
        }
    }
    println!();
}

async fn show_logs(lines: usize) {
    println!("{} Validator Logs", "📜".bright_yellow());
    println!();
    println!("   The native validator logs directly to the terminal.");
    println!("   Check the terminal where you ran 'kawai validator start'.");
    println!();
    println!("   To get the last {} transactions:", lines);
    println!();
    
    // Try to get recent signatures
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getEpochInfo"
    });
    
    match client.post("http://127.0.0.1:8899")
        .json(&body)
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await 
    {
        Ok(response) if response.status().is_success() => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(result) = json.get("result") {
                    println!("   {}", "Current Epoch Info:".bright_cyan());
                    println!("   Epoch: {}", result.get("epoch").unwrap_or(&serde_json::Value::Null));
                    println!("   Slot: {}", result.get("absoluteSlot").unwrap_or(&serde_json::Value::Null));
                    println!("   Transactions: {}", result.get("transactionCount").unwrap_or(&serde_json::Value::Null));
                }
            }
        }
        _ => {
            println!("   {} Validator not running", "⚠️".bright_yellow());
        }
    }
}

async fn install_info(backend: &str) {
    println!("{} Kawai Native Validator", "✨".bright_magenta());
    println!();
    println!("   {} No installation needed!", "🎉".bright_green());
    println!();
    println!("   The Kawai native validator is built directly into this CLI.");
    println!("   It runs as a pure Windows application - no Docker, no WSL,");
    println!("   no external dependencies required.");
    println!();
    println!("{}", "Quick Start:".bright_yellow());
    println!();
    println!("   {} Start validator:", "1.".bright_cyan());
    println!("      kawai validator start");
    println!();
    println!("   {} Configure your project:", "2.".bright_cyan());
    println!("      kawai config set --url http://127.0.0.1:8899");
    println!();
    println!("   {} Get test SOL:", "3.".bright_cyan());
    println!("      kawai airdrop 2 YOUR_WALLET_ADDRESS");
    println!();
    println!("   {} Check status:", "4.".bright_cyan());
    println!("      kawai validator status");
    println!();
    
    if backend == "docker" || backend == "wsl" {
        println!("{}", "Note:".bright_yellow());
        println!("   Docker and WSL backends are deprecated in Kawai.");
        println!("   The native validator is faster and simpler!");
    }
}
