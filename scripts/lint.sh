#!/usr/bin/env bash
# WIOS Lint & Format Script
set -euo pipefail

echo "=== WIOS Lint & Format ==="

# Rust
echo ""
echo "--- Rust Format ---"
cd crates
cargo fmt --all --check
echo ""
echo "--- Rust Clippy ---"
cargo clippy --workspace --all-targets -- -D warnings
cd ..

# Flutter/Dart
echo ""
echo "--- Dart Format ---"
melos run format
echo ""
echo "--- Dart Analyze ---"
melos run analyze

echo ""
echo "=== Lint & Format Complete ==="
