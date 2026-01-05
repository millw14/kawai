# Debugging Segmentation Fault

## Issue
Plugin loads but then segfaults. The shutdown message appears, suggesting the plugin was loaded and then unloaded, but then crashes.

## Possible Causes

1. **Entry point signature mismatch** - The validator expects a different signature
2. **Memory safety issue** - Problem with how the trait object is created/managed
3. **Version mismatch** - Interface version doesn't match validator expectations
4. **Missing trait bounds** - Plugin needs to be Send + Sync

## Changes Made

1. Added explicit `Send + Sync` implementations
2. Simplified entry point (removed `unsafe` from function signature, kept it only where needed)
3. Changed trait object creation

## Next Steps

1. **Rebuild:**
   ```bash
   cargo clean
   cargo build --release
   ```

2. **Test again:**
   ```bash
   solana-test-validator --geyser-plugin-config geyser-plugin-config.example.json
   ```

3. **If still crashing, try:**
   - Check validator logs: `cat test-ledger/validator.log`
   - Run with gdb for backtrace: `gdb --args solana-test-validator --geyser-plugin-config geyser-plugin-config.example.json`
   - Check if there's a version mismatch

4. **Alternative: Check example plugins**
   - Look at official Solana examples
   - Compare entry point signatures
   - Verify trait implementation

## Current Entry Point

```rust
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = KawaiPlugin::default();
    Box::into_raw(Box::new(plugin) as Box<dyn GeyserPlugin>)
}
```

If this still doesn't work, we may need to check the actual interface crate source to see the expected signature.


