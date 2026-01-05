#!/bin/bash
# Find the correct entry point signature from the interface crate

echo "=== Finding interface crate ==="
INTERFACE_PATH=$(find ~/.cargo/registry/src -type d -name "solana-geyser-plugin-interface-1.18.26" 2>/dev/null | head -1)

if [ -z "$INTERFACE_PATH" ]; then
    echo "Not found, trying any version..."
    INTERFACE_PATH=$(find ~/.cargo/registry/src -type d -name "solana-geyser-plugin-interface-1.18*" 2>/dev/null | head -1)
fi

if [ -z "$INTERFACE_PATH" ]; then
    echo "Interface crate not found in registry"
    exit 1
fi

echo "Found: $INTERFACE_PATH"
echo ""

echo "=== Searching for entry point references ==="
grep -r "create_plugin\|_create" "$INTERFACE_PATH/src" 2>/dev/null | head -20

echo ""
echo "=== Checking trait definition ==="
grep -A 20 "pub trait GeyserPlugin" "$INTERFACE_PATH/src"/*.rs 2>/dev/null | head -40

echo ""
echo "=== Checking for example plugins ==="
find "$INTERFACE_PATH" -name "*.rs" | xargs grep -l "example\|Example" 2>/dev/null | head -5

