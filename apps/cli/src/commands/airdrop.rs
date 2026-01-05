//! Airdrop command handler

use colored::*;
use kawai_sdk::prelude::*;
use std::str::FromStr;

pub async fn handle(amount: f64, account: Option<String>) {
    println!("{} Requesting airdrop...", "🎁".bright_yellow());
    println!();

    let pubkey_str = match account {
        Some(pk) => pk,
        None => {
            match kawai_wallet::WalletManager::new() {
                Ok(manager) => {
                    match manager.load_default() {
                        Ok(wallet) => wallet.pubkey_string(),
                        Err(_) => {
                            println!("{} No account specified and no default wallet", "❌".bright_red());
                            return;
                        }
                    }
                }
                Err(_) => {
                    println!("{} No account specified", "❌".bright_red());
                    return;
                }
            }
        }
    };

    let pubkey = match Pubkey::from_str(&pubkey_str) {
        Ok(pk) => pk,
        Err(e) => {
            println!("{} Invalid pubkey: {}", "❌".bright_red(), e);
            return;
        }
    };

    let lamports = sol!(amount);

    match Kawai::devnet().await {
        Ok(kawai) => {
            match kawai.airdrop(&pubkey, lamports).await {
                Ok(result) => {
                    println!("{} Airdrop successful!", "✅".bright_green());
                    println!();
                    println!("   {} {} SOL", "Amount:".bright_cyan(), amount);
                    println!("   {} {}", "Account:".bright_cyan(), pubkey_str);
                    println!("   {} {}", "Signature:".bright_cyan(), result.signature.dimmed());
                }
                Err(e) => {
                    println!("{} Airdrop failed: {}", "❌".bright_red(), e);
                    println!();
                    println!("{}", "💡 Tips:".bright_yellow());
                    println!("   • Airdrop only works on devnet/testnet");
                    println!("   • Rate limits apply (1 SOL per request usually)");
                    println!("   • Try again in a few seconds");
                }
            }
        }
        Err(e) => {
            println!("{} Failed to connect to devnet: {}", "❌".bright_red(), e);
        }
    }
    println!();
}

