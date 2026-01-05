//! Validator command handlers

use crate::ValidatorCommands;
use colored::*;
use kawai_validator::{Validator, ValidatorConfig, config::BackendPreference};

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
            install_validator(&backend).await;
        }
    }
}

async fn start_validator(reset: bool, port: u16, backend: &str) {
    println!("{} Starting local validator...", "🖥️".bright_yellow());
    println!();

    let backend_pref = match backend {
        "docker" => BackendPreference::Docker,
        "wsl" => BackendPreference::Wsl,
        "cloud" => BackendPreference::Cloud,
        _ => BackendPreference::Auto,
    };

    let config = ValidatorConfig::default()
        .rpc_port(port)
        .reset(reset)
        .backend(backend_pref);

    match Validator::with_config(config).await {
        Ok(validator) => {
            println!("   {} {}", "Backend:".bright_cyan(), validator.backend_name());
            
            match validator.start().await {
                Ok(()) => {
                    println!();
                    println!("{} Validator started!", "✅".bright_green());
                    println!();
                    println!("   {} {}", "RPC URL:".bright_cyan(), validator.rpc_url());
                    println!("   {} {}", "WebSocket:".bright_cyan(), validator.ws_url());
                    println!();
                    println!("{}", "💡 Use 'kawai validator stop' to stop".dimmed());
                }
                Err(e) => {
                    println!("{} Failed to start validator: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize validator: {}", "❌".bright_red(), e);
            println!();
            println!("{}", "💡 Tips:".bright_yellow());
            println!("   • Install Docker Desktop for the easiest setup");
            println!("   • Or enable WSL2 and install Solana tools");
            println!("   • Run 'kawai validator install' to set up");
        }
    }
}

async fn stop_validator() {
    println!("{} Stopping validator...", "🛑".bright_yellow());

    match Validator::auto().await {
        Ok(validator) => {
            match validator.stop().await {
                Ok(()) => {
                    println!("{} Validator stopped", "✅".bright_green());
                }
                Err(e) => {
                    println!("{} Failed to stop validator: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} No validator running or error: {}", "⚠️".bright_yellow(), e);
        }
    }
}

async fn validator_status() {
    println!("{} Validator Status", "📊".bright_yellow());
    println!();

    match Validator::auto().await {
        Ok(validator) => {
            let running = validator.is_running().await;
            
            if running {
                println!("   {} {}", "Status:".bright_cyan(), "🟢 Running".bright_green());
                println!("   {} {}", "Backend:".bright_cyan(), validator.backend_name());
                println!("   {} {}", "RPC:".bright_cyan(), validator.rpc_url());
                
                if let Ok(slot) = validator.slot().await {
                    println!("   {} {}", "Slot:".bright_cyan(), slot);
                }
                
                if let Some(uptime) = validator.uptime().await {
                    println!("   {} {:?}", "Uptime:".bright_cyan(), uptime);
                }
            } else {
                println!("   {} {}", "Status:".bright_cyan(), "🔴 Not running".bright_red());
            }
        }
        Err(e) => {
            println!("   {} {}", "Status:".bright_cyan(), "🔴 Not running".bright_red());
            println!("   {} {}", "Note:".dimmed(), e.to_string().dimmed());
        }
    }
    println!();
}

async fn show_logs(lines: usize) {
    println!("{} Validator Logs (last {} lines)", "📜".bright_yellow(), lines);
    println!();

    match Validator::docker() {
        Ok(validator) => {
            match validator.logs(lines).await {
                Ok(logs) => {
                    println!("{}", logs);
                }
                Err(e) => {
                    println!("{} Failed to get logs: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Logs only available for Docker backend: {}", "⚠️".bright_yellow(), e);
        }
    }
}

async fn install_validator(backend: &str) {
    println!("{} Installing validator for {}...", "📦".bright_yellow(), backend);
    println!();

    match backend {
        "docker" => {
            println!("   Pulling Docker image...");
            
            let status = tokio::process::Command::new("docker")
                .args(["pull", "solanalabs/solana:v1.18.0"])
                .status()
                .await;

            match status {
                Ok(s) if s.success() => {
                    println!();
                    println!("{} Docker image ready!", "✅".bright_green());
                    println!();
                    println!("   Run 'kawai validator start' to start the validator");
                }
                Ok(_) => {
                    println!("{} Failed to pull Docker image", "❌".bright_red());
                    println!("   Make sure Docker Desktop is running");
                }
                Err(e) => {
                    println!("{} Docker not available: {}", "❌".bright_red(), e);
                    println!();
                    println!("{}", "💡 Install Docker Desktop from:".bright_yellow());
                    println!("   https://www.docker.com/products/docker-desktop");
                }
            }
        }
        "wsl" => {
            println!("   Installing Solana tools in WSL...");
            
            let script = r#"
                curl -sSfL "https://release.solana.com/v1.18.0/install" | sh
                echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
            "#;
            
            let status = tokio::process::Command::new("wsl")
                .args(["bash", "-c", script])
                .status()
                .await;

            match status {
                Ok(s) if s.success() => {
                    println!();
                    println!("{} Solana installed in WSL!", "✅".bright_green());
                }
                Ok(_) => {
                    println!("{} Installation failed", "❌".bright_red());
                }
                Err(e) => {
                    println!("{} WSL not available: {}", "❌".bright_red(), e);
                    println!();
                    println!("{}", "💡 Enable WSL2:".bright_yellow());
                    println!("   wsl --install");
                }
            }
        }
        _ => {
            println!("{} Unknown backend: {}", "❌".bright_red(), backend);
            println!("   Use 'docker' or 'wsl'");
        }
    }
}

