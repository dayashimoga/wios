#!/usr/bin/env bash
# WIOS Build Script
set -euo pipefail

PLATFORM="${1:-all}"

echo "=== WIOS Build: $PLATFORM ==="

build_rust() {
    echo "Building Rust workspace..."
    cd crates
    cargo build --workspace --release
    cd ..
}

build_web() {
    echo "Building Flutter Web..."
    cd apps/wios_app
    flutter build web --release
    cd ../..
}

build_android() {
    echo "Building Flutter Android..."
    cd apps/wios_app
    flutter build apk --release
    cd ../..
}

build_windows() {
    echo "Building Flutter Windows..."
    cd apps/wios_app
    flutter build windows --release
    cd ../..
}

build_linux() {
    echo "Building Flutter Linux..."
    cd apps/wios_app
    flutter build linux --release
    cd ../..
}

case "$PLATFORM" in
    rust)     build_rust ;;
    web)      build_web ;;
    android)  build_android ;;
    windows)  build_windows ;;
    linux)    build_linux ;;
    all)
        build_rust
        build_web
        build_android
        build_windows
        build_linux
        ;;
    *)
        echo "Usage: $0 {rust|web|android|windows|linux|all}"
        exit 1
        ;;
esac

echo "=== Build Complete ==="
