#!/bin/bash
# Install Solana CLI from the downloaded installer file

echo "=== Installing Solana CLI from downloaded file ==="
echo ""

# Make the installer executable
chmod +x solana-install-init-x86_64-unknown-linux-gnu

# Run the installer
echo "Running Solana installer..."
./solana-install-init-x86_64-unknown-linux-gnu

# The installer should add Solana to PATH
# Add to PATH manually if needed
if [ -d "$HOME/.local/share/solana/install/active_release/bin" ]; then
    echo ""
    echo "✅ Solana installed!"
    echo ""
    echo "Adding to PATH..."
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
    
    # Add to .bashrc if not already there
    if ! grep -q "solana/install/active_release/bin" ~/.bashrc; then
        echo '' >> ~/.bashrc
        echo '# Solana CLI' >> ~/.bashrc
        echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
        echo "✅ Added Solana to PATH in ~/.bashrc"
    fi
    
    echo ""
    echo "Verifying installation..."
    source ~/.bashrc 2>/dev/null || true
    solana --version
    
    echo ""
    echo "✅ Installation complete!"
    echo "Close and reopen your terminal, or run: source ~/.bashrc"
else
    echo ""
    echo "⚠️  Installation may have completed, but bin directory not found"
    echo "Check: ~/.local/share/solana/install/"
    echo "Or run the installer again"
fi


