# WSL Setup Guide for Solana Geyser Plugin Development

## Quick Start

If you're on Windows and encountering the `std::os::unix::ffi::OsStringExt` error, follow these steps to set up WSL.

## Copy & Paste in WSL

**Quick Reference:**
- **Copy:** `Ctrl+Shift+C` (or select text, then `Ctrl+C` in some terminals)
- **Paste:** `Ctrl+Shift+V` (most common) or `Shift+Insert`
- **Right-click menu:** Right-click in terminal → Select "Copy" or "Paste"
- **Tip:** You can copy from Windows and paste into WSL (and vice versa) seamlessly!

## Step 1: Install WSL

1. Open **PowerShell as Administrator** (Right-click > Run as administrator)
2. Run this command (copy only the command, not the markdown syntax):
   ```powershell
   wsl --install
   ```
   **Note:** Copy only `wsl --install` - don't include the ```powershell part!
   
3. Wait for installation to complete (you'll see progress like "Installing: Virtual Machine Platform")
4. **Restart your PC** when prompted
4. After restart, search for "Ubuntu" in Start menu and open it
5. Create a username and password (remember these!)

## Step 2: Verify WSL Installation

**Note:** The `wsl` command is run from **Windows PowerShell/CMD**, not from inside WSL!

From Windows PowerShell (not inside WSL):
```powershell
wsl -l -v
```

If you're already inside the Ubuntu terminal (you see a prompt like `username@hostname:~$`), you're good to go! Skip to Step 3.

To verify you're in Linux/WSL from inside the terminal:
```bash
uname -a
```
You should see "Linux" in the output.

## Step 3: Install Rust in WSL

In the Ubuntu terminal:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Press Enter or type `1` for default installation
- Close and reopen the terminal
- Verify: `rustc --version` and `cargo --version`

## Step 4: Install Solana CLI in WSL

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

**If you get SSL errors:**

1. **Update CA certificates:**
   ```bash
   sudo apt-get update
   sudo apt-get install -y ca-certificates
   ```

2. **Try again:**
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   ```

3. **Alternative: Use apt repository (if curl still fails):**
   ```bash
   sudo sh -c 'echo "deb https://release.solana.com/stable/apt stable main" > /etc/apt/sources.list.d/solana.list'
   sudo apt update
   sudo apt install -y solana
   ```
   
   **If you still get TLS/SSL errors with apt:**
   
   a. **Update CA certificates first:**
      ```bash
      sudo apt-get update
      sudo apt-get install -y ca-certificates
      sudo update-ca-certificates
      ```
   
   b. **Try manual binary installation instead:**
      ```bash
      # Download latest release info (if you can access GitHub)
      # Or manually download from: https://github.com/solana-labs/solana/releases
      # Then extract and add to PATH
      ```
   
   c. **Check Windows firewall/antivirus** - they may be interfering with WSL network connections
   
   d. **Try from a different network** (e.g., mobile hotspot) to rule out network-level blocking

- Close/reopen terminal after installation
- Verify: `solana --version`

## Step 5: Install Build Tools (Required for Compiling Rust)

Before building Rust projects, you need C compiler and build tools:

```bash
sudo apt-get update
sudo apt-get install -y build-essential
```

This installs `gcc`, `make`, and other essential build tools.

## Step 6: Access Your Project in WSL

Your Windows files are accessible via `/mnt/c/`:

```bash
# Navigate to your project
cd /mnt/c/Users/YourName/Documents/milla\ projects/kawai

# Build the project
cargo build --release
```

## Step 7: VS Code Integration (Recommended)

1. Install the **"Remote - WSL"** extension in VS Code
2. In VS Code: `Ctrl+Shift+P` > "WSL: Connect to WSL"
3. Install the **rust-analyzer** extension in the WSL context
4. Open your project folder in VS Code

This gives you:
- ✅ Full Rust language support
- ✅ No false "unresolved import" warnings
- ✅ Integrated terminal in WSL
- ✅ Seamless development experience

## Building Your Plugin

Once in WSL:

```bash
# Navigate to project
cd /mnt/c/Users/YourName/Documents/milla\ projects/kawai

# Build
cargo build --release

# Run tests
cargo test

# The plugin will be in: target/release/libkawai.so
```

## Testing with Solana Test Validator

```bash
# Start test validator with your plugin
solana-test-validator --geyser-plugin-config config.json
```

## Troubleshooting

### "Command not found" after installing Rust/Solana
- Close and reopen the terminal
- Or run: `source ~/.bashrc` or `source ~/.profile`

### Permission denied errors
- Make sure you're in your home directory or have proper permissions
- Use `chmod` if needed for script files

### Build still fails
- Run `cargo clean` and rebuild
- Check Rust version: `rustc --version` (should be 1.75+)
- Verify Solana CLI: `solana --version`

## Alternative: Cross-Compile from Windows

If you prefer to stay in Windows PowerShell:

```powershell
# Install Linux target
rustup target add x86_64-unknown-linux-gnu

# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

However, WSL is recommended for a smoother development experience.

## Notes

- WSL 2 is required (not WSL 1)
- Your Windows files are accessible from WSL via `/mnt/c/`
- Changes made in WSL are immediately visible in Windows
- VS Code Remote-WSL provides the best development experience


  Downloaded libsecp256k1-core v0.2.2
  Downloaded pest_meta v2.8.4
  Downloaded pest_generator v2.8.4
  Downloaded pest_derive v2.8.4
  Downloaded parking_lot_core v0.9.12
  Downloaded iri-string v0.7.9
  Downloaded indicatif v0.17.11
  Downloaded indexmap v2.12.1
  Downloaded percent-encoding v1.0.1
  Downloaded encoding_rs v0.8.35
  Downloaded pkg-config v0.3.32
  Downloaded pin-utils v0.1.0
  Downloaded pin-project-internal v1.1.10
  Downloaded pin-project-lite v0.2.16
  Downloaded pbkdf2 v0.11.0
  Downloaded paste v1.0.15
  Downloaded parking_lot_core v0.8.6
  Downloaded parking v2.2.1
  Downloaded pairing v0.23.0
  Downloaded num-derive v0.4.2
  Downloaded futures-util v0.3.31
  Downloaded parking_lot v0.12.5
  Downloaded indicatif v0.18.3
  Downloaded pkcs8 v0.10.2
  Downloaded icu_normalizer_data v2.1.1
  Downloaded icu_normalizer v2.1.1
  Downloaded icu_locale_core v2.1.1
  Downloaded icu_collections v2.1.1
  Downloaded io-uring v0.7.11
  Downloaded memchr v2.7.6
  Downloaded itertools v0.10.5
  Downloaded idna v1.1.0
  Downloaded rtoolbox v0.0.3
  Downloaded num-bigint v0.2.6
  Downloaded scopeguard v1.2.0
  Downloaded rusticata-macros v4.1.0
  Downloaded same-file v1.0.6
  Downloaded rustc-hash v2.1.1
  Downloaded rfc6979 v0.4.0
  Downloaded rand_chacha v0.2.2
  Downloaded proc-macro-crate v3.4.0
  Downloaded rustc-hash v1.1.0
  Downloaded num-bigint v0.4.6
  Downloaded itertools v0.13.0
  Downloaded quote v1.0.42
  Downloaded rand_xorshift v0.3.0
  Downloaded radium v0.7.0
  Downloaded proc-macro-error-attr2 v2.0.0
  Downloaded rand_chacha v0.3.1
  Downloaded brotli v8.0.2
  Downloaded proc-macro-crate v0.1.5
  Downloaded predicates-tree v1.0.12
  Downloaded pretty-hex v0.3.0
  Downloaded mio v1.1.1
  Downloaded qualifier_attr v0.2.2
  Downloaded qstring v0.7.2
  Downloaded rustls-pemfile v1.0.4
  Downloaded rustls-native-certs v0.8.2
  Downloaded potential_utf v0.1.4
  Downloaded rustc-demangle v0.1.26
  Downloaded seqlock v0.2.0
  Downloaded prost-build v0.11.9
  Downloaded libm v0.2.15
  Downloaded reed-solomon-erasure v6.0.0
  Downloaded sec1 v0.7.3
  Downloaded sct v0.7.1
  Downloaded serde-big-array v0.5.1
  Downloaded itertools v0.12.1
  Downloaded powerfmt v0.2.0
  Downloaded prost-derive v0.11.9
  Downloaded rand_xoshiro v0.6.0
  Downloaded prost v0.11.9
  Downloaded rand_core v0.6.4
  Downloaded predicates v2.1.5
  Downloaded proc-macro-error2 v2.0.1
  Downloaded ppv-lite86 v0.2.21
  Downloaded rustc_version v0.4.1
  Downloaded serde_bytes v0.11.19
  Downloaded predicates-core v1.0.9
  Downloaded rand_chacha v0.9.0
  Downloaded hyper v0.14.32
  Downloaded rand_core v0.9.3
  Downloaded rand_core v0.5.1
  Downloaded rustversion v1.0.22
  Downloaded reqwest-middleware v0.4.2
  Downloaded quanta v0.12.6
  Downloaded icu_properties_data v2.1.2
  Downloaded hyper v1.8.1
  Downloaded im v15.1.0
  Downloaded http v0.2.12
  Downloaded hashbrown v0.16.1
  Downloaded hashbrown v0.15.5
  Downloaded hashbrown v0.14.5
  Downloaded sha-1 v0.9.8
  Downloaded hashbrown v0.13.2
  Downloaded hashbrown v0.12.3
  Downloaded quinn-udp v0.5.14
  Downloaded prost-types v0.11.9
  Downloaded h2 v0.3.27
  Downloaded bzip2-sys v0.1.13+1.0.8
  Downloaded parking_lot v0.11.2
  Downloaded serde_urlencoded v0.7.1
  Downloaded smpl_jwt v0.7.1
  Downloaded siphasher v1.0.1
  Downloaded slab v0.4.11
  Downloaded siphasher v0.3.11
  Downloaded simpl v0.1.0
  Downloaded simd-adler32 v0.3.8
  Downloaded signature v2.2.0
  Downloaded signal-hook-registry v1.4.7
  Downloaded solana-account v3.3.0
  Downloaded nom v7.1.3
  Downloaded smallvec v1.15.1
  Downloaded sized-chunks v0.6.5
  Downloaded miniz_oxide v0.8.9
  Downloaded minimal-lexical v0.2.1
  Downloaded solana-address v1.1.0
  Downloaded solana-account-info v3.1.0
  Downloaded solana-account-decoder-client-types v3.1.5
  Downloaded solana-account-decoder v3.1.5
  Downloaded k256 v0.13.4
  Downloaded semver v1.0.27
  Downloaded ryu v1.0.21
  Downloaded rustls-platform-verifier v0.6.2
  Downloaded solana-atomic-u64 v3.0.0
  Downloaded solana-address-lookup-table-interface v3.0.0
  Downloaded rustls-pki-types v1.13.2
  Downloaded native-tls v0.2.14
  Downloaded mockall_derive v0.11.4
  Downloaded matchit v0.7.3
  Downloaded lru v0.7.8
  Downloaded log v0.4.29
  Downloaded libsecp256k1 v0.6.0
  Downloaded jsonrpc-derive v18.0.0
  Downloaded socket2 v0.5.10
  Downloaded clap v2.34.0
  Downloaded solana-cluster-type v3.0.0
  Downloaded solana-borsh v3.0.0
  Downloaded solana-bloom v3.1.5
  Downloaded solana-blake3-hasher v3.1.0
  Downloaded solana-bincode v3.1.0
  Downloaded solana-big-mod-exp v3.0.0
  Downloaded socket2 v0.6.1
  Downloaded shlex v1.3.0
  Downloaded sha2 v0.10.9
  Downloaded sha2 v0.9.9
  Downloaded serde_derive v1.0.228
  Downloaded serde_core v1.0.228
  Downloaded prettyplease v0.1.25
  Downloaded pest v2.8.4
  Downloaded serde_yaml v0.9.34+deprecated
  Downloaded serde_with_macros v3.16.1
  Downloaded signal-hook v0.3.18
  Downloaded openssl-sys v0.9.111
  Downloaded solana-clock v3.0.0
  Downloaded solana-address v2.0.0
  Downloaded signature v1.6.4
  Downloaded shell-words v1.1.1
  Downloaded sha1 v0.10.6
  Downloaded proc-macro2 v1.0.103
  Downloaded polyval v0.6.2
  Downloaded soketto v0.7.1
  Downloaded hyper-rustls v0.24.2
  Downloaded solana-client-traits v3.0.0
  Downloaded hyper-proxy v0.9.1
  Downloaded http-body-util v0.1.3
  Downloaded hmac v0.12.1
  Downloaded futures v0.1.31
  Downloaded axum v0.6.20
  Downloaded aes v0.8.4
  Downloaded solana-bn254 v3.1.2
  Downloaded solana-builtins v3.1.5
  Downloaded solana-bls-signatures v1.0.0
  Downloaded solana-cli-config v3.1.5
  Downloaded modular-bitfield-impl v0.13.1
  Downloaded memmap2 v0.9.9
  Downloaded solana-commitment-config v3.1.0
  Downloaded solana-builtins-default-costs v3.1.5
  Downloaded httpdate v1.0.3
  Downloaded http-body v0.4.6
  Downloaded home v0.5.12
  Downloaded base64 v0.21.7
  Downloaded solana-client v3.1.5
  Downloaded solana-cli-output v3.1.5
  Downloaded solana-bpf-loader-program v3.1.5
  Downloaded solana-compute-budget v3.1.5
  Downloaded solana-clap-utils v3.1.5
  Downloaded solana-bucket-map v3.1.5
  Downloaded solana-config-interface v2.0.0
  Downloaded solana-compute-budget-program v3.1.5
  Downloaded solana-compute-budget-interface v3.0.0
  Downloaded ark-ff v0.4.2
  Downloaded libsecp256k1-gen-ecmult v0.2.1
  Downloaded libloading v0.7.4
  Downloaded lazy_static v1.5.0
  Downloaded itoa v1.0.16
  Downloaded ipnet v2.11.0
  Downloaded indexmap v1.9.3
  Downloaded hyper-rustls v0.27.7
  Downloaded httparse v1.10.1
  Downloaded flate2 v1.1.5
  Downloaded elliptic-curve v0.13.8
  Downloaded educe v0.6.0
  Downloaded ed25519-dalek v2.2.0
  Downloaded derive_more v0.99.20
  Downloaded der-parser v8.2.0
  Downloaded der v0.7.10
  Downloaded darling_core v0.21.3
  Downloaded solana-compute-budget-instruction v3.1.5
  Downloaded darling v0.21.3
  Downloaded crypto-bigint v0.5.5
  Downloaded openssl v0.10.75
  Downloaded crossbeam-channel v0.5.15
  Downloaded light-poseidon v0.4.0
  Downloaded light-poseidon v0.2.0
  Downloaded nix v0.30.1
  Downloaded solana-last-restart-slot v3.0.0
  Downloaded solana-hash v4.0.1
  Downloaded const-oid v0.9.6
  Downloaded combine v3.8.1
  Downloaded idna_adapter v1.2.1
  Downloaded ident_case v1.0.1
  Downloaded humantime v2.3.0
  Downloaded solana-merkle-tree v3.1.5
  Downloaded solana-epoch-rewards v3.0.0
  Downloaded solana-derivation-path v3.0.0
  Downloaded solana-epoch-schedule v3.0.0
  Downloaded solana-genesis-config v3.0.0
  Downloaded solana-epoch-rewards-hasher v3.1.0
  Downloaded solana-msg v3.0.0
  Downloaded solana-measure v3.1.5
  Downloaded solana-loader-v4-interface v3.1.0
  Downloaded solana-epoch-info v3.1.0
  Downloaded solana-feature-gate-interface v3.0.0
  Downloaded solana-fee-calculator v3.0.0
  Downloaded solana-file-download v3.1.0
  Downloaded solana-fee v3.1.5
  Downloaded solana-ed25519-program v3.0.0
  Downloaded solana-connection-cache v3.1.5
  Downloaded solana-curve25519 v3.1.5
  Downloaded solana-define-syscall v4.0.1
  Downloaded solana-cpi v3.1.0
  Downloaded group v0.13.0
  Downloaded solana-native-token v3.0.0
  Downloaded globset v0.4.18
  Downloaded glob v0.3.3
  Downloaded five8 v1.0.0
  Downloaded solana-hard-forks v3.0.0
  Downloaded solana-fee-structure v3.0.0
  Downloaded rand v0.8.5
  Downloaded quinn v0.11.9
  Downloaded event-listener v5.4.1
  Downloaded env_logger v0.11.8
  Downloaded digest v0.10.7
  Downloaded rayon-core v1.13.0
  Downloaded derivative v2.2.0
  Downloaded crc32fast v1.5.0
  Downloaded clang-sys v1.8.1
  Downloaded solana-nohash-hasher v0.2.1
  Downloaded console v0.15.11
  Downloaded solana-packet v3.0.0
  Downloaded solana-offchain-message v3.0.0
  Downloaded solana-instruction-error v2.1.0
  Downloaded solana-inflation v3.0.0
  Downloaded rand v0.9.2
  Downloaded solana-instruction v3.0.0
  Downloaded solana-hash v3.1.0
  Downloaded rustls-webpki v0.103.8
  Downloaded solana-nonce v3.0.0
  Downloaded asn1-rs-derive v0.4.0
  Downloaded solana-keccak-hasher v3.1.0
  Downloaded solana-instructions-sysvar v3.0.0
  Downloaded ascii v0.9.3
  Downloaded backoff v0.4.0
  Downloaded crypto-common v0.1.7
  Downloaded axum-core v0.3.4
  Downloaded solana-keypair v3.0.1
  Downloaded rand v0.7.3
  Downloaded base64ct v1.8.1
  Downloaded solana-pubkey v3.0.0
  Downloaded solana-program-entrypoint v3.1.1
  Downloaded solana-presigner v3.0.0
  Downloaded solana-precompile-error v3.0.0
  Downloaded solana-poseidon v3.1.5
  Downloaded solana-program-error v3.0.0
  Downloaded solana-poh-config v3.0.0
  Downloaded solana-net-utils v3.1.5
  Downloaded solana-faucet v3.1.5
  Downloaded raw-cpuid v11.6.0
  Downloaded solana-entry v3.1.5
  Downloaded solana-loader-v2-interface v3.0.0
  Downloaded solana-loader-v3-interface v6.1.0
  Downloaded solana-rent v3.1.0
  Downloaded solana-reward-info v3.0.0
  Downloaded solana-nonce-account v3.0.0
  Downloaded solana-genesis-utils v3.1.5
  Downloaded solana-program-memory v3.1.0
  Downloaded solana-lattice-hash v3.1.5
  Downloaded solana-loader-v4-program v3.1.5
  Downloaded serde v1.0.228
  Downloaded solana-pubkey v4.0.0
  Downloaded solana-program-pack v3.0.0
  Downloaded solana-program-option v3.0.0
  Downloaded solana-quic-definitions v3.0.0
  Downloaded lz4-sys v1.11.1+lz4-1.10.0
  Downloaded solana-remote-wallet v3.1.5
  Downloaded solana-metrics v3.1.5
  Downloaded solana-rpc-client-api v3.1.5
  Downloaded solana-sanitize v3.0.1
  Downloaded solana-sdk-macro v3.0.0
  Downloaded solana-sdk-ids v3.1.0
  Downloaded solana-message v3.0.1
  Downloaded solana-pubsub-client v3.1.5
  Downloaded solana-rayon-threadlimit v3.1.5
  Downloaded rpassword v7.4.0
  Downloaded portable-atomic v1.12.0
  Downloaded solana-rpc-client-types v3.1.5
  Downloaded solana-download-utils v3.1.5
  Downloaded solana-seed-derivable v3.0.0
  Downloaded solana-secp256r1-program v3.0.0
  Downloaded solana-rpc-client-nonce-utils v3.1.5
  Downloaded solana-secp256k1-program v3.0.0
  Downloaded solana-runtime-transaction v3.1.5
  Downloaded reqwest v0.11.27
  Downloaded reqwest v0.12.28
  Downloaded solana-secp256k1-recover v3.1.0
  Downloaded solana-quic-client v3.1.5
  Downloaded rayon v1.11.0
  Downloaded solana-perf v3.1.5
  Downloaded regex v1.12.2
  Downloaded rocksdb v0.23.0
  Downloaded serde_json v1.0.147
  Downloaded solana-slot-history v3.0.0
  Downloaded rustls-webpki v0.101.7
  Downloaded solana-serialize-utils v3.1.0
  Downloaded solana-stable-layout v3.0.0
  Downloaded solana-slot-hashes v3.0.0
  Downloaded solana-shred-version v3.0.0
  Downloaded quinn-proto v0.11.13
  Downloaded serde_with v3.16.1
  Downloaded solana-short-vec v3.0.0
  Downloaded solana-sha256-hasher v3.1.0
  Downloaded solana-serde-varint v3.0.0
  Downloaded solana-seed-phrase v3.0.0
  Downloaded solana-signature v3.1.0
  Downloaded solana-serde v3.0.0
  Downloaded solana-svm-measure v3.1.5
  Downloaded solana-svm-log-collector v3.1.5
  Downloaded solana-svm-feature-set v3.1.5
  Downloaded solana-svm-callback v3.1.5
  Downloaded solana-time-utils v3.0.0
  Downloaded solana-svm-type-overrides v3.1.5
  Downloaded solana-transaction-status-client-types v3.1.5
  Downloaded solana-svm-transaction v3.1.5
  Downloaded solana-svm-timings v3.1.5
  Downloaded solana-tls-utils v3.1.5
  Downloaded solana-storage-proto v3.1.5
  Downloaded solana-validator-exit v3.0.0
  Downloaded solana-cost-model v3.1.5
  Downloaded solana-signer v3.0.0
  Downloaded solana-version v3.1.5
  Downloaded solana-transaction-error v3.0.0
  Downloaded solana-system-interface v2.0.0
  Downloaded solana-system-transaction v3.0.0
  Downloaded solana-sysvar-id v3.1.0
  Downloaded solana-transaction v3.0.2
  Downloaded spl-discriminator-derive v0.2.0
  Downloaded spl-associated-token-account-interface v2.0.0
  Downloaded spl-discriminator v0.5.1
  Downloaded solana-transaction-metrics-tracker v3.1.5
  Downloaded solana-zk-elgamal-proof-program v3.1.5
  Downloaded strsim v0.11.1
  Downloaded spl-token-confidential-transfer-proof-extraction v0.5.1
  Downloaded spl-pod v0.7.1
  Downloaded spl-discriminator-syn v0.2.1
  Downloaded solana-vote-interface v4.0.4
  Downloaded solana-udp-client v3.1.5
  Downloaded solana-transaction-context v3.1.5
  Downloaded solana-tpu-client v3.1.5
  Downloaded jiff v0.2.17
  Downloaded spinning_top v0.3.0
  Downloaded stream-cancel v0.8.2
  Downloaded solana-send-transaction-service v3.1.5
  Downloaded stable_deref_trait v1.2.1
  Downloaded spl-token-group-interface v0.7.1
  Downloaded spl-memo-interface v2.0.0
  Downloaded spl-generic-token v2.0.1
  Downloaded spki v0.7.3
  Downloaded spin v0.9.8
  Downloaded solana-zk-token-proof-program v3.1.5
  Downloaded solana-vote v3.1.5
  Downloaded solana-unified-scheduler-logic v3.1.5
  Downloaded solana-sysvar v3.1.1
  Downloaded solana-system-program v3.1.5
  Downloaded solana-stake-interface v2.0.2
  Downloaded sync_wrapper v1.0.2
  Downloaded solana-transaction-status v3.1.5
  Downloaded subtle v2.6.1
  Downloaded strum v0.24.1
  Downloaded strsim v0.8.0
  Downloaded static_assertions v1.1.0
  Downloaded spl-type-length-value v0.9.0
  Downloaded spl-token-metadata-interface v0.8.0
  Downloaded spl-token-interface v2.0.0
  Downloaded spl-token-confidential-transfer-proof-generation v0.5.1
  Downloaded solana-tpu-client-next v3.1.5
  Downloaded strum_macros v0.24.3
  Downloaded sync_wrapper v0.1.2
  Downloaded symlink v0.1.0
  Downloaded tap v1.0.1
  Downloaded tinyvec_macros v0.1.1
  Downloaded textwrap v0.11.0
  Downloaded thiserror v2.0.17
  Downloaded thiserror v1.0.69
  Downloaded solana-program-runtime v3.1.5
  Downloaded time-macros v0.2.24
  Downloaded solana-poh v3.1.5
  Downloaded termtree v0.5.1
  Downloaded time-core v0.1.6
  Downloaded libc v0.2.178
  Downloaded tokio-io-timeout v1.2.1
  Downloaded synstructure v0.13.2
  Downloaded tokio-rustls v0.24.1
  Downloaded tokio-native-tls v0.3.1
  Downloaded tokio-macros v2.6.0
  Downloaded threadpool v1.8.1
  Downloaded solana-rpc-client v3.1.5
  Downloaded synstructure v0.12.6
  Downloaded rustls v0.21.12
  Downloaded tinyvec v1.10.0
  Downloaded thiserror-impl v1.0.69
  Downloaded tower-layer v0.3.3
  Downloaded thiserror-impl v2.0.17
  Downloaded tonic-build v0.9.2
  Downloaded tower-service v0.3.3
  Downloaded tempfile v3.24.0
  Downloaded tokio-tungstenite v0.28.0
  Downloaded try-lock v0.2.5
  Downloaded toml v0.5.11
  Downloaded tracing-attributes v0.1.31
  Downloaded toml_datetime v0.7.5+spec-1.1.0
  Downloaded tar v0.4.44
  Downloaded tokio-stream v0.1.17
  Downloaded utf-8 v0.7.6
  Downloaded unwrap_none v0.1.2
  Downloaded trees v0.4.2
  Downloaded unreachable v1.0.0
  Downloaded tokio-rustls v0.26.4
  Downloaded unicase v2.8.1
  Downloaded solana-geyser-plugin-manager v3.1.5
  Downloaded tinystr v0.8.2
  Downloaded utf8_iter v1.0.4
  Downloaded ucd-trie v0.1.7
  Downloaded void v1.0.2
  Downloaded unicode-xid v0.2.6
  Downloaded universal-hash v0.5.1
  Downloaded untrusted v0.9.0
  Downloaded unit-prefix v0.5.2
  Downloaded which v4.4.2
  Downloaded webpki-roots v0.26.11
  Downloaded toml_parser v1.0.6+spec-1.1.0
  Downloaded version_check v0.9.5
  Downloaded vec_map v0.8.2
  Downloaded wincode-derive v0.1.1
  Downloaded walkdir v2.5.0
  Downloaded xattr v1.6.1
  Downloaded petgraph v0.6.5
  Downloaded wyz v0.5.1
  Downloaded wincode v0.1.2
  Downloaded libz-sys v1.1.23
  Downloaded writeable v0.6.2
  Downloaded want v0.3.1
  Downloaded utf8parse v0.2.2
  Downloaded solana-sbpf v0.13.1
  Downloaded solana-rpc v3.1.5
  Downloaded rustls v0.23.35
  Downloaded solana-vote-program v3.1.5
  Downloaded regex-syntax v0.8.8
  Downloaded yoke-derive v0.8.1
  Downloaded tracing-core v0.1.36
  Downloaded unicode-ident v1.0.22
  Downloaded unsafe-libyaml v0.2.11
  Downloaded zerofrom v0.1.6
  Downloaded yoke v0.8.1
  Downloaded zerofrom-derive v0.1.6
  Downloaded zeroize_derive v1.4.2
  Downloaded uriparse v0.6.4
  Downloaded rustix v0.38.44
  Downloaded unicode-bidi v0.3.18
  Downloaded solana-streamer v3.1.5
  Downloaded zeroize v1.8.2
  Downloaded solana-svm v3.1.5
  Downloaded zerovec-derive v0.11.2
  Downloaded zstd v0.13.3
  Downloaded zstd-safe v7.2.4
  Downloaded rustix v1.1.3
  Downloaded zmij v0.1.8
  Downloaded solana-accounts-db v3.1.5
  Downloaded solana-storage-bigtable v3.1.5
  Downloaded tokio-util v0.6.10
  Downloaded tonic v0.9.2
  Downloaded spl-token-2022-interface v2.1.0
  Downloaded solana-ledger v3.1.5
  Downloaded tiny-bip39 v2.0.0
  Downloaded typenum v1.19.0
  Downloaded sha3 v0.10.8
  Downloaded tungstenite v0.28.0
  Downloaded tower v0.5.2
  Downloaded tower v0.4.13
  Downloaded toml_edit v0.23.10+spec-1.0.0
  Downloaded solana-gossip v3.1.5
  Downloaded time v0.3.44
  Downloaded regex-automata v0.4.13
  Downloaded url v1.7.2
  Downloaded tokio-util v0.7.17
  Downloaded url v2.5.7
  Downloaded tower-http v0.6.8
  Downloaded syn v1.0.109
  Downloaded unicode-normalization v0.1.25
  Downloaded syn v2.0.111
  Downloaded x509-parser v0.14.0
  Downloaded solana-runtime v3.1.5
  Downloaded zerotrie v0.2.3
  Downloaded winnow v0.7.14
  Downloaded webpki-roots v1.0.4
  Downloaded webpki-roots v0.25.4
  Downloaded vcpkg v0.2.15
  Downloaded unicode-width v0.2.2
  Downloaded unicode-width v0.1.14
  Downloaded zerovec v0.11.5
  Downloaded zerocopy v0.8.31
  Downloaded tracing v0.1.44
  Downloaded aquamarine v0.6.0
  Downloaded tokio v1.48.0
  Downloaded ring v0.17.14
  Downloaded zstd-sys v2.0.16+zstd.1.5.7
  Downloaded linux-raw-sys v0.4.15
  Downloaded linux-raw-sys v0.11.0
  Downloaded solana-zk-sdk v4.0.0
  Downloaded solana-zk-token-sdk v3.1.5
  Downloaded librocksdb-sys v0.17.3+10.4.2
  Downloaded protobuf-src v1.1.0+21.5
  Downloaded openssl-src v300.5.4+3.5.4
  Downloaded 794 crates (75.0MiB) in 18.87s (largest was `openssl-src` at 6.8MiB)
   Compiling proc-macro2 v1.0.103
   Compiling quote v1.0.42
   Compiling unicode-ident v1.0.22
   Compiling libc v0.2.178
   Compiling cfg-if v1.0.4
   Compiling serde_core v1.0.228
   Compiling version_check v0.9.5
   Compiling serde v1.0.228
   Compiling typenum v1.19.0
   Compiling once_cell v1.21.3
   Compiling subtle v2.6.1
   Compiling zerocopy v0.8.31
   Compiling const-oid v0.9.6
   Compiling autocfg v1.5.0
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.16.1
   Compiling cpufeatures v0.2.17
   Compiling cfg_aliases v0.2.1
   Compiling winnow v0.7.14
error: linker `cc` not found
  |
  = note: No such file or directory (os error 2)

error: could not compile `proc-macro2` (build script) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `serde` (build script) due to 1 previous error
error: could not compile `quote` (build script) due to 1 previous error
error: could not compile `serde_core` (build script) due to 1 previous error+
ious error
error: could not compile `libc` (build script) due to 1 previous error
error: could not compile `zerocopy` (build script) due to 1 previous error
error: could not compile `libc` (build script) due to 1 previous error