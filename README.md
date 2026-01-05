# 🌸 Kawai

**Native Windows Solana Development Kit - No WSL Required**

[![License: MIT](https://img.shields.io/badge/License-MIT-pink.svg)](https://opensource.org/licenses/MIT)
[![Windows](https://img.shields.io/badge/platform-Windows-0078D6?logo=windows)](https://github.com/millw14/kawai)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)

---

## What is Kawai?

Kawai brings **full Solana development to Windows** without WSL, VMs, or Linux. Pure native Windows performance with a beautiful interface.

> **"Everything WSL Solana can do, but faster and prettier on Windows."**

```
┌─────────────────────────────────────────────────────────────┐
│                    🌸 KAWAI SDK 🌸                          │
│                                                             │
│   ✅ Native Windows binaries (no WSL)                       │
│   ✅ Beautiful CLI with colors & emojis                     │
│   ✅ Wallet management built-in                             │
│   ✅ One-command project setup                              │
│   ✅ Faster than WSL (no VM overhead)                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Quick Start

### Download & Install

```powershell
# Coming soon: winget install kawai

# For now, build from source:
git clone https://github.com/millw14/kawai.git
cd kawai
cargo build --release
```

### Basic Usage

```powershell
# Create a wallet
kawai wallet create main

# Check balance
kawai balance

# Request devnet SOL
kawai airdrop --amount 2

# Transfer SOL
kawai transfer <RECIPIENT_PUBKEY> 1.0

# Monitor accounts
kawai monitor --accounts <PUBKEY1>,<PUBKEY2>

# Initialize new project
kawai init my-solana-app
```

---

## Features

| Feature | Description |
|---------|-------------|
| 🔑 **Wallet Manager** | Create, import, export wallets. Mnemonic support. |
| 💰 **Balance Checking** | Check any account on any network |
| 🎁 **Airdrop** | Request devnet/testnet SOL instantly |
| 💸 **Transfers** | Send SOL with transaction confirmation |
| 👀 **Account Monitor** | Real-time balance tracking with alerts |
| 🚀 **Project Init** | Scaffold new Solana projects |
| 📊 **Network Info** | Slot, epoch, TPS, and more |
| ⚙️ **Configuration** | Persistent settings |

---

## SDK Usage (Rust)

Add Kawai to your project:

```toml
[dependencies]
kawai-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

Use it:

```rust
use kawai_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to devnet
    let kawai = Kawai::devnet().await?;

    // Create a wallet
    let wallet = KawaiKeypair::new();
    println!("🌸 New wallet: {}", wallet.pubkey());

    // Request airdrop
    kawai.airdrop(&wallet.pubkey(), sol!(2.0)).await?;

    // Check balance
    let balance = kawai.balance(&wallet.pubkey()).await?;
    println!("💰 Balance: {} SOL", balance.sol);

    Ok(())
}
```

---

## Why Kawai over WSL?

| Aspect | WSL2 | Kawai Native |
|--------|------|--------------|
| **Startup** | 2-5 sec VM boot | ⚡ Instant |
| **Memory** | 2-8GB VM overhead | 📉 App only |
| **File I/O** | Cross-FS penalty | 🚀 Native NTFS |
| **Network** | Virtual adapter | 🌐 Direct sockets |
| **Setup** | Complex | 📦 One install |

---

## Project Structure

```
kawai/
├── apps/
│   ├── cli/              # kawai command-line tool
│   └── desktop/          # GUI application (coming soon)
├── crates/
│   ├── kawai-sdk/        # Core Rust SDK
│   ├── kawai-wallet/     # Wallet management
│   └── kawai-rpc/        # RPC client
├── packages/             # JavaScript SDK (coming soon)
├── src/                  # Original monitor (legacy)
└── installer/            # Windows installer
```

---

## Roadmap

- [x] **v0.1** - Account monitoring, notifications, logging
- [x] **v0.2** - SDK structure, wallet management, CLI
- [ ] **v0.3** - Full transaction support, token operations
- [ ] **v0.4** - Desktop GUI (Tauri)
- [ ] **v0.5** - NFT tools, Anchor integration
- [ ] **v1.0** - Complete Windows Solana toolkit

---

## Commands Reference

### Wallet

```powershell
kawai wallet create <name>           # Create new wallet
kawai wallet import <name>           # Import from private key
kawai wallet list                    # List all wallets
kawai wallet show [name]             # Show wallet details
kawai wallet default <name>          # Set default wallet
kawai wallet export <name>           # Export private key
kawai wallet delete <name>           # Delete wallet
kawai wallet mnemonic --words 24     # Generate mnemonic
```

### Operations

```powershell
kawai balance [--account <pk>] [--network devnet]
kawai airdrop [--amount 1.0] [--account <pk>]
kawai transfer <to> <amount> [--from <wallet>]
kawai info [--network devnet]
```

### Monitoring

```powershell
kawai monitor --accounts <pk1>,<pk2> [--interval 5]
```

### Project

```powershell
kawai init <name> [--template basic|anchor|token]
kawai config show
kawai config network <devnet|testnet|mainnet>
kawai config rpc <url>
```

---

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- Windows 10/11

### Build

```powershell
# Clone
git clone https://github.com/millw14/kawai.git
cd kawai

# Build all
cargo build --release

# Or build specific package
cargo build -p kawai-sdk --release
cargo build -p kawai-cli --release

# Install CLI globally
cargo install --path apps/cli
```

### Test

```powershell
cargo test --workspace
```

---

## Contributing

We need help with:

- 🦀 **Rust** - Core SDK development
- 🎨 **Frontend** - Desktop UI (Tauri + React)
- 📚 **Docs** - Documentation and examples
- 🧪 **Testing** - Test coverage and CI

---

## License

MIT License - Build cool stuff! 🚀

---

<p align="center">
  <b>Kawai</b> - Solana Development, Windows Native 🌸
  <br>
  <sub>Made with 💖 for Windows developers</sub>
</p>
