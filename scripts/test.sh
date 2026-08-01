#!/usr/bin/env bash
# WIOS Test Runner
set -euo pipefail

echo "=== Running WIOS Tests ==="

# Rust tests
echo ""
echo "--- Rust Tests ---"
cd crates
cargo test --workspace --verbose
cd ..

# Flutter tests
echo ""
echo "--- Flutter Tests ---"
melos run test

echo ""
echo "=== All Tests Passed ==="
