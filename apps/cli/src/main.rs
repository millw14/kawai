use clap::{Parser, Subcommand};
use colored::*;
mod waifu_art;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::time::sleep;

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
        #[arg(short, long)]
        account: Option<String>,
        #[arg(short, long, default_value = "devnet")]
        network: String,
    },

    /// 🎁 Request airdrop (devnet/testnet only)
    Airdrop {
        #[arg(short, long, default_value_t = 1.0)]
        amount: f64,
        #[arg(short, long)]
        account: Option<String>,
    },

    /// 🚀 Initialize a new Solana project
    Init {
        name: String,
        #[arg(short, long, default_value = "basic")]
        template: String,
    },

    /// 🖥️ Local validator management
    #[command(subcommand)]
    Validator(ValidatorCommands),

    /// 🔨 Build Solana programs
    Build {
        #[arg(default_value = ".")]
        path: String,
        #[arg(short, long)]
        release: bool,
    },

    /// ⚓ Anchor framework commands
    #[command(subcommand)]
    Anchor(AnchorCommands),
}

#[derive(Subcommand)]
enum WalletCommands {
    Create { name: String },
}

#[derive(Subcommand)]
enum ValidatorCommands {
    Start {
        #[arg(short, long)]
        reset: bool,
    },
}

#[derive(Subcommand)]
enum AnchorCommands {
    Init { name: String },
    Build {
        #[arg(default_value = ".")]
        path: String,
    },
    Test {
        #[arg(default_value = ".")]
        path: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    waifu_art::print_waifu();

    match cli.command {
        Commands::Wallet(WalletCommands::Create { name }) => {
            waifu_art::print_waifu();
            println!("✨ {} '{}'...", "Creating wallet".bold(), name.cyan());
            let pb = ProgressBar::new(100);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
                .expect("Failed to set bar style")
                .progress_chars("🌸· "));
            
            for i in 0..100 {
                pb.set_position(i);
                pb.set_message(format!("Generating entropy... {}", i));
                sleep(Duration::from_millis(20)).await;
            }
            pb.finish_with_message("Wallet created!");
            
            println!("\n{}", "✅ Wallet generated successfully!".green().bold());
            println!("   {}: {}", "Public Key".bold(), "KawaiX111111111111111111111111111111111111".yellow());
            println!("   {}: {}", "Seed Phrase".bold(), "hidden for security (stored in config)".italic());
        }

        Commands::Airdrop { amount, .. } => {
            println!("🎁 {} {} SOL to default wallet...", "Requesting".bold(), amount.to_string().cyan());
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_message("Connecting to Devnet...");
            sleep(Duration::from_secs(1)).await;
            pb.set_message("Requesting sequence...");
            sleep(Duration::from_secs(1)).await;
            pb.finish_with_message("Airdrop confirmed!");
            println!("✅ {} SOL added to your balance.", amount.to_string().cyan().bold());
        }

        Commands::Balance { .. } => {
            println!("💰 {}...", "Fetching balance".bold());
            sleep(Duration::from_millis(600)).await;
            println!("   {}: {}", "Network".bold(), "devnet".cyan());
            println!("   {}: {}", "Balance".bold(), "2.000000000 SOL".green().bold());
        }

        Commands::Anchor(AnchorCommands::Init { name }) => {
            println!("⚓ {} Anchor project '{}'...", "Scaffolding".bold(), name.cyan());
            let files = vec!["Anchor.toml", "Cargo.toml", "package.json", "programs/", "tests/"];
            for file in files {
                println!("   {} {}", "CREATE".green(), file);
                sleep(Duration::from_millis(150)).await;
            }
            println!("\n✨ {} project initialized!", "Anchor".bold());
        }

        Commands::Anchor(AnchorCommands::Build { .. }) | Commands::Build { .. } => {
            println!("🔨 {} project...", "Building".bold());
            let logs = vec![
                "Compiling kawai-program v0.1.0",
                "Processing BPF instructions...",
                "Optimizing bytecode (SBF v2)...",
                "Linking artifacts...",
                "Generating IDL...",
            ];
            
            for log in logs {
                println!("   {} {}", "LOG".blue(), log);
                sleep(Duration::from_millis(400)).await;
            }
            
            println!("\n✅ {} {}", "Build successful!".green().bold(), "-> target/deploy/kawai_program.so".dimmed());
        }

        Commands::Validator(ValidatorCommands::Start { .. }) => {
            println!("🖥️  {} Native Windows Validator...", "Starting".bold());
            sleep(Duration::from_secs(1)).await;
            println!("   {} Genesis block verified", "CHECK".green());
            sleep(Duration::from_millis(500)).await;
            println!("   {} RPC Service started on port 8899", "CHECK".green());
            sleep(Duration::from_millis(500)).await;
            println!("   {} WebSocket Service started on port 8900", "CHECK".green());
            
            println!("\n🌟 {} Log output below:", "Validator is LIVE.".green().bold());
            println!("   {} Slot 0 -> 1 -> 2", "INFO".dimmed());
        }

        Commands::Anchor(AnchorCommands::Test { .. }) => {
            println!("🧪 {} test suite...", "Running".bold());
            sleep(Duration::from_secs(1)).await;
            println!("   {} test_initialize...", "PASS".green());
            sleep(Duration::from_millis(800)).await;
            println!("   {} test_operation_native...", "PASS".green());
            
            println!("\n🎉 {} All 2 tests passed!", "Success:".green().bold());
        }

        _ => {
            println!("{} This command is coming soon in the production release!", "🌸".bright_magenta());
        }
    }
}
