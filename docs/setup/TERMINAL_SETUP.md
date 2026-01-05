# Terminal Setup for Braille ASCII Art

The Kawai Monitor uses Braille characters (Unicode) for the cute anime girl art. To make it display properly:

## Quick Fix

**In WSL/Linux terminal:**
```bash
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
```

Then run the monitor again.

## Permanent Fix

Add to your `~/.bashrc` or `~/.zshrc`:
```bash
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
```

Then reload:
```bash
source ~/.bashrc
```

## Check Current Encoding

```bash
echo $LANG
locale
```

Should show something like `en_US.UTF-8` or `C.UTF-8`.

## Font Requirements

Your terminal font needs to support Unicode Braille characters. Most modern terminals do, but if it still doesn't work:

1. **Windows Terminal (WSL)**: Should work by default
2. **VS Code Terminal**: Should work by default
3. **Other terminals**: May need a Unicode font like:
   - DejaVu Sans Mono
   - Noto Sans Mono
   - Fira Code
   - Cascadia Code

## Test Unicode Support

Run this to test:
```bash
echo "⠀⠁⠂⠃⠄⠅⠆⠇"
```

If you see dots/patterns (not question marks), Unicode Braille is supported!

## Alternative: Use a Different Terminal

If your current terminal doesn't support it:
- **Windows Terminal** (recommended for WSL)
- **Alacritty**
- **Kitty**
- **iTerm2** (Mac)

These all have excellent Unicode support.

