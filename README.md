# 🌸 Kawai Monitor - Waifu-Themed Solana RPC Monitor 💕

A cute, waifu-themed Solana monitoring tool that watches accounts, transactions, and slots via RPC WebSockets. Get desktop notifications, CSV logging, and scam detection—all with adorable messages!

![Kawai Monitor](photo_5940298091359570915_x-removebg-preview.png)

## ✨ Features

- 🐾 **Multi-Account Monitoring** - Watch multiple Solana accounts simultaneously
- 😻 **Transaction Tracking** - Real-time transaction notifications
- 🌸 **Slot Updates** - Monitor validator slot progression
- ⚠️ **Scam Detection** - Basic heuristics to flag suspicious transactions
- 📊 **CSV Logging** - All events logged to `kawai_logs.csv`
- 🔔 **Desktop Notifications** - Get alerts for important events
- 💕 **Waifu-Themed** - Cute messages and emojis throughout!

## 🚀 Quick Start

### Option 1: Download Pre-built Binary

1. Go to [Releases](https://github.com/yourusername/kawai/releases)
2. Download the binary for your OS:
   - Windows: `kawai.exe`
   - Mac: `kawai-mac`
   - Linux: `kawai-linux`
3. Run it:
   ```bash
   ./kawai --rpc-url https://api.devnet.solana.com --accounts YOUR_PUBKEY
   ```

### Option 2: Build from Source

**Prerequisites:**
- Rust (install from https://rustup.rs/)

**Build:**
```bash
# Clone the repo
git clone https://github.com/yourusername/kawai.git
cd kawai

# Build
cargo build --release

# Run
./target/release/kawai --rpc-url https://api.devnet.solana.com
```

## 📖 Usage

### Basic Usage

```bash
# Monitor a specific account
./kawai --accounts YOUR_PUBKEY_HERE

# Monitor multiple accounts
./kawai --accounts pubkey1,pubkey2,pubkey3

# Enable scam detection
./kawai --accounts YOUR_PUBKEY --scam-detect

# Use a different RPC endpoint
./kawai --rpc-url https://api.mainnet-beta.solana.com --accounts YOUR_PUBKEY

# Disable desktop notifications
./kawai --accounts YOUR_PUBKEY --no-notifications
```

### Configuration File

Create `kawai_config.json` for persistent settings:

```json
{
  "rpc_url": "https://api.devnet.solana.com",
  "accounts": ["YourPubkeyHere1", "YourPubkeyHere2"],
  "scam_detect": true,
  "no_notifications": false
}
```

The tool will automatically load this config if present.

## 🎯 What It Monitors

- **Account Updates**: Balance changes, data modifications
- **Transactions**: All confirmed transactions on the network
- **Slot Updates**: Validator slot progression and status
- **Suspicious Activity**: Large transfers, potential scams (when enabled)

## 📊 Output

### Console Output
```
╔═══════════════════════════════════════════════════════╗
║     🌸  Kawai Monitor - Solana RPC Monitor  🌸        ║
║          Nyaa~ Ready to monitor Solana~ 💕           ║
╚═══════════════════════════════════════════════════════╝

🔗 Connecting to https://api.devnet.solana.com... 🐱
✅ Connected! Starting monitoring... 🎉

🐾 Kawaii Alert! Account ABC123... updated to 1000000 lamports on slot 12345!
😻 Purr~ Transaction DEF456... confirmed on slot 12346!
🌸 Slot update: Slot 12347 (parent: 12346)
```

### CSV Logging
All events are logged to `kawai_logs.csv`:
```csv
Type,Details,Slot/Timestamp
Account,Kawaii Alert! Account ABC123... updated,12345
Tx,Purr~ Transaction DEF456... confirmed,12346
Slot,Slot update: Slot 12347,12347
```

## 🛠️ Building for Different Platforms

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Mac (Intel)
cargo build --release --target x86_64-apple-darwin

# Mac (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

## 🎨 Customization

### Add More Waifu Messages

Edit `src/main.rs` and customize the messages:
- `println!("Nyaa~ ...")` - Loading messages
- `println!("🐾 Kawaii Alert! ...")` - Account updates
- `println!("😻 Purr~ ...")` - Transaction notifications
- `println!("🌸 ...")` - Slot updates

### Enhance Scam Detection

Modify the `is_suspicious()` function in `src/main.rs` to add:
- Blacklist checking
- Pattern matching
- Heuristic improvements
- Integration with scam databases

## 📝 Requirements

- Rust 1.75+ (for building from source)
- Internet connection (for RPC/WebSocket)
- Desktop notifications require:
  - Windows: Built-in support
  - Mac: macOS 10.9+
  - Linux: `libnotify` (usually pre-installed)

## 🐛 Troubleshooting

### Connection Errors
- Check your internet connection
- Verify the RPC URL is correct
- Try a different RPC endpoint (e.g., Helius, QuickNode)

### No Notifications
- Windows: Check notification settings
- Mac: Check System Preferences > Notifications
- Linux: Install `libnotify`: `sudo apt-get install libnotify-bin`

### Build Errors
```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build --release
```

## 🎁 Features Roadmap

- [ ] GUI version (using Tauri)
- [ ] Web dashboard
- [ ] Advanced scam detection patterns
- [ ] Price tracking integration
- [ ] Discord/Telegram notifications
- [ ] Historical data analysis

## 📄 License

MIT License - Feel free to use and modify!

## 💕 Credits

Built with love and waifu energy~ 🌸

---

**Nyaa~ Enjoy monitoring Solana! 💕🐾**
