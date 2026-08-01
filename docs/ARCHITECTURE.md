# WIOS Architecture

## System Overview

WIOS (Wireless Intelligence Operating System) is a multi-platform application built with a clean separation between frontend (Flutter/Dart) and backend (Rust), connected via flutter_rust_bridge.

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                    Flutter Frontend                          │
│  ┌──────────┐  ┌───────────┐  ┌────────────┐  ┌──────────┐ │
│  │ Material3 │  │   BLoC    │  │ Use Cases  │  │  Repos   │ │
│  │    UI     │──│   State   │──│  (Domain)  │──│(Abstracts)│ │
│  └──────────┘  └───────────┘  └────────────┘  └──────────┘ │
│                                                    │         │
│  ┌──────────────────────────────────────────────────┘        │
│  │  flutter_rust_bridge (FFI / Code Generation)              │
├──┼───────────────────────────────────────────────────────────┤
│  ▼           Rust Backend (Cargo Workspace)                  │
│  ┌──────────────────────────────────────────────────────────┐│
│  │ wios-bridge (API surface for Flutter)                    ││
│  └──────┬───────────────────────────────────────────────────┘│
│         │                                                    │
│  ┌──────▼──────┐  ┌─────────────┐  ┌──────────────────────┐ │
│  │ wios-core   │  │ wios-crypto │  │ wios-storage         │ │
│  │ Types,Config│  │ E2EE,RBAC   │  │ SQLite,RocksDB,Sync  │ │
│  │ Events,Errs │  │ Keys,Audit  │  │ CRDTs,Migrations     │ │
│  └─────────────┘  └─────────────┘  └──────────────────────┘ │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────────────┐ │
│  │ wios-network│  │ wios-ai     │  │ wios-compute         │ │
│  │ libp2p Mesh │  │ ONNX,TFLite │  │ Task Scheduler       │ │
│  │ Discovery   │  │ llama.cpp   │  │ Resource Manager     │ │
│  │ Routing     │  │ Pipeline    │  │ Device Sharing       │ │
│  └─────────────┘  └─────────────┘  └──────────────────────┘ │
│                                                              │
│  ┌──────────────────────────────────────────────────────────┐│
│  │ wios-api (REST via Axum / gRPC via Tonic)               ││
│  └──────────────────────────────────────────────────────────┘│
└──────────────────────────────────────────────────────────────┘
```

## Design Principles

1. **Clean Architecture** — Domain layer has zero framework dependencies
2. **SOLID** — Every service is behind an interface/trait
3. **DDD** — Domain-Driven Design with bounded contexts per crate
4. **Repository Pattern** — Data access abstracted behind interfaces
5. **Dependency Injection** — get_it (Dart) for runtime DI
6. **Event-Driven** — Tokio broadcast channels for decoupled communication
7. **Offline-First** — All data persisted locally, synced when connected
8. **Zero Trust** — No implicit trust, all connections authenticated

## Monorepo Structure

```
wios/
├── apps/wios_app/          # Flutter app (Android/iOS/Web/Windows/Linux/macOS)
├── packages/
│   ├── wios_ui/            # Material 3 design system
│   ├── wios_domain/        # Domain entities and use cases
│   ├── wios_data/          # Repository implementations
│   └── wios_common/        # Shared utilities
├── crates/
│   ├── wios-core/          # Rust core (types, config, events, errors)
│   ├── wios-crypto/        # Cryptography (E2EE, RBAC, audit)
│   ├── wios-storage/       # Storage (SQLite, RocksDB, sync)
│   ├── wios-network/       # Networking (libp2p mesh)
│   ├── wios-ai/            # AI inference (ONNX, TFLite, llama.cpp)
│   ├── wios-compute/       # Distributed compute
│   ├── wios-bridge/        # Flutter bridge API
│   └── wios-api/           # REST/gRPC server
├── docs/                   # Documentation
├── scripts/                # Build and setup scripts
└── .github/                # CI/CD workflows
```

## Technology Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Frontend | Flutter 3.x | Cross-platform, Material 3, strong typing |
| Backend | Rust | Memory safety, performance, no GC |
| Bridge | flutter_rust_bridge | Mature, typed, async |
| Networking | libp2p | P2P standard, modular, Rust-native |
| Storage | SQLite + RocksDB | Embedded, proven, fast |
| Crypto | ring + ed25519-dalek | Audited, fast, safe |
| AI | ONNX + TFLite + llama.cpp | Multi-engine, edge-optimized |
| API | Axum + Tonic | High perf, tower ecosystem |
| State Mgmt | BLoC | Predictable, testable, scalable |
| Monorepo | Melos | Flutter-standard, scriptable |
