//! Kawai CLI - Native Windows Solana Development Tools
//!
//! A complete CLI for Solana development on Windows.
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

    // ========== NEW COMMANDS ==========

    /// 🖥️ Local validator management
    #[command(subcommand)]
    Validator(ValidatorCommands),

    /// 🔨 Build Solana programs
    #[command(subcommand)]
    Build(BuildCommands),

    /// ⚓ Anchor framework commands
    #[command(subcommand)]
    Anchor(AnchorCommands),

    /// 🚢 Deploy programs
    Deploy {
        /// Path to .so file or project directory
        path: Option<String>,

        /// Target cluster (devnet, testnet, mainnet, localnet)
        #[arg(short, long, default_value = "devnet")]
        cluster: String,

        /// Program keypair path
        #[arg(short, long)]
        keypair: Option<String>,
    },

    /// ⚙️ Configuration
    #[command(subcommand)]
    Config(ConfigCommands),

    /// 🔧 Toolchain management
    #[command(subcommand)]
    Toolchain(ToolchainCommands),
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Create a new wallet
    Create { name: String },
    /// Import wallet from private key
    Import {
        name: String,
        #[arg(short, long)]
        key: Option<String>,
    },
    /// List all wallets
    List,
    /// Show wallet details
    Show { name: Option<String> },
    /// Set default wallet
    Default { name: String },
    /// Export wallet private key
    Export { name: String },
    /// Delete a wallet
    Delete { name: String },
    /// Generate new mnemonic
    Mnemonic {
        #[arg(short, long, default_value_t = 12)]
        words: usize,
    },
}

#[derive(Subcommand)]
enum ValidatorCommands {
    /// Start local test validator
    Start {
        /// Reset ledger on start
        #[arg(short, long)]
        reset: bool,

        /// RPC port
        #[arg(long, default_value_t = 8899)]
        port: u16,

        /// Backend (auto, docker, wsl, cloud)
        #[arg(short, long, default_value = "auto")]
        backend: String,
    },
    /// Stop local validator
    Stop,
    /// Check validator status
    Status,
    /// View validator logs
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value_t = 50)]
        lines: usize,
    },
    /// Install validator (Docker image or WSL tools)
    Install {
        /// Backend to install for (docker, wsl)
        #[arg(short, long, default_value = "docker")]
        backend: String,
    },
}

#[derive(Subcommand)]
enum BuildCommands {
    /// Build the program
    Program {
        /// Project directory
        #[arg(default_value = ".")]
        path: String,

        /// Release mode
        #[arg(short, long)]
        release: bool,

        /// Backend (auto, docker, wsl, cloud)
        #[arg(short, long, default_value = "auto")]
        backend: String,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Check toolchain status
    Status,
    /// Install build toolchain
    InstallToolchain {
        /// Backend (docker, wsl)
        #[arg(short, long, default_value = "docker")]
        backend: String,
    },
}

#[derive(Subcommand)]
enum AnchorCommands {
    /// Initialize new Anchor project
    Init {
        /// Project name
        name: String,
    },
    /// Build Anchor project
    Build {
        /// Project directory
        #[arg(default_value = ".")]
        path: String,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run Anchor tests
    Test {
        /// Project directory
        #[arg(default_value = ".")]
        path: String,

        /// Skip build before test
        #[arg(long)]
        skip_build: bool,
    },
    /// Deploy Anchor program
    Deploy {
        /// Project directory
        #[arg(default_value = ".")]
        path: String,

        /// Target cluster
        #[arg(short, long, default_value = "devnet")]
        cluster: String,
    },
    /// Generate IDL
    Idl {
        /// Project directory
        #[arg(default_value = ".")]
        path: String,

        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set default network
    Network { network: String },
    /// Set default RPC URL
    Rpc { url: String },
}

#[derive(Subcommand)]
enum ToolchainCommands {
    /// Show toolchain status
    Status,
    /// Install Solana in WSL
    InstallSolana {
        /// Solana version
        #[arg(short, long, default_value = "1.18.0")]
        version: String,
    },
    /// Install Anchor in WSL
    InstallAnchor {
        /// Anchor version
        #[arg(short, long, default_value = "0.29.0")]
        version: String,
    },
    /// Pull Docker build image
    PullDocker {
        /// Docker image
        #[arg(short, long, default_value = "projectserum/build:v0.27.0")]
        image: String,
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
        Commands::Validator(cmd) => {
            commands::validator::handle(cmd).await;
        }
        Commands::Build(cmd) => {
            commands::build::handle(cmd).await;
        }
        Commands::Anchor(cmd) => {
            commands::anchor::handle(cmd).await;
        }
        Commands::Deploy { path, cluster, keypair } => {
            commands::deploy::handle(path, cluster, keypair).await;
        }
        Commands::Config(config_cmd) => {
            commands::config::handle(config_cmd).await;
        }
        Commands::Toolchain(cmd) => {
            commands::toolchain::handle(cmd).await;
        }
    }
}
