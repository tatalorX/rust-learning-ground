#!/bin/bash
# Check all problems compile/run correctly

set -e

echo "🦀 Checking all Rust Learning Ground problems..."
echo ""

FAILED=0
PASSED=0

# Check exercises (1-150)
echo "Checking exercises (1-150)..."
for i in $(seq 1 150); do
    DIR=$(printf "problems/%03d_exercise" $i)
    if [ -f "$DIR/template.rs" ]; then
        if rustc "$DIR/template.rs" -o /tmp/check_bin 2>/dev/null; then
            PASSED=$((PASSED + 1))
            printf "✅ %03d\n" $i
        else
            FAILED=$((FAILED + 1))
            printf "❌ %03d (compile error)\n" $i
        fi
    else
        FAILED=$((FAILED + 1))
        printf "❌ %03d (missing)\n" $i
    fi
done

# Clean up
rm -f /tmp/check_bin

# Check projects (151-170)
echo ""
echo "Checking projects (151-170)..."
for i in $(seq 151 170); do
    DIR=$(printf "problems/%03d_project" $i)
    if [ -f "$DIR/Cargo.toml" ]; then
        if cargo test --quiet --manifest-path "$DIR/Cargo.toml" 2>/dev/null; then
            PASSED=$((PASSED + 1))
            printf "✅ %03d\n" $i
        else
            FAILED=$((FAILED + 1))
            printf "❌ %03d (test failed)\n" $i
        fi
    else
        FAILED=$((FAILED + 1))
        printf "❌ %03d (missing)\n" $i
    fi
done

echo ""
echo "=========================================="
echo "Results: $PASSED passed, $FAILED failed"
echo "=========================================="

if [ $FAILED -gt 0 ]; then
    exit 1
fi
