#!/bin/bash
# Check validator logs for plugin errors

echo "=== Checking validator logs ==="
echo ""

if [ -f "test-ledger/validator.log" ]; then
    echo "Last 50 lines of validator.log:"
    tail -50 test-ledger/validator.log
else
    echo "Validator log not found at test-ledger/validator.log"
    echo "Looking for log files..."
    find . -name "*.log" -type f 2>/dev/null | head -5
fi

echo ""
echo "=== Checking for error patterns ==="
if [ -f "test-ledger/validator.log" ]; then
    echo "Errors found:"
    grep -i "error\|fail\|segfault\|panic\|plugin" test-ledger/validator.log | tail -20
fi

