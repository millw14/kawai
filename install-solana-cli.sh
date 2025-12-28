#!/bin/bash
# Install Solana CLI - try multiple methods

echo "=== Installing Solana CLI ==="
echo ""

# Method 1: Try the official installer (if SSL works)
echo "Method 1: Trying official installer..."
if sh -c "$(curl -sSfL https://release.solana.com/stable/install)"; then
    echo "✅ Solana CLI installed successfully!"
    echo "Close and reopen your terminal, or run: source ~/.cargo/env"
    exit 0
fi

echo "⚠️  Official installer failed (SSL issue)"
echo ""

# Method 2: Try with updated CA certificates first
echo "Method 2: Updating CA certificates and retrying..."
sudo apt-get update
sudo apt-get install -y ca-certificates
sudo update-ca-certificates

if sh -c "$(curl -sSfL https://release.solana.com/stable/install)"; then
    echo "✅ Solana CLI installed successfully!"
    exit 0
fi

echo "⚠️  Still failing, trying alternative methods..."
echo ""

# Method 3: Manual download from GitHub
echo "Method 3: Manual installation from GitHub releases..."
echo "Visit: https://github.com/solana-labs/solana/releases"
echo "Download the latest release for Linux x86_64"
echo "Extract and add to PATH"
echo ""

# Method 4: Use apt repository (if available)
echo "Method 4: Trying apt repository..."
if ! grep -q "solana" /etc/apt/sources.list.d/*.list 2>/dev/null; then
    echo "Adding Solana repository..."
    sudo sh -c 'echo "deb https://release.solana.com/stable/apt stable main" > /etc/apt/sources.list.d/solana.list'
    sudo apt update
    sudo apt install -y solana
    if command -v solana &> /dev/null; then
        echo "✅ Solana CLI installed via apt!"
        exit 0
    fi
fi

echo ""
echo "❌ All installation methods failed"
echo ""
echo "Alternative: You can test the plugin by:"
echo "1. Copying libkawai.so to a Linux machine with Solana installed"
echo "2. Using a Docker container with Solana"
echo "3. Installing Solana CLI on a different network (mobile hotspot)"
echo "4. Waiting for SSL/network issues to resolve"


