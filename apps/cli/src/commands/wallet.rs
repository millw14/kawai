//! Wallet command handlers

use crate::WalletCommands;
use colored::*;
use kawai_wallet::{KawaiKeypair, Mnemonic, WalletManager};

pub async fn handle(cmd: WalletCommands) {
    match cmd {
        WalletCommands::Create { name } => create_wallet(&name).await,
        WalletCommands::Import { name, key } => import_wallet(&name, key).await,
        WalletCommands::List => list_wallets().await,
        WalletCommands::Show { name } => show_wallet(name).await,
        WalletCommands::Default { name } => set_default(&name).await,
        WalletCommands::Export { name } => export_wallet(&name).await,
        WalletCommands::Delete { name } => delete_wallet(&name).await,
        WalletCommands::Mnemonic { words } => generate_mnemonic(words).await,
    }
}

async fn create_wallet(name: &str) {
    println!("{} Creating wallet '{}'...", "🔑".bright_yellow(), name);
    
    match WalletManager::new() {
        Ok(mut manager) => {
            match manager.create(name) {
                Ok(keypair) => {
                    println!();
                    println!("{} Wallet created successfully!", "✅".bright_green());
                    println!();
                    println!("   {} {}", "Name:".bright_cyan(), name);
                    println!("   {} {}", "Pubkey:".bright_cyan(), keypair.pubkey_string());
                    println!();
                    println!("{}", "⚠️  Save your private key securely!".bright_yellow());
                }
                Err(e) => {
                    println!("{} Failed to create wallet: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize wallet manager: {}", "❌".bright_red(), e);
        }
    }
}

async fn import_wallet(name: &str, key: Option<String>) {
    let private_key = match key {
        Some(k) => k,
        None => {
            // Prompt for key
            println!("Enter private key (base58):");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        }
    };

    println!("{} Importing wallet '{}'...", "🔑".bright_yellow(), name);

    match WalletManager::new() {
        Ok(mut manager) => {
            match manager.import(name, &private_key) {
                Ok(keypair) => {
                    println!();
                    println!("{} Wallet imported successfully!", "✅".bright_green());
                    println!();
                    println!("   {} {}", "Name:".bright_cyan(), name);
                    println!("   {} {}", "Pubkey:".bright_cyan(), keypair.pubkey_string());
                }
                Err(e) => {
                    println!("{} Failed to import wallet: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize wallet manager: {}", "❌".bright_red(), e);
        }
    }
}

async fn list_wallets() {
    println!("{} Your Wallets", "🔑".bright_yellow());
    println!();

    match WalletManager::new() {
        Ok(manager) => {
            let wallets = manager.list();
            if wallets.is_empty() {
                println!("   No wallets found. Create one with: kawai wallet create <name>");
            } else {
                for wallet in wallets {
                    let default_marker = if wallet.is_default { " (default)" } else { "" };
                    println!(
                        "   {} {} {}",
                        if wallet.is_default { "→".bright_green() } else { " ".normal() },
                        wallet.name.bright_cyan(),
                        format!("{}{}", wallet.pubkey[..8].to_string() + "...", default_marker).dimmed()
                    );
                }
            }
        }
        Err(e) => {
            println!("{} Failed to list wallets: {}", "❌".bright_red(), e);
        }
    }
    println!();
}

async fn show_wallet(name: Option<String>) {
    match WalletManager::new() {
        Ok(manager) => {
            let wallet_name = name.or_else(|| {
                manager.list().iter().find(|w| w.is_default).map(|w| w.name.clone())
            });

            match wallet_name {
                Some(n) => {
                    if let Some(info) = manager.get_info(&n) {
                        println!();
                        println!("{} Wallet: {}", "🔑".bright_yellow(), info.name.bright_cyan());
                        println!();
                        println!("   {} {}", "Pubkey:".bright_cyan(), info.pubkey);
                        println!("   {} {}", "Default:".bright_cyan(), if info.is_default { "Yes" } else { "No" });
                        println!();
                    } else {
                        println!("{} Wallet '{}' not found", "❌".bright_red(), n);
                    }
                }
                None => {
                    println!("{} No wallet specified and no default wallet set", "❌".bright_red());
                }
            }
        }
        Err(e) => {
            println!("{} Failed to show wallet: {}", "❌".bright_red(), e);
        }
    }
}

async fn set_default(name: &str) {
    match WalletManager::new() {
        Ok(mut manager) => {
            match manager.set_default(name) {
                Ok(()) => {
                    println!("{} Default wallet set to '{}'", "✅".bright_green(), name);
                }
                Err(e) => {
                    println!("{} Failed to set default: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize wallet manager: {}", "❌".bright_red(), e);
        }
    }
}

async fn export_wallet(name: &str) {
    println!("{}", "⚠️  WARNING: Never share your private key!".bright_red());
    println!();

    match WalletManager::new() {
        Ok(manager) => {
            match manager.load(name) {
                Ok(keypair) => {
                    println!("{} Private key for '{}':", "🔑".bright_yellow(), name);
                    println!();
                    println!("   {}", keypair.to_base58());
                    println!();
                }
                Err(e) => {
                    println!("{} Failed to export wallet: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize wallet manager: {}", "❌".bright_red(), e);
        }
    }
}

async fn delete_wallet(name: &str) {
    println!("{} Deleting wallet '{}'...", "🗑️".bright_yellow(), name);

    match WalletManager::new() {
        Ok(mut manager) => {
            match manager.delete(name) {
                Ok(()) => {
                    println!("{} Wallet '{}' deleted", "✅".bright_green(), name);
                }
                Err(e) => {
                    println!("{} Failed to delete wallet: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize wallet manager: {}", "❌".bright_red(), e);
        }
    }
}

async fn generate_mnemonic(words: usize) {
    let mnemonic = match words {
        12 => Mnemonic::new_12(),
        24 => Mnemonic::new_24(),
        _ => {
            println!("{} Word count must be 12 or 24", "❌".bright_red());
            return;
        }
    };

    match mnemonic {
        Ok(m) => {
            println!();
            println!("{} New {}-word mnemonic:", "🌱".bright_green(), words);
            println!();
            println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_yellow());
            
            let words_vec = m.words();
            for (i, chunk) in words_vec.chunks(4).enumerate() {
                let line: Vec<String> = chunk.iter().enumerate().map(|(j, w)| {
                    format!("{}. {}", i * 4 + j + 1, w)
                }).collect();
                println!("║  {}  ║", line.join("  ").bright_white());
            }
            
            println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_yellow());
            println!();
            println!("{}", "⚠️  Write down these words and store them securely!".bright_red());
            println!("{}", "    Anyone with these words can access your funds.".dimmed());
            println!();
        }
        Err(e) => {
            println!("{} Failed to generate mnemonic: {}", "❌".bright_red(), e);
        }
    }
}

