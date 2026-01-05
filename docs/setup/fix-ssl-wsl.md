# Fixing SSL/TLS Errors in WSL

If you're getting SSL/TLS errors when trying to install Solana (or other software), try these solutions:

## Solution 1: Update CA Certificates

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates
sudo update-ca-certificates
```

Then try installing again.

## Solution 2: Check Windows Firewall/Antivirus

Windows firewall or antivirus software may be interfering with WSL's network connections:

1. **Temporarily disable Windows Firewall** (just for testing)
2. **Check antivirus settings** - some antivirus software blocks SSL connections
3. **Add WSL to Windows Firewall exceptions**

## Solution 3: Network/Proxy Issues

If you're behind a corporate proxy or firewall:

1. **Try from a different network** (mobile hotspot) to test
2. **Configure proxy in WSL** if needed:
   ```bash
   # In ~/.bashrc or ~/.profile
   export http_proxy=http://proxy-server:port
   export https_proxy=http://proxy-server:port
   ```

## Solution 4: Manual Installation

Instead of using apt or the installer script, download Solana manually:

1. **Download from GitHub releases:**
   - Go to: https://github.com/solana-labs/solana/releases
   - Download: `solana-release-x86_64-unknown-linux-gnu.tar.bz2`
   - Extract and add to PATH

2. **Or use the manual install script** (see `manual-solana-install.sh`)

## Solution 5: Check WSL Network Configuration

```bash
# Check DNS resolution
nslookup release.solana.com

# Check connectivity
ping -c 3 release.solana.com

# Check SSL certificate
openssl s_client -connect release.solana.com:443 -showcerts
```

## Solution 6: Use Different DNS

If DNS is the issue, try using Google DNS:

```bash
# Edit resolv.conf
sudo nano /etc/resolv.conf

# Add:
nameserver 8.8.8.8
nameserver 8.8.4.4
```

**Note:** WSL may overwrite this on restart. To make it permanent, disable auto-generation in Windows.

## Quick Test

To test if SSL is working at all:

```bash
curl -I https://www.google.com
```

If this also fails, it's a broader SSL/network configuration issue in WSL.

