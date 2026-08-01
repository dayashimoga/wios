# WIOS Decisions

## ADR-001: Apache 2.0 License
**Decision:** Use Apache 2.0 license.
**Rationale:** Permissive, patent grant, industry standard for infrastructure software.

## ADR-002: Flutter + Rust Architecture
**Decision:** Flutter for frontend, Rust for backend, connected via flutter_rust_bridge.
**Rationale:** Flutter provides cross-platform UI with Material 3. Rust provides memory safety, performance, and no GC for networking/crypto.

## ADR-003: libp2p for Mesh Networking
**Decision:** Use libp2p (Rust) for P2P mesh networking.
**Rationale:** Production-proven (IPFS, Ethereum), modular, supports mDNS + Kademlia + Gossipsub + Noise, Rust-native.

## ADR-004: SQLite + RocksDB for Storage
**Decision:** SQLite for structured data, RocksDB for blob/KV storage.
**Rationale:** Both are embedded, battle-tested, and work offline. SQLite has excellent Rust support via rusqlite.

## ADR-005: BLoC for State Management
**Decision:** Use BLoC pattern for Flutter state management.
**Rationale:** Predictable state, testable, scales well, industry standard for complex Flutter apps.

## ADR-006: CRDT Sync with Vector Clocks
**Decision:** Use CRDTs with vector clocks for offline-first sync.
**Rationale:** Enables conflict-free merging without central coordination, essential for mesh/offline scenarios.

## ADR-007: Cargo Workspace Monorepo
**Decision:** Use Cargo workspace for Rust crates, Melos for Flutter packages.
**Rationale:** Shared dependencies, consistent versioning, single CI pipeline.
