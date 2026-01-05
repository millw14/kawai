//! Kawai CLI - Native Windows Solana Development Tools
//!
//! A beautiful CLI for Solana development on Windows.
//! No WSL required!

use clap::{Parser, Subcommand};
use colored::*;

mod commands;

/// 🌸 Kawai - Native Windows Solana Development Kit
#[derive(Parser)]
#[command(name = "kawai")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🔑 Wallet management
    #[command(subcommand)]
    Wallet(WalletCommands),

    /// 💰 Check account balance
    Balance {
        /// Account public key (or use default wallet)
        #[arg(short, long)]
        account: Option<String>,

        /// Network (devnet, testnet, mainnet)
        #[arg(short, long, default_value = "devnet")]
        network: String,
    },

    /// 🎁 Request airdrop (devnet/testnet only)
    Airdrop {
        /// Amount in SOL
        #[arg(short, long, default_value_t = 1.0)]
        amount: f64,

        /// Account public key (or use default wallet)
        #[arg(short, long)]
        account: Option<String>,
    },

    /// 💸 Transfer SOL
    Transfer {
        /// Recipient public key
        to: String,

        /// Amount in SOL
        amount: f64,

        /// From wallet name (or use default)
        #[arg(short, long)]
        from: Option<String>,
    },

    /// 📊 Network information
    Info {
        /// Network (devnet, testnet, mainnet)
        #[arg(short, long, default_value = "devnet")]
        network: String,
    },

    /// 👀 Monitor accounts (original kawai feature)
    Monitor {
        /// Accounts to monitor (comma-separated)
        #[arg(short, long)]
        accounts: String,

        /// RPC URL
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc_url: String,

        /// Polling interval in seconds
        #[arg(short, long, default_value_t = 5)]
        interval: u64,
    },

    /// 🚀 Initialize a new Solana project
    Init {
        /// Project name
        name: String,

        /// Project template (basic, anchor, token)
        #[arg(short, long, default_value = "basic")]
        template: String,
    },

    /// ⚙️ Configuration
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Create a new wallet
    Create {
        /// Wallet name
        name: String,
    },

    /// Import wallet from private key
    Import {
        /// Wallet name
        name: String,

        /// Private key (base58)
        #[arg(short, long)]
        key: Option<String>,
    },

    /// List all wallets
    List,

    /// Show wallet details
    Show {
        /// Wallet name
        name: Option<String>,
    },

    /// Set default wallet
    Default {
        /// Wallet name
        name: String,
    },

    /// Export wallet private key
    Export {
        /// Wallet name
        name: String,
    },

    /// Delete a wallet
    Delete {
        /// Wallet name
        name: String,
    },

    /// Generate new mnemonic
    Mnemonic {
        /// Word count (12 or 24)
        #[arg(short, long, default_value_t = 12)]
        words: usize,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Set default network
    Network {
        /// Network name
        network: String,
    },

    /// Set default RPC URL
    Rpc {
        /// RPC URL
        url: String,
    },
}

fn print_banner() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║                                                       ║".bright_magenta());
    println!("{}", "║     🌸  KAWAI - Solana Dev Kit for Windows  🌸      ║".bright_magenta());
    println!("{}", "║              No WSL. Pure Performance.                ║".bright_magenta());
    println!("{}", "║                                                       ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".bright_magenta());
    println!();
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Wallet(wallet_cmd) => {
            commands::wallet::handle(wallet_cmd).await;
        }
        Commands::Balance { account, network } => {
            commands::balance::handle(account, network).await;
        }
        Commands::Airdrop { amount, account } => {
            commands::airdrop::handle(amount, account).await;
        }
        Commands::Transfer { to, amount, from } => {
            commands::transfer::handle(to, amount, from).await;
        }
        Commands::Info { network } => {
            commands::info::handle(network).await;
        }
        Commands::Monitor { accounts, rpc_url, interval } => {
            commands::monitor::handle(accounts, rpc_url, interval).await;
        }
        Commands::Init { name, template } => {
            commands::init::handle(name, template).await;
        }
        Commands::Config(config_cmd) => {
            commands::config::handle(config_cmd).await;
        }
    }
}

