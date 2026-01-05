# Minimal Plugin Test

The segfault suggests there might be an issue with the plugin structure or entry point. Let's try a minimal version to isolate the problem.

## Current Issue

- Plugin loads (we see shutdown message)
- Then segfaults with "capacity overflow"
- Happens in `LoadedGeyserPlugin::new`

## Possible Solutions

1. **Check if entry point name is correct** - Maybe it's not `_create_plugin`
2. **Check if we need a different return type** - Maybe not `*mut dyn GeyserPlugin`
3. **Check if there's a macro to use** - Some crates provide macros for entry points
4. **Check version compatibility** - Validator 1.18.26 vs Interface 1.18.26 should match

## Next Steps

Run the find-entry-point script to see what the interface expects:

```bash
chmod +x find-entry-point.sh
./find-entry-point.sh
```

This will show us the actual expected signature from the interface crate source.

