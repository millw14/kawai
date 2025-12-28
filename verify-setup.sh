#!/bin/bash
# Quick verification script for WSL/Linux setup

echo "=== Environment Check ==="
echo "OS Info:"
uname -a
echo ""

echo "=== Rust Check ==="
if command -v rustc &> /dev/null; then
    echo "✅ Rust is installed: $(rustc --version)"
    cargo --version
else
    echo "❌ Rust is not installed"
    echo "Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi
echo ""

echo "=== Solana CLI Check ==="
if command -v solana &> /dev/null; then
    echo "✅ Solana CLI is installed: $(solana --version)"
else
    echo "❌ Solana CLI is not installed"
    echo "Install with: sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
fi
echo ""

echo "=== Current Directory ==="
pwd
echo ""

echo "=== Checking for Rust Project ==="
if [ -f "Cargo.toml" ]; then
    echo "✅ Found Cargo.toml"
    echo "You can build with: cargo build"
else
    echo "⚠️  Cargo.toml not found in current directory"
    echo "Navigate to your project directory first"
fi

