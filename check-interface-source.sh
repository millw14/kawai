#!/bin/bash
# Check the actual interface crate source for the entry point

echo "=== Finding interface crate source ==="
INTERFACE_PATH=$(find ~/.cargo/registry/src -type d -name "solana-geyser-plugin-interface-1.18.26" 2>/dev/null | head -1)

if [ -z "$INTERFACE_PATH" ]; then
    echo "Interface crate not found. Try: cargo build first"
    exit 1
fi

echo "Found at: $INTERFACE_PATH"
echo ""

echo "=== Checking for example plugins ==="
find "$INTERFACE_PATH" -name "*.rs" -type f | grep -E "example|test" | head -5

echo ""
echo "=== Checking lib.rs for entry point info ==="
if [ -f "$INTERFACE_PATH/src/lib.rs" ]; then
    grep -A 5 -B 5 "create_plugin\|entry\|_create" "$INTERFACE_PATH/src/lib.rs" | head -30
fi

echo ""
echo "=== Checking geyser_plugin_interface.rs ==="
if [ -f "$INTERFACE_PATH/src/geyser_plugin_interface.rs" ]; then
    grep -A 10 "create_plugin\|entry\|_create" "$INTERFACE_PATH/src/geyser_plugin_interface.rs" | head -40
fi

