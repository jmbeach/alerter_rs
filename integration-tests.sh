#!/usr/bin/env bash
set -euo pipefail

echo "Building showcase example..."
cargo build --example showcase
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

PASS=0
FAIL=0
PIDS=()
NAMES=()

# Launch notification types concurrently
for notification_type in banner sound badge list; do
    ./target/debug/examples/showcase "$notification_type" &
    PIDS+=($!)
    NAMES+=("$notification_type")
done

# Wait for all background processes and track results
for i in "${!PIDS[@]}"; do
    if wait "${PIDS[$i]}"; then
        echo "PASS: ${NAMES[$i]}"
        PASS=$((PASS + 1))
    else
        echo "FAIL: ${NAMES[$i]}"
        FAIL=$((FAIL + 1))
    fi
done

echo ""
echo "Summary: $PASS passed, $FAIL failed"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi

exit 0
