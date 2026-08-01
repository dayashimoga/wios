# WIOS — Wireless Intelligence Operating System

WIOS is a production-ready, offline-first, AI-powered operating system for wireless mesh networks. It combines Flutter for cross-platform UI with a high-performance Rust backend.

## Key Features

- **Mesh Networking** — libp2p-based P2P mesh with mDNS, Kademlia, Gossipsub
- **Zero Trust Security** — E2EE, X25519 key exchange, RBAC, MFA/TOTP
- **Offline-First Storage** — SQLite + CRDT sync + content-addressed chunking
- **Edge AI** — ONNX Runtime, TFLite, llama.cpp inference
- **Distributed Compute** — Task scheduling across mesh nodes
- **Wireless Sensing** — RSSI positioning, RF heatmaps
- **Extensible** — Plugin system, REST + gRPC APIs, SDK

## Quick Start

```bash
# Clone and setup
git clone https://github.com/your-org/wios.git
cd wios
./scripts/setup.sh

# Run Rust tests
cd crates && cargo test --workspace

# Run Flutter app
cd apps/wios_app && flutter run
```
