#!/usr/bin/env bash
# WIOS Setup Script
set -euo pipefail

echo "=== WIOS Development Environment Setup ==="

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi
echo "Rust: $(rustc --version)"

# Check Flutter
if ! command -v flutter &> /dev/null; then
    echo "ERROR: Flutter not found. Please install Flutter SDK."
    echo "Visit: https://docs.flutter.dev/get-started/install"
    exit 1
fi
echo "Flutter: $(flutter --version | head -1)"

# Check Melos
if ! command -v melos &> /dev/null; then
    echo "Installing Melos..."
    dart pub global activate melos
fi

# Bootstrap Flutter packages
echo "Bootstrapping Flutter packages..."
melos bootstrap

# Build Rust workspace
echo "Building Rust workspace..."
cd crates
cargo build
cd ..

echo ""
echo "=== Setup Complete ==="
echo "Run 'cd apps/wios_app && flutter run' to start the app."
