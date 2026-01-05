//! Transfer command handler

use colored::*;
use kawai_sdk::prelude::*;
use std::str::FromStr;

pub async fn handle(to: String, amount: f64, from: Option<String>) {
    println!("{} Preparing transfer...", "💸".bright_yellow());
    println!();

    // Get sender wallet
    let sender = match kawai_wallet::WalletManager::new() {
        Ok(manager) => {
            let wallet = match from {
                Some(name) => manager.load(&name),
                None => manager.load_default(),
            };
            match wallet {
                Ok(w) => w,
                Err(e) => {
                    println!("{} Failed to load wallet: {}", "❌".bright_red(), e);
                    return;
                }
            }
        }
        Err(e) => {
            println!("{} Failed to initialize wallet manager: {}", "❌".bright_red(), e);
            return;
        }
    };

    let to_pubkey = match Pubkey::from_str(&to) {
        Ok(pk) => pk,
        Err(e) => {
            println!("{} Invalid recipient pubkey: {}", "❌".bright_red(), e);
            return;
        }
    };

    let lamports = sol!(amount);

    println!("   {} {}", "From:".bright_cyan(), sender.pubkey_string());
    println!("   {} {}", "To:".bright_cyan(), to);
    println!("   {} {} SOL", "Amount:".bright_cyan(), amount);
    println!();

    match Kawai::devnet().await {
        Ok(kawai) => {
            // Check balance first
            match kawai.balance(&sender.pubkey()).await {
                Ok(balance) => {
                    if balance.lamports < lamports {
                        println!("{} Insufficient balance!", "❌".bright_red());
                        println!("   Have: {} SOL", balance.sol);
                        println!("   Need: {} SOL", amount);
                        return;
                    }
                }
                Err(e) => {
                    println!("{} Failed to check balance: {}", "❌".bright_red(), e);
                    return;
                }
            }

            match kawai.transfer(sender.inner(), &to_pubkey, lamports).await {
                Ok(result) => {
                    println!("{} Transfer successful!", "✅".bright_green());
                    println!();
                    println!("   {} {}", "Signature:".bright_cyan(), result.signature);
                    println!("   {} {}", "Status:".bright_cyan(), result.status);
                }
                Err(e) => {
                    println!("{} Transfer failed: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to connect: {}", "❌".bright_red(), e);
        }
    }
    println!();
}

