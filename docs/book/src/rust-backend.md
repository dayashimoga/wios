# Rust Backend

## Crates Overview

### wios-core (15 tests)
Core types, error hierarchy, configuration, event bus, trait definitions.

### wios-crypto (30 tests)
- AES-256-GCM encryption/decryption
- Ed25519 signing/verification
- X25519 key exchange (ECDH)
- Argon2 password hashing
- TOTP MFA + recovery codes
- RBAC with role/permission model
- Session management
- Certificate/identity management
- Audit logging

### wios-storage (23 tests)
- SQLite key-value store
- Schema migration system
- CRDT vector clock sync
- Content-addressed chunking + dedup
- Storage quota management
- Pluggable `StorageBackend` trait (SQLite, RocksDB stub)

### wios-network (19 tests)
- libp2p mesh node scaffold
- Gossipsub pub/sub broker
- Offline message queue (priority, TTL, retry)
- mDNS + Kademlia discovery service
- RSSI positioning + RF heatmap
- Resumable file transfer protocol
- WebRTC signaling abstraction

### wios-ai (8 tests)
- Inference engine + pipeline
- Model manager (load, cache, list)
- `AiBackend` trait (ONNX, TFLite, llama.cpp)

### wios-compute (6 tests)
- Task scheduler
- Resource manager
- Distributed task assignment (CPU/RAM/GPU matching)

### wios-api (5 tests)
- REST server (axum)
- gRPC service trait
- Plugin system with lifecycle management

### wios-bridge (9 tests)
- 14 flutter_rust_bridge API functions
- FFI surface for Flutter↔Rust communication
