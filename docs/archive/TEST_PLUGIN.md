# 🧪 Testing Your Kawai Plugin

## ✅ Solana CLI Installed

Your Solana CLI is ready:
```
solana-test-validator 1.18.26
```

## 🚀 Test Your Plugin

### Step 1: Verify Plugin Library Exists

```bash
ls -lh target/release/libkawai.so
```

You should see the compiled plugin library.

### Step 2: Update Config File Path

Make sure `geyser-plugin-config.example.json` has the correct absolute path:

```json
{
  "libpath": "/mnt/c/Users/1/Documents/milla projects/kawai/target/release/libkawai.so",
  "account_filter": {
    "account": []
  },
  "transaction_filter": {}
}
```

**Note:** The path must be absolute and point to your `.so` file.

### Step 3: Start the Test Validator

```bash
solana-test-validator --geyser-plugin-config geyser-plugin-config.example.json
```

### Step 4: Watch for Plugin Messages

You should see:
- ✅ "Nyaa~ KawaiPlugin loading up! Ready to monitor Solana~ 💕"
- ✅ Account updates as they happen
- ✅ Transaction notifications
- ✅ Slot status updates
- ✅ "Yay~ All startup accounts loaded! Time for real action~ 🎉"

### Step 5: Generate Test Traffic (Optional)

In **another terminal**, generate some test transactions:

```bash
# Set cluster to localhost
solana config set --url localhost

# Create a test keypair
solana-keygen new --outfile test-keypair.json --no-bip39-passphrase

# Airdrop some SOL
solana airdrop 2 $(solana-keygen pubkey test-keypair.json)

# Create a transaction (this will trigger your plugin!)
solana transfer $(solana-keygen pubkey test-keypair.json) 0.5 $(solana-keygen pubkey) --allow-unfunded-recipient
```

## 🐛 Troubleshooting

### If Plugin Doesn't Load

1. **Check the library path:**
   ```bash
   # Verify the file exists
   ls -lh target/release/libkawai.so
   
   # Check if it's a valid shared library
   file target/release/libkawai.so
   ```

2. **Check library dependencies:**
   ```bash
   ldd target/release/libkawai.so
   ```

3. **Check validator logs:**
   - Look for plugin loading errors
   - Verify the config file path is correct

### If You See Errors

- **"library not found"**: Check the `libpath` in config is absolute and correct
- **"symbol not found"**: Rebuild the plugin: `cargo build --release`
- **"permission denied"**: Check file permissions: `chmod 755 target/release/libkawai.so`

## 🎉 Success Indicators

Your plugin is working if you see:
- ✅ Plugin loading message
- ✅ Account update messages with pubkeys and lamports
- ✅ Transaction notifications with signatures
- ✅ Slot status updates
- ✅ No error messages in the validator output

## 📝 Next Steps

Once your plugin is working:
1. **Add filtering** - Update config to monitor specific accounts
2. **Add persistence** - Save data to a database
3. **Add analytics** - Track statistics and metrics
4. **Deploy** - Use with a production validator

Enjoy your waifu-themed Solana geyser plugin! 🐾💕


