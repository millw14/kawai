#!/bin/bash
# Manual Solana installation script (when SSL/TLS fails)
# This downloads from GitHub releases instead

set -e

echo "=== Manual Solana Installation (Alternative Method) ==="
echo ""

SOLANA_VERSION="1.18.26"  # Update this to latest stable version
ARCH="x86_64-unknown-linux-gnu"

echo "Step 1: Creating Solana install directory..."
mkdir -p ~/.local/share/solana/install/releases/${SOLANA_VERSION}
cd ~/.local/share/solana/install/releases/${SOLANA_VERSION}

echo "Step 2: Attempting to download Solana release..."
echo "Trying GitHub releases as alternative..."

# Try downloading from GitHub releases
GITHUB_URL="https://github.com/solana-labs/solana/releases/download/v${SOLANA_VERSION}/solana-release-${ARCH}.tar.bz2"

echo "URL: $GITHUB_URL"
echo ""
echo "If this fails, you can manually download from:"
echo "https://github.com/solana-labs/solana/releases"
echo ""

# Try wget first (sometimes works when curl fails)
if command -v wget &> /dev/null; then
    echo "Using wget..."
    wget -O solana-release.tar.bz2 "$GITHUB_URL" || {
        echo "❌ Download failed with wget"
        echo ""
        echo "Please manually download from:"
        echo "https://github.com/solana-labs/solana/releases/download/v${SOLANA_VERSION}/solana-release-${ARCH}.tar.bz2"
        echo "Save it as: ~/.local/share/solana/install/releases/${SOLANA_VERSION}/solana-release.tar.bz2"
        exit 1
    }
else
    echo "⚠️  wget not found. Please install it: sudo apt install wget"
    exit 1
fi

echo "Step 3: Extracting..."
tar jxf solana-release.tar.bz2

echo "Step 4: Creating symlink and adding to PATH..."
mkdir -p ~/.local/share/solana/install/active_release
ln -sfn ~/.local/share/solana/install/releases/${SOLANA_VERSION}/solana-release ~/.local/share/solana/install/active_release

# Add to PATH in .bashrc if not already there
if ! grep -q "~/.local/share/solana/install/active_release/bin" ~/.bashrc; then
    echo '' >> ~/.bashrc
    echo '# Solana CLI' >> ~/.bashrc
    echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
    echo "✅ Added Solana to PATH in ~/.bashrc"
fi

echo ""
echo "✅ Solana installed manually!"
echo "Close and reopen your terminal, or run: source ~/.bashrc"
echo "Then verify with: solana --version"

