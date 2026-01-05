//! Balance command handler

use colored::*;
use kawai_sdk::prelude::*;

pub async fn handle(account: Option<String>, network: String) {
    let network: Network = match network.parse() {
        Ok(n) => n,
        Err(e) => {
            println!("{} Invalid network: {}", "❌".bright_red(), e);
            return;
        }
    };

    println!("{} Checking balance on {}...", "💰".bright_yellow(), network.name().bright_cyan());
    println!();

    let pubkey_str = match account {
        Some(pk) => pk,
        None => {
            // Try to get default wallet
            match kawai_wallet::WalletManager::new() {
                Ok(manager) => {
                    match manager.load_default() {
                        Ok(wallet) => wallet.pubkey_string(),
                        Err(_) => {
                            println!("{} No account specified and no default wallet", "❌".bright_red());
                            println!("   Use: kawai balance --account <PUBKEY>");
                            println!("   Or create a wallet: kawai wallet create <name>");
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

    match Kawai::connect(network).await {
        Ok(kawai) => {
            match kawai.balance_str(&pubkey_str).await {
                Ok(balance) => {
                    println!("   {} {}", "Account:".bright_cyan(), pubkey_str);
                    println!("   {} {} SOL", "Balance:".bright_cyan(), format!("{:.9}", balance.sol).bright_green());
                    println!("   {} {} lamports", "         ".dimmed(), balance.lamports.to_string().dimmed());
                }
                Err(e) => {
                    println!("{} Failed to get balance: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to connect to {}: {}", "❌".bright_red(), network.name(), e);
        }
    }
    println!();
}

