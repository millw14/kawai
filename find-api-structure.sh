#!/bin/bash
# Script to help discover the solana-geyser-plugin-manager API structure

echo "=== Finding solana-geyser-plugin-manager crate location ==="
CRATE_PATH=$(find ~/.cargo/registry/src -type d -name "solana-geyser-plugin-manager-*" 2>/dev/null | head -1)

if [ -z "$CRATE_PATH" ]; then
    echo "Crate not found in registry. Try: cargo build first"
    exit 1
fi

echo "Found at: $CRATE_PATH"
echo ""
echo "=== Checking lib.rs for module exports ==="
if [ -f "$CRATE_PATH/src/lib.rs" ]; then
    echo "Modules found in lib.rs:"
    grep -E "^pub mod|^mod " "$CRATE_PATH/src/lib.rs" | head -20
    echo ""
    echo "Public re-exports:"
    grep -E "^pub use" "$CRATE_PATH/src/lib.rs" | head -20
else
    echo "lib.rs not found, checking for main.rs or other entry points..."
    ls -la "$CRATE_PATH/src/" | head -10
fi

echo ""
echo "=== Listing all source files ==="
find "$CRATE_PATH/src" -name "*.rs" -type f | head -20

echo ""
echo "=== To see full documentation, run: ==="
echo "cargo doc --open"
echo ""
echo "Then navigate to: solana_geyser_plugin_manager"

