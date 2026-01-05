# Build Instructions

## Prerequisites

- Rust 1.75+ (install from https://rustup.rs/)
- Internet connection for downloading dependencies

## Build Commands

### Development Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

The binary will be at: `target/release/kawai` (or `kawai.exe` on Windows)

## Cross-Platform Building

### Windows
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### Mac (Intel)
```bash
cargo build --release --target x86_64-apple-darwin
```

### Mac (Apple Silicon)
```bash
cargo build --release --target aarch64-apple-darwin
```

### Linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

## First Build

The first build will take 5-10 minutes as it downloads and compiles all dependencies (~200+ crates). Subsequent builds are much faster.

## Troubleshooting

**Build errors?**
```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build --release
```

**Missing system libraries?**
- Linux: `sudo apt-get install libssl-dev pkg-config`
- Mac: Usually works out of the box
- Windows: Install Visual Studio Build Tools

