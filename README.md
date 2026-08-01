# WIOS — Wireless Intelligence Operating System

[![CI](https://github.com/user/wios/actions/workflows/ci.yml/badge.svg)](https://github.com/user/wios/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Coverage](https://img.shields.io/badge/coverage-%3E90%25-brightgreen)]()

**WIOS** is a production-ready, open-source, offline-first, AI-powered operating system for wireless intelligence. It enables intelligent mesh networking, decentralized storage, distributed edge computing, device capability sharing, wireless sensing, indoor positioning, secure collaboration, and extensible plugin development — all without requiring internet connectivity.

## Features

- **AI-Managed Mesh Networking** — Intelligent peer-to-peer connectivity with multi-hop routing, automatic peer discovery, and AI-optimized path selection
- **Offline-First Communication** — Messaging, voice, and video that work without internet via device-to-device mesh
- **Distributed Encrypted Storage** — End-to-end encrypted file storage with CRDT-based conflict-free sync
- **Edge AI Inference** — Local ONNX, TFLite, and llama.cpp model execution for NLP, vision, and automation
- **Distributed Computing** — Share CPU, GPU, RAM, and storage across the mesh for distributed task execution
- **Device Capability Sharing** — Share cameras, microphones, displays, keyboards, mice, printers, and sensors
- **Indoor Positioning** — WiFi/BLE RSSI-based positioning with fingerprinting and trilateration
- **Wireless Sensing** — Motion detection, presence sensing, occupancy monitoring, RF heatmaps, and signal analytics
- **Zero Trust Security** — E2EE, MFA, Passkeys, RBAC, audit logging, and certificate-based identity
- **Plugin SDK** — Extensible architecture with sandboxed plugin execution and public APIs

## Architecture

```
Flutter 3.x (Material 3)  ←→  flutter_rust_bridge  ←→  Rust Backend
     ├── BLoC State                                      ├── libp2p Mesh
     ├── Clean Architecture                              ├── SQLite/RocksDB
     └── Material Design 3                               ├── ONNX/TFLite/llama.cpp
                                                         ├── E2EE/Zero Trust
                                                         └── gRPC/REST APIs
```

## Repository Structure

```
wios/
├── apps/
│   └── wios_app/           # Main Flutter application
├── packages/
│   ├── wios_ui/            # Design system & Material 3 components
│   ├── wios_domain/        # Domain entities, use cases, interfaces
│   ├── wios_data/          # Repository implementations, data sources
│   └── wios_common/        # Shared utilities, constants, extensions
├── crates/
│   ├── wios-core/          # Core types, errors, config, traits
│   ├── wios-crypto/        # E2EE, key management, signatures
│   ├── wios-storage/       # SQLite/RocksDB abstraction
│   ├── wios-network/       # libp2p mesh networking engine
│   ├── wios-ai/            # ONNX/TFLite/llama.cpp inference
│   ├── wios-compute/       # Distributed compute manager
│   ├── wios-bridge/        # flutter_rust_bridge API surface
│   └── wios-api/           # gRPC/REST server
├── sdk/                    # SDK packages for third-party developers
├── docs/                   # Documentation
├── docker/                 # Docker configurations
├── scripts/                # Build, test, and setup scripts
├── configs/                # Configuration templates
└── .github/                # CI/CD workflows
```

## Quick Start (Docker-First)

### Prerequisites

- **Docker** & **Docker Compose** — that's it. No local Rust, Flutter, or other tooling needed.

### Setup

```bash
# Clone the repository
git clone https://github.com/user/wios.git
cd wios

# Build the development Docker image (one-time)
make build

# Run all tests
make test

# Build everything (Rust + Flutter Web)
make all

# Serve the web app on http://localhost:8082
make flutter-serve

# Open a shell inside the dev container
make shell
```

### Common Commands

```bash
make rust-check      # Check Rust compilation
make rust-test       # Run Rust tests
make rust-clippy     # Lint Rust code
make flutter-web     # Build Flutter web release
make flutter-test    # Run Flutter tests
make ci              # Full CI check (lint + test + build)
make help            # Show all available commands
```

## Platforms

| Platform | Status |
|----------|--------|
| Android  | ✅ Supported |
| iOS      | ✅ Supported |
| Windows  | ✅ Supported |
| Linux    | ✅ Supported |
| macOS    | ✅ Supported |
| Web      | ✅ Supported (graceful degradation) |

## Documentation

- [Requirements](docs/REQUIREMENTS.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Implementation](docs/IMPLEMENTATION.md)
- [API Reference](docs/API.md)
- [SDK Guide](docs/SDK.md)
- [Security](docs/SECURITY.md)
- [Testing](docs/TESTING.md)
- [Deployment](docs/DEPLOYMENT.md)
- [Contributing](docs/CONTRIBUTING.md)
- [Roadmap](docs/ROADMAP.md)
- [Changelog](docs/CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
