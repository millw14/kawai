//! Deploy command handler

use colored::*;
use std::path::Path;

pub async fn handle(path: Option<String>, cluster: String, keypair: Option<String>) {
    println!("{} Deploying program to {}...", "🚢".bright_yellow(), cluster.bright_cyan());
    println!();

    let project_path = path.unwrap_or_else(|| ".".to_string());

    // Check if it's an Anchor project
    let anchor_toml = Path::new(&project_path).join("Anchor.toml");
    if anchor_toml.exists() {
        println!("   Detected Anchor project, using Anchor deploy...");
        super::anchor::handle(crate::AnchorCommands::Deploy {
            path: project_path,
            cluster,
        }).await;
        return;
    }

    // Check for .so file
    let so_path = if project_path.ends_with(".so") {
        std::path::PathBuf::from(&project_path)
    } else {
        // Look in target/deploy
        let deploy_dir = Path::new(&project_path).join("target").join("deploy");
        if deploy_dir.exists() {
            // Find first .so file
            if let Ok(entries) = std::fs::read_dir(&deploy_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "so").unwrap_or(false) {
                        path
                    } else {
                        continue;
                    };
                }
            }
            println!("{} No .so file found in target/deploy", "❌".bright_red());
            println!("   Build your program first: kawai build program");
            return;
        } else {
            println!("{} No target/deploy directory found", "❌".bright_red());
            println!("   Build your program first: kawai build program");
            return;
        }
    };

    println!("   {} {}", "Program:".bright_cyan(), so_path.display());
    println!("   {} {}", "Cluster:".bright_cyan(), cluster);

    if let Some(kp) = &keypair {
        println!("   {} {}", "Keypair:".bright_cyan(), kp);
    }

    println!();
    
    // TODO: Implement actual deployment
    // This requires:
    // 1. Loading the program keypair
    // 2. Creating a buffer account
    // 3. Writing program data to buffer
    // 4. Deploying using BPF loader
    
    println!("{} Program deployment requires the Solana CLI or Anchor", "⚠️".bright_yellow());
    println!();
    println!("{}", "For now, use one of these methods:".bright_cyan());
    println!("   1. Anchor: kawai anchor deploy");
    println!("   2. WSL: wsl solana program deploy <program.so>");
    println!();
    println!("{}", "Native deployment coming in a future update!".dimmed());
}

