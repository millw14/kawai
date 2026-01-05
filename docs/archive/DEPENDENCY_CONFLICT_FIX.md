# Fixing Dependency Version Conflicts

## Current Issue

The build is failing with:
```
error[E0277]: the trait bound `solana_hash::Hash: serde::Deserialize<'de>` is not satisfied
```

This indicates a version mismatch between Solana crates. `solana-sysvar` 3.1.1 expects `solana_hash::Hash` to have serde support, but the version being used doesn't.

## Solutions

### Option 1: Use a Known Working Version

Try using a specific version that's known to work together:

```toml
solana-geyser-plugin-manager = "=3.1.5"
```

### Option 2: Clean and Rebuild

Sometimes Cargo's dependency resolution gets confused. Try:

```bash
cargo clean
rm -rf ~/.cargo/registry/cache
cargo build
```

### Option 3: Use Dependency Overrides

If the conflict persists, you can force compatible versions in `Cargo.toml`:

```toml
[patch.crates-io]
solana-hash = { version = "4.0.1", features = ["serde"] }
```

### Option 4: Check Solana Release Notes

Check what version combinations are officially supported:
- https://github.com/solana-labs/solana/releases
- Look for version compatibility notes

### Option 5: Use an Older Stable Version

If 3.1.5 has conflicts, try a known stable version:

```toml
solana-geyser-plugin-manager = "3.0.12"
```

But you'll need to ensure ALL Solana dependencies use 3.0.x series.

## Quick Test

Try building with verbose output to see the actual dependency tree:

```bash
cargo tree | grep solana
```

This will show you which versions are being used and help identify the conflict.

