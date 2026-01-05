#!/bin/bash
# Remove Solana repository to stop SSL errors from blocking apt updates

echo "Removing Solana repository (can be re-added later)..."
sudo rm -f /etc/apt/sources.list.d/solana.list
echo "✅ Removed Solana repository"
echo ""
echo "Your apt updates will work normally now."
echo "You can install Solana CLI later when needed (it's not required to build the Rust code)."

