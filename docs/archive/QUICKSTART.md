# 🚀 Quick Start Guide

## Build and Run

```bash
# Build the project
cargo build --release

# Run with default settings (devnet)
./target/release/kawai

# Monitor specific accounts
./target/release/kawai --accounts YOUR_PUBKEY_HERE

# Enable scam detection
./target/release/kawai --accounts YOUR_PUBKEY --scam-detect

# Use mainnet
./target/release/kawai --rpc-url https://api.mainnet-beta.solana.com --accounts YOUR_PUBKEY
```

## Configuration File

Create `kawai_config.json`:

```json
{
  "rpc_url": "https://api.devnet.solana.com",
  "accounts": ["YourPubkey1", "YourPubkey2"],
  "scam_detect": true,
  "no_notifications": false
}
```

## Features

- ✅ Multi-account monitoring
- ✅ Slot updates
- ✅ Desktop notifications
- ✅ CSV logging (kawai_logs.csv)
- ✅ Scam detection
- ✅ Auto-reconnect on errors

## Troubleshooting

**Connection errors?**
- Check your internet connection
- Try a different RPC endpoint
- Use a paid RPC like Helius for better reliability

**No notifications?**
- Windows: Check notification settings
- Mac: System Preferences > Notifications
- Linux: Install `libnotify-bin`

