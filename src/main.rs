use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use clap::Parser;
use csv::Writer;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use tokio::time::{sleep, Duration};
use notify_rust::Notification;

#[derive(Parser, Debug)]
#[command(name = "kawai")]
#[command(about = "🌸 Kawai Solana RPC Monitor - Cute waifu logs for your Solana accounts! 🐾")]
struct Args {
    /// RPC URL to connect to
    #[arg(long, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Comma-separated list of account pubkeys to monitor
    #[arg(long)]
    accounts: Option<String>,

    /// Enable basic scam detection (large balance changes)
    #[arg(long)]
    scam_detect: bool,

    /// Disable desktop notifications
    #[arg(long)]
    no_notifications: bool,

    /// Polling interval in seconds
    #[arg(long, default_value_t = 5)]
    interval: u64,
}

#[derive(serde::Deserialize)]
struct Config {
    rpc_url: Option<String>,
    accounts: Vec<String>,
    scam_detect: bool,
    no_notifications: bool,
    interval: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set UTF-8 encoding for proper Braille character display
    // This ensures the art renders correctly without user configuration
    // Setting these environment variables helps terminals display Unicode properly
    std::env::set_var("LANG", "en_US.UTF-8");
    std::env::set_var("LC_ALL", "en_US.UTF-8");
    std::env::set_var("LC_CTYPE", "en_US.UTF-8");
    
    // Rust's println! already handles UTF-8 correctly
    // The art will display if the terminal supports Unicode Braille characters
    
