#!/bin/bash
# Script to find available modules in solana-geyser-plugin-manager

echo "Checking crate structure..."
cd "/mnt/c/Users/1/Documents/milla projects/kawai"

# Try to extract module information from cargo metadata
cargo metadata --format-version 1 2>/dev/null | grep -i "geyser" | head -5

# Or check if we can list what's in the crate
echo ""
echo "Trying to build with verbose output to see module structure..."
cargo build 2>&1 | grep -i "module\|pub\|trait\|struct" | head -20

