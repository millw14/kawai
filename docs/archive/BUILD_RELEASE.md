# Building in Release Mode

## Issue
You only have a `debug` build. For production use, you need a `release` build.

## Solution

Run the release build:

```bash
cargo build --release
```

This will:
- Create `target/release/` directory
- Build optimized code
- Generate `libkawai.so` in `target/release/`

## Verify

After building:

```bash
ls -lh target/release/libkawai.so
```

You should see the compiled library file.

## Why Release Mode?

- **Debug mode**: Faster compilation, includes debug symbols, not optimized
- **Release mode**: Optimized code, smaller binary, production-ready

For plugins, always use `--release` for production!


