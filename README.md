# 🌸 Kawai

**The Complete Solana Development Kit for Windows**

[![License: MIT](https://img.shields.io/badge/License-MIT-pink.svg)](https://opensource.org/licenses/MIT)
[![Windows](https://img.shields.io/badge/platform-Windows-0078D6?logo=windows)](https://github.com/millw14/kawai)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)

<p align="center">
  <img src="assets/images/kawai-chan.png" alt="Kawai-chan" width="300">
</p>

---

> **No WSL. No VM. No Linux. Just Windows.**

Kawai is a native Windows toolkit for Solana blockchain development. Build, test, and deploy Solana programs entirely on Windows with better performance than WSL.

```
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   🖥️  Local Test Validator    →  kawai validator start       ║
║   🔨  Program Compilation     →  kawai build program         ║
║   ⚓  Anchor Framework        →  kawai anchor init           ║
║   🚢  Deploy Programs         →  kawai deploy                ║
║   🔑  Wallet Management       →  kawai wallet create         ║
║   👀  Account Monitoring      →  kawai monitor               ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

---

## 📦 Installation

```powershell
# Clone and build
git clone https://github.com/millw14/kawai.git
cd kawai
cargo build --release

# Install CLI globally
cargo install --path apps/cli
```

---

## 🚀 Quick Start

```powershell
# 1. Create a wallet
kawai wallet create dev

# 2. Get devnet SOL
kawai airdrop --amount 2

# 3. Check balance
kawai balance

# 4. Create an Anchor project
kawai anchor init my-program

# 5. Build it
kawai anchor build

# 6. Start local validator & test
kawai validator start
kawai anchor test
```

---

## ✨ Features

### Development Tools

| Command | Description |
|---------|-------------|
| `kawai validator` | Native Windows test validator (no external tools!) |
| `kawai build` | Compile Solana programs to `.so` |
| `kawai anchor` | Full Anchor framework support |
| `kawai deploy` | Deploy programs to any cluster |
| `kawai toolchain` | Manage build tools |

### Wallet & Transactions

| Command | Description |
|---------|-------------|
| `kawai wallet` | Create, import, export wallets |
| `kawai balance` | Check SOL balance |
| `kawai airdrop` | Request devnet/testnet SOL |
| `kawai transfer` | Send SOL transactions |
| `kawai monitor` | Real-time account tracking |

### Project Management

| Command | Description |
|---------|-------------|
| `kawai init` | Scaffold new projects |
| `kawai config` | Manage settings |
| `kawai info` | Network statistics |

---

## 🖥️ Local Validator

Run a local Solana test validator on Windows:

```powershell
# Start native Windows validator
kawai validator start

# With options
kawai validator start --port 8899 --reset

# Check status
kawai validator status

# View epoch info
kawai validator logs

# Stop validator
kawai validator stop
```

**Pure Windows - No External Tools Required!**

The Kawai validator runs as a native Windows process.
No Docker Desktop. No WSL installation. No Linux VMs.
Just start it and go.

**Legacy Backends (deprecated):**
- **Docker** — Uses `solanalabs/solana` image (recommended)
- **WSL** — Uses Solana tools in WSL2
- **Cloud** — Falls back to devnet

---

## 🔨 Build Programs

Compile Solana programs without leaving Windows:

```powershell
# Build current directory
kawai build program .

# Release mode with verbose output
kawai build program . --release --verbose

# Force specific backend
kawai build program . --backend docker

# Check toolchain status
kawai build status
```

**Output:** `target/deploy/your_program.so`

---

## ⚓ Anchor Framework

Full Anchor support for Windows:

```powershell
# Create new project
kawai anchor init my-program

# Build
kawai anchor build

# Test (auto-starts validator)
kawai anchor test

# Deploy
kawai anchor deploy --cluster devnet

# Generate IDL
kawai anchor idl
```

**Project structure created:**
```
my-program/
├── programs/my-program/src/lib.rs
├── tests/my-program.ts
├── Anchor.toml
├── Cargo.toml
├── package.json
└── tsconfig.json
```

---

## 🔑 Wallet Management

```powershell
# Create wallet
kawai wallet create main

# Generate mnemonic
kawai wallet mnemonic --words 24

# Import from key
kawai wallet import trading --key <BASE58_KEY>

# List all
kawai wallet list

# Set default
kawai wallet default main

# Export (careful!)
kawai wallet export main
```

---

## 🔧 Toolchain Setup

```powershell
# Check what's installed
kawai toolchain status

# Install Solana in WSL
kawai toolchain install-solana --version 1.18.0

# Install Anchor in WSL
kawai toolchain install-anchor --version 0.29.0

# Pull Docker build image
kawai toolchain pull-docker
```

**Status output:**
```
🔧 Kawai Toolchain Status

   ✅ Docker Desktop: Installed
   ✅ WSL2: Available
      ✅ Solana: solana-cli 1.18.0
      ✅ Anchor: anchor-cli 0.29.0
   ✅ Cloud Build: Always available
```

---

## ⚡ Why Kawai?

| | WSL2 | Kawai |
|---|------|-------|
| **Startup** | 2-5 sec | Instant |
| **Memory** | 2-8 GB overhead | App only |
| **File I/O** | Cross-FS penalty | Native NTFS |
| **Network** | Virtual adapter | Direct sockets |
| **Setup** | Complex | One command |

---

## 📁 Project Structure

```
kawai/
├── apps/
│   ├── cli/                    # Command-line tool
│   └── desktop/                # GUI (coming soon)
├── crates/
│   ├── kawai-sdk/              # Core SDK
│   ├── kawai-wallet/           # Wallet management
│   ├── kawai-rpc/              # RPC client
│   ├── kawai-validator/        # Local validator
│   ├── kawai-build/            # Program compilation
│   └── kawai-anchor/           # Anchor integration
├── assets/                     # Images and misc files
├── config/examples/            # Example configurations
├── docs/
│   ├── archive/                # Old documentation
│   └── setup/                  # Setup guides
├── scripts/
│   ├── solana/                 # Solana installation scripts
│   └── debug/                  # Debug utilities
├── packages/                   # JS SDK (coming soon)
└── src/                        # Original monitor
```

---

## 🗺️ Roadmap

- [x] **v0.1** — Account monitoring, notifications
- [x] **v0.2** — SDK, wallet management, CLI
- [x] **v0.3** — Local validator, build tools, Anchor
- [ ] **v0.4** — Desktop GUI (Tauri)
- [ ] **v0.5** — Token tools, NFT support
- [ ] **v1.0** — Production ready

---

## 📖 Command Reference

<details>
<summary><b>Validator Commands</b></summary>

```powershell
kawai validator start [--reset] [--port 8899]    # Start native validator
kawai validator stop                              # Stop validator
kawai validator status                            # Check if running
kawai validator logs                              # View epoch info
```
*Native Windows implementation - no Docker/WSL needed!*
</details>

<details>
<summary><b>Build Commands</b></summary>

```powershell
kawai build program <path> [--release] [--backend auto|docker|wsl|cloud] [--verbose]
kawai build status
kawai build install-toolchain [--backend docker|wsl]
```
</details>

<details>
<summary><b>Anchor Commands</b></summary>

```powershell
kawai anchor init <name>
kawai anchor build [path] [--verbose]
kawai anchor test [path] [--skip-build]
kawai anchor deploy [path] [--cluster devnet|testnet|mainnet]
kawai anchor idl [path] [--output file.json]
```
</details>

<details>
<summary><b>Wallet Commands</b></summary>

```powershell
kawai wallet create <name>
kawai wallet import <name> [--key <BASE58>]
kawai wallet list
kawai wallet show [name]
kawai wallet default <name>
kawai wallet export <name>
kawai wallet delete <name>
kawai wallet mnemonic [--words 12|24]
```
</details>

<details>
<summary><b>Transaction Commands</b></summary>

```powershell
kawai balance [--account <pubkey>] [--network devnet|testnet|mainnet]
kawai airdrop [--amount 1.0] [--account <pubkey>]
kawai transfer <to> <amount> [--from <wallet>]
kawai deploy [path] [--cluster devnet] [--keypair <path>]
```
</details>

<details>
<summary><b>Other Commands</b></summary>

```powershell
kawai monitor --accounts <pk1>,<pk2> [--interval 5] [--rpc-url <url>]
kawai init <name> [--template basic|anchor|token]
kawai info [--network devnet]
kawai config show
kawai config network <devnet|testnet|mainnet>
kawai config rpc <url>
kawai toolchain status
kawai toolchain install-solana [--version 1.18.0]
kawai toolchain install-anchor [--version 0.29.0]
kawai toolchain pull-docker [--image <name>]
```
</details>

---

## 🛠️ Building from Source

```powershell
# Prerequisites: Rust 1.75+, Windows 10/11

git clone https://github.com/millw14/kawai.git
cd kawai

# Build everything
cargo build --release

# Build specific crate
cargo build -p kawai-sdk --release
cargo build -p kawai-validator --release

# Run tests
cargo test --workspace

# Install CLI
cargo install --path apps/cli
```

---

## 🤝 Contributing

Help wanted:

| Area | Skills |
|------|--------|
| 🦀 Core | Rust, Solana |
| 🎨 Desktop | Tauri, React |
| 📚 Docs | Technical writing |
| 🧪 Testing | CI/CD, QA |

---

## 📄 License

MIT — Build cool stuff!

---

<p align="center">
<b>Kawai</b> — Solana Development, Windows Native 🌸
<br>
<sub>Made with 💖 for Windows developers who don't want to touch Linux</sub>
</p>
