# Kawai Monitor v0.1.0

**Initial Release**

Kawai Monitor is a lightweight, real-time Solana blockchain monitoring tool for tracking accounts, transactions, and network activity.

## Features

- **Multi-Account Monitoring**: Track multiple Solana accounts simultaneously
- **Real-Time Updates**: Receive instant notifications for balance changes and transactions
- **Desktop Notifications**: System-level alerts for important events
- **CSV Logging**: Comprehensive event logging to `kawai_logs.csv`
- **Scam Detection**: Optional heuristics to flag suspicious transactions
- **Slot Tracking**: Monitor validator slot progression
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Configuration File Support**: JSON-based configuration for persistent settings

## Quick Start

```bash
# Download the binary for your platform from the assets below
# Then run:
./kawai --accounts YOUR_PUBKEY_HERE
```

## Usage Example

```bash
# Monitor a single account
./kawai --accounts YOUR_PUBKEY

# Monitor multiple accounts
./kawai --accounts pubkey1,pubkey2,pubkey3

# Use a custom RPC endpoint
./kawai --rpc-url https://api.mainnet-beta.solana.com --accounts YOUR_PUBKEY

# Enable scam detection
./kawai --accounts YOUR_PUBKEY --scam-detect
```

## Requirements

- Windows, macOS, or Linux
- Internet connection for RPC endpoints
- Desktop notifications supported on all platforms

## Building from Source

```bash
git clone https://github.com/millw14/kawai.git
cd kawai
cargo build --release
./target/release/kawai --accounts YOUR_PUBKEY
```

## Documentation

See the [README.md](README.md) for complete documentation, troubleshooting, and usage examples.

---

**Full Changelog**: Initial release



