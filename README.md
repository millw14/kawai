# Kawai Monitor

**A lightweight, real-time Solana blockchain monitoring tool for tracking accounts, transactions, and network activity.**

---

## Overview

Kawai Monitor is a standalone Rust application that connects to Solana RPC endpoints to monitor account balances, track transactions, and provide real-time alerts. Built for reliability and ease of use, it offers desktop notifications, CSV logging, and optional scam detection.

![Kawai Monitor](photo_5940298091359570915_x-removebg-preview.png)

---

## Features

| Feature | Description |
|---------|-------------|
| **Multi-Account Monitoring** | Track multiple Solana accounts simultaneously |
| **Real-Time Updates** | Receive instant notifications for balance changes and transactions |
| **Desktop Notifications** | System-level alerts for important events |
| **CSV Logging** | Comprehensive event logging for analysis and auditing |
| **Scam Detection** | Optional heuristics to flag suspicious transactions |
| **Slot Tracking** | Monitor validator slot progression |
| **Cross-Platform** | Works on Windows, macOS, and Linux |
| **Zero Configuration** | Run immediately with sensible defaults |

---

## Quick Start

### Download Pre-built Binary

1. Visit the [Releases](https://github.com/millw14/kawai/releases) page
2. Download the binary for your operating system:
   - **Windows**: `kawai.exe`
   - **macOS**: `kawai-mac`
   - **Linux**: `kawai-linux`
3. Run from terminal:

```bash
./kawai --accounts YOUR_PUBKEY_HERE
```

### Build from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) 1.75 or later

**Build Steps:**

```bash
# Clone the repository
git clone https://github.com/millw14/kawai.git
cd kawai

# Build the release binary
cargo build --release

# Run the application
./target/release/kawai --accounts YOUR_PUBKEY_HERE
```

---

## Usage

### Command Line Options

```bash
# Monitor a single account
./kawai --accounts YOUR_PUBKEY

# Monitor multiple accounts (comma-separated)
./kawai --accounts pubkey1,pubkey2,pubkey3

# Use a custom RPC endpoint
./kawai --rpc-url https://api.mainnet-beta.solana.com --accounts YOUR_PUBKEY

# Enable scam detection
./kawai --accounts YOUR_PUBKEY --scam-detect

# Adjust polling interval (seconds)
./kawai --accounts YOUR_PUBKEY --interval 10

# Disable desktop notifications
./kawai --accounts YOUR_PUBKEY --no-notifications
```

### Configuration File

Create a `kawai_config.json` file in the same directory for persistent settings:

```json
{
  "rpc_url": "https://api.devnet.solana.com",
  "accounts": [
    "YourPubkeyHere1",
    "YourPubkeyHere2"
  ],
  "scam_detect": true,
  "no_notifications": false,
  "interval": 5
}
```

The application will automatically load this configuration file if present.

---

## What It Monitors

- **Account Updates**: Balance changes and account data modifications
- **Transaction Activity**: Confirmed transactions on monitored accounts
- **Slot Progression**: Network slot updates and validator status
- **Suspicious Activity**: Large transfers and potentially fraudulent transactions (when enabled)

---

## Output Examples

### Console Output

```
╔═══════════════════════════════════════════════════════╗
║                                                       ║
║         🌸  Kawai Monitor - Your Helper  🌸         ║
║                                                       ║
╚═══════════════════════════════════════════════════════╝

Connecting to https://api.devnet.solana.com...
Connected! Starting monitoring...

Account ABC123... updated: 1000000 lamports (Slot: 12345)
Transaction DEF456... confirmed (Slot: 12346)
Slot update: 12347 (Parent: 12346)
```

### CSV Log Format

All events are automatically logged to `kawai_logs.csv`:

```csv
Type,Details,Slot/Timestamp
Account,Account ABC123... updated to 1000000 lamports,12345
Tx,Transaction DEF456... confirmed,12346
Slot,Slot update: 12347,12347
```

---

## Building for Different Platforms

### Windows

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### macOS (Intel)

```bash
cargo build --release --target x86_64-apple-darwin
```

### macOS (Apple Silicon)

```bash
cargo build --release --target aarch64-apple-darwin
```

### Linux

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

---

## Requirements

- **Rust**: Version 1.75 or later (for building from source)
- **Internet Connection**: Required for RPC endpoint communication
- **Desktop Notifications** (optional):
  - **Windows**: Built-in support
  - **macOS**: macOS 10.9 or later
  - **Linux**: `libnotify` library (typically pre-installed)

---

## Troubleshooting

### Connection Issues

- Verify your internet connection is active
- Confirm the RPC URL is correct and accessible
- Try alternative RPC endpoints (e.g., Helius, QuickNode, Triton)
- Check if your firewall is blocking connections

### Notification Problems

- **Windows**: Verify notification settings in Windows Settings > System > Notifications
- **macOS**: Check System Preferences > Notifications > kawai
- **Linux**: Install libnotify: `sudo apt-get install libnotify-bin` (Ubuntu/Debian) or equivalent

### Build Errors

```bash
# Update all dependencies
cargo update

# Clean build artifacts and rebuild
cargo clean
cargo build --release
```

### Common Issues

- **"Account not found"**: Verify the public key is correct and the account exists on the network
- **"RPC endpoint error"**: The endpoint may be rate-limited or unavailable; try a different endpoint
- **High CPU usage**: Increase the polling interval with `--interval` flag (default: 5 seconds)

---

## Customization

### Modifying Messages

Edit `src/main.rs` to customize console output messages and notification text.

### Enhancing Scam Detection

Modify the detection logic in `src/main.rs` to add:
- Blacklist checking
- Pattern recognition
- Advanced heuristics
- Integration with external scam databases

### Adding Features

The codebase is structured for easy extension. Key areas:
- `src/main.rs`: Main application logic and monitoring loop
- Configuration handling: Add new config options in the `Config` struct
- Notification system: Extend with custom notification handlers

---

## Roadmap

- [ ] Graphical user interface (Tauri-based)
- [ ] Web dashboard for remote monitoring
- [ ] Enhanced scam detection with ML-based patterns
- [ ] Price tracking and portfolio valuation
- [ ] Discord and Telegram notification integration
- [ ] Historical data analysis and reporting
- [ ] Multi-network support (extend to other chains)

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## Acknowledgments

Built with the Solana Rust SDK and Tokio async runtime.

---

**For questions, issues, or feature requests, please open an issue on GitHub.**
