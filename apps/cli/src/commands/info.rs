//! Network info command handler

use colored::*;
use kawai_sdk::prelude::*;

pub async fn handle(network: String) {
    let network: Network = match network.parse() {
        Ok(n) => n,
        Err(e) => {
            println!("{} Invalid network: {}", "❌".bright_red(), e);
            return;
        }
    };

    println!("{} Network Information: {}", "📊".bright_yellow(), network.name().bright_cyan());
    println!();

    match Kawai::connect(network).await {
        Ok(kawai) => {
            match kawai.stats().await {
                Ok(stats) => {
                    println!("   {} {}", "Slot:".bright_cyan(), stats.slot.to_string().bright_white());
                    println!("   {} {}", "Epoch:".bright_cyan(), stats.epoch);
                    println!("   {} {}", "Block Height:".bright_cyan(), stats.block_height);
                    if let Some(tps) = stats.tps {
                        println!("   {} {:.2} tx/s", "TPS:".bright_cyan(), tps);
                    }
                    println!();
                    println!("   {} {}", "RPC:".dimmed(), kawai.rpc_url().dimmed());
                }
                Err(e) => {
                    println!("{} Failed to get network info: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to connect to {}: {}", "❌".bright_red(), network.name(), e);
        }
    }
    println!();
}

