#!/bin/bash
# Quick check script for Solana installation

echo "=== Checking Solana Installation ==="
echo ""

if command -v solana &> /dev/null; then
    echo "✅ Solana CLI is installed!"
    solana --version
    echo ""
    echo "Location: $(which solana)"
else
    echo "❌ Solana CLI is not installed"
    echo ""
    echo "Checking repository configuration..."
    if [ -f /etc/apt/sources.list.d/solana.list ]; then
        echo "✅ Solana repository file exists"
        cat /etc/apt/sources.list.d/solana.list
        echo ""
        echo "Try running:"
        echo "  sudo apt update"
        echo "  sudo apt install -y solana"
    else
        echo "❌ Solana repository not configured"
        echo "Run: sudo sh -c 'echo \"deb https://release.solana.com/stable/apt stable main\" > /etc/apt/sources.list.d/solana.list'"
    fi
fi