    // Print cute ASCII art (standard ASCII characters for maximum compatibility)
    // Using r##"..."## to avoid any parsing issues with special characters
    println!(r##"
╔═══════════════════════════════════════════════════════╗
║                                                       ║
║         🌸  Kawai Monitor - Your Helper  🌸         ║
║                                                       ║
╚═══════════════════════════════════════════════════════╝

                           _________________
                      ____/:::::::::::::::::\_____
                   __/::::::::::::::::::::::::::::\___
                 _/:::::::::::::::::::::::::::::::::::\__
               _/::::::::::::::::::::::::::::::::::::::::\_
              /::::::::::::::::::::::::::::::::::::::::::::\
             |::::::::::::::::::::::::::::::::::::::::::::::\
            /::::::::::::::::::::::::::::::::::::::::::::::::\
           |:::/.:::::::;:::::::::::::::::::::::::::::::::::::|
          /:::/.:::::::/..:::::::::::::::::::::::::::::::::::::\
         |:::|.::::::;/.::::::::::::::::::::::::::::::::::::::::|
         |::/.::::::/..:::::::;;'.::::::::::::::::::::::::::::::|
         |:|.::::/./.::::::;;/..:::::::::::::::::::::::::::::::::|
         `:|.:::|.|.:::::;/..;;;;;;-'.:;;;-':::::::::::::::::::::|
          \|.:::|.|.:::;/.;;/  -..::'''...:::::::::::::::::::::::|
           \;;::|.|.::/.;/--__      |::::::::::::::::::::::::::::|
              \;;\\::/|/ =-__ --_  /::::::::::::::::::::::::::::::
                  \\/    /|  -.   |.::::::::::::::::::::::::::::::
                 _.'    /|'   /=  ||::::::::::::::::::::::::::::::
             _.-'      //'        ||::::::::::::::::::::::::::::::
            (          -          `|::::::::::::::::::::::::::::::
             \                      \:::::::::::::::::::::::::::::
              |                      \:::::::::::PART:10::::::::::
             /                      __/:::::::::::::::::::::::::::
             \                   __/::;::;;::::::       ::::::::::
             (`                 /;;;;/::|  \::::  :___:  :::::::::
             |                 |'_,::::/ \ |:::: .|  |`. :::::::::
            .'                _/::::::/  / /:::: | \.' | ::. .::::
            |                /.::;;:-'_)/_/::::: `._| .' ::.D.::::
 ----.__    |               |.::|   \___/:::::::         ::.i.::::
 :::::::`----\_____          \:::\.-'::::::::::: ___:___ ::.z.::::
 ;;;;;:::::::::::::`------     \:::::::::::::::: ___|___ ::.z.::::
      `-------:::::::\         /::::::::::::::::  __|    ::.y.::::
 ___.--------'::::::::\       |::::::::::::::::: |  |-.  ::.n.::::
 :::::;;;:--:::::::::::|      /:::::::::::::::::  --'  ' ::.e.::::
 ----'  _,-:.:::::::::::\    |.:::::::::::::::::         ::.s.::::
     __/.::::::::::::::::|   |.:::::::::::::::::  :   :  ::.s.::::
  __/.:::;;::::::::;/.:::|   |.:::::::::::::::::  |    : ::. .::::
 /.::::;/ /.:::::;/ |.::::|   \_.:::::::::::::::  | .  | :::::::::
 :::::/  /.:::::/  /.:::::|     \__.::::::::::::  `.'  ' :::::::::
 ::::|  |.:::::/  /.:::::.|        \,::::::::::::       ::::::::::
 ::::|  |.::::|   |.:::::/|       __/::::::::::::::::::::::::::::
 \.:::\  \.:::|   |.::::||| __.--::::::::::::::::::::::GST:::::::
  \.:::\_ \.:::\   \.:::'/.:::::::::::::::::::::::::::::55min::::
   \.::::\ \.:::\   \.:::::::::::::::::::::::::::::::1995.10.01:

        🎀🎀🎀🎀🎀🎀🎀🎀❤️❤️❤️❤️

        "Nyaa~ I'll watch your accounts for you! 🐾"
        "Just tell me which ones to monitor~ 😻"
"##);

    let mut args = Args::parse();

    // Load config if exists
    if let Ok(file) = File::open("kawai_config.json") {
        println!("📝 Loading config from kawai_config.json...");
        let config: Config = serde_json::from_reader(file)?;
        args.rpc_url = config.rpc_url.unwrap_or(args.rpc_url);
        if !config.accounts.is_empty() {
            args.accounts = Some(config.accounts.join(","));
        }
        args.scam_detect = config.scam_detect;
        args.no_notifications = config.no_notifications;
        args.interval = config.interval.unwrap_or(args.interval);
        println!("✅ Config loaded! 🎉");
    }

    println!("🔗 Connecting to {}... 🐱", args.rpc_url);
    let rpc = RpcClient::new(args.rpc_url.clone());

    // CSV logger
    let mut csv_writer = Writer::from_path("kawai_logs.csv")?;
    csv_writer.write_record(&["Type", "Details", "Slot/Timestamp"])?;
    csv_writer.flush()?;

    println!("📊 Logging to kawai_logs.csv");
    println!();

    // Parse accounts to monitor
    let mut monitored_accounts: HashSet<Pubkey> = HashSet::new();
    if let Some(accs) = &args.accounts {
        for acc_str in accs.split(',') {
            let acc_str = acc_str.trim();
            match Pubkey::from_str(acc_str) {
                Ok(pubkey) => {
                    monitored_accounts.insert(pubkey);
                    println!("👀 Monitoring account: {} 🐾", acc_str);
                }
                Err(e) => {
                    println!("⚠️  Invalid pubkey '{}': {} - skipping", acc_str, e);
                }
            }
        }
    }

    if monitored_accounts.is_empty() {
        println!("💡 Tip: Use --accounts pubkey1,pubkey2 to monitor specific accounts");
        println!("📡 Monitoring slot updates only...");
    }

    println!();
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║  🎉 Yay~ Monitoring live! Press Ctrl+C to stop~ 🎉   ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();

    // Track previous states
    let mut last_slot = 0u64;
    let mut account_balances: std::collections::HashMap<Pubkey, u64> = std::collections::HashMap::new();

    // Main monitoring loop
    loop {
        // Check for Ctrl+C
        if tokio::signal::ctrl_c().await.is_ok() {
            break;
        }

        // Get current slot
        match rpc.get_slot() {
            Ok(current_slot) => {
                if current_slot > last_slot {
                    let msg = format!("🌸 Slot update: Slot {} (previous: {})", current_slot, last_slot);
                    println!("{}", msg);
                    let _ = csv_writer.write_record(&["Slot", &msg, &current_slot.to_string()]);
                    let _ = csv_writer.flush();
                    last_slot = current_slot;
                }
            }
            Err(e) => {
                println!("😿 Error getting slot: {}", e);
            }
        }

        // Monitor accounts
        for pubkey in monitored_accounts.iter() {
            match rpc.get_account(pubkey) {
                Ok(account) => {
                    let current_balance = account.lamports;
                    let pubkey_str = bs58::encode(pubkey.to_bytes()).into_string();
                    
                    if let Some(&prev_balance) = account_balances.get(pubkey) {
                        if current_balance != prev_balance {
                            let mut msg = format!(
                                "🐾 Kawaii Alert! Account {} balance changed: {} → {} lamports!",
                                pubkey_str,
                                prev_balance,
                                current_balance
                            );
                            
                            // Basic scam detection: large balance changes
                            if args.scam_detect {
                                let change = if current_balance > prev_balance {
                                    current_balance - prev_balance
                                } else {
                                    prev_balance - current_balance
                                };
                                
                                if change > 1_000_000_000 { // 1 SOL threshold
                                    msg.push_str(" ⚠️ SUSPICIOUS LARGE CHANGE!");
                                    if !args.no_notifications {
                                        Notification::new()
                                            .summary("🚨 Kawai Alert!")
                                            .body(&format!("Suspicious activity detected on {}", pubkey_str))
                                            .show()?;
                                    }
                                }
                            }
                            
                            println!("{}", msg);
                            let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
                            let _ = csv_writer.write_record(&["Account", &msg, &timestamp.to_string()]);
                            let _ = csv_writer.flush();
                            
                            if !args.no_notifications {
                                Notification::new()
                                    .summary("🐾 Kawai Alert!")
                                    .body(&format!("Account {} balance changed!", pubkey_str))
                                    .show()?;
                            }
                        }
                    }
                    
                    account_balances.insert(*pubkey, current_balance);
                }
                Err(e) => {
                    println!("😿 Error getting account {}: {}", bs58::encode(pubkey.to_bytes()).into_string(), e);
                }
            }
        }

        sleep(Duration::from_secs(args.interval)).await;
    }
    
    Ok(())
}
