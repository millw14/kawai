//! Monitor command handler (original kawai feature)

use colored::*;
use kawai_sdk::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

pub async fn handle(accounts: String, rpc_url: String, interval: u64) {
    println!("{}", "╔═══════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║         🌸  Kawai Monitor - Live Tracking  🌸        ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    let pubkeys: Vec<Pubkey> = accounts
        .split(',')
        .filter_map(|s| Pubkey::from_str(s.trim()).ok())
        .collect();

    if pubkeys.is_empty() {
        println!("{} No valid pubkeys provided", "❌".bright_red());
        return;
    }

    println!("{} Connecting to {}...", "🔗".bright_cyan(), rpc_url);
    
    let kawai = match Kawai::new(&rpc_url).connect(Network::Custom).await {
        Ok(k) => k,
        Err(_) => Kawai::new(&rpc_url),
    };

    println!("{} Monitoring {} account(s)", "👀".bright_yellow(), pubkeys.len());
    for pk in &pubkeys {
        println!("   • {}", pk.to_string().dimmed());
    }
    println!();
    println!("{} Press Ctrl+C to stop", "💡".dimmed());
    println!();

    let mut previous_balances: HashMap<String, u64> = HashMap::new();

    loop {
        for pubkey in &pubkeys {
            match kawai.balance(pubkey).await {
                Ok(balance) => {
                    let pk_str = pubkey.to_string();
                    let short_pk = format!("{}...{}", &pk_str[..4], &pk_str[pk_str.len()-4..]);
                    
                    if let Some(prev) = previous_balances.get(&pk_str) {
                        if *prev != balance.lamports {
                            let diff = balance.lamports as i64 - *prev as i64;
                            let change = if diff > 0 {
                                format!("+{:.9} SOL", diff as f64 / 1_000_000_000.0).bright_green()
                            } else {
                                format!("{:.9} SOL", diff as f64 / 1_000_000_000.0).bright_red()
                            };
                            println!(
                                "{} {} Balance changed: {} ({})",
                                "💫".bright_yellow(),
                                short_pk.bright_cyan(),
                                format!("{:.9} SOL", balance.sol).bright_white(),
                                change
                            );
                        }
                    } else {
                        println!(
                            "{} {} Balance: {} SOL",
                            "💰".bright_yellow(),
                            short_pk.bright_cyan(),
                            format!("{:.9}", balance.sol).bright_white()
                        );
                    }
                    
                    previous_balances.insert(pk_str, balance.lamports);
                }
                Err(e) => {
                    println!("{} Error checking {}: {}", "❌".bright_red(), pubkey, e);
                }
            }
        }

        sleep(Duration::from_secs(interval)).await;
    }
}

