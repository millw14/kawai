#!/bin/bash
# Troubleshooting script for Solana CLI installation SSL errors

echo "=== Troubleshooting Solana Installation ==="
echo ""

echo "1. Testing connectivity to Solana release server..."
if curl -I https://release.solana.com 2>&1 | grep -q "HTTP"; then
    echo "✅ Connection successful"
else
    echo "❌ Connection failed"
    echo ""
    echo "Trying alternative methods..."
fi
echo ""

echo "2. Checking CA certificates..."
if [ -f /etc/ssl/certs/ca-certificates.crt ]; then
    echo "✅ CA certificates found"
else
    echo "⚠️  CA certificates may be missing"
    echo "Try: sudo apt-get update && sudo apt-get install -y ca-certificates"
fi
echo ""

echo "3. Trying installation with insecure flag (not recommended for production)..."
echo "If the standard method fails, you can try:"
echo "  curl -k https://release.solana.com/stable/install | sh"
echo ""
echo "4. Alternative: Manual installation steps"
echo "  - Download from: https://github.com/solana-labs/solana/releases"
echo "  - Or use apt: sudo sh -c 'echo \"deb https://release.solana.com/stable/apt stable main\" > /etc/apt/sources.list.d/solana.list'"
echo "  - Then: sudo apt update && sudo apt install solana"
echo ""

echo "5. Testing DNS resolution..."
nslookup release.solana.com || echo "DNS lookup failed"

