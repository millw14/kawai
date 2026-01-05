#!/bin/bash
# Find the built library file

echo "=== Searching for built library ==="
echo ""

echo "1. Checking target/release directory:"
ls -lah target/release/ 2>/dev/null | head -20

echo ""
echo "2. Searching for .so files:"
find target/release -name "*.so" 2>/dev/null

echo ""
echo "3. Searching for files with 'kawai' in name:"
find target/release -name "*kawai*" 2>/dev/null

echo ""
echo "4. Checking deps directory:"
ls -lah target/release/deps/ 2>/dev/null | grep -E "\.so|kawai" | head -10

echo ""
echo "5. All files in release:"
find target/release -type f 2>/dev/null | head -20


