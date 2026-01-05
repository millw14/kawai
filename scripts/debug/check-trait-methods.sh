#!/bin/bash
# Check all required trait methods

INTERFACE_PATH=$(find ~/.cargo/registry/src -type d -name "solana-geyser-plugin-interface-1.18.26" 2>/dev/null | head -1)

if [ -z "$INTERFACE_PATH" ]; then
    echo "Interface crate not found"
    exit 1
fi

echo "=== Full trait definition ==="
grep -A 100 "pub trait GeyserPlugin" "$INTERFACE_PATH/src/geyser_plugin_interface.rs" | head -150

echo ""
echo "=== Entry point documentation ==="
grep -A 20 -B 5 "_create_plugin" "$INTERFACE_PATH/src/geyser_plugin_interface.rs" | head -40

