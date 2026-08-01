# WIOS Code Structure

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full architecture diagram.

## Key Files

### Rust Backend
| File | Purpose |
|------|---------|
| `crates/wios-core/src/config.rs` | Configuration system |
| `crates/wios-core/src/error.rs` | Error type hierarchy |
| `crates/wios-core/src/types.rs` | Core domain types |
| `crates/wios-core/src/event.rs` | Event bus |
| `crates/wios-core/src/traits.rs` | Service interfaces |
| `crates/wios-crypto/src/encryption.rs` | AES-256-GCM |
| `crates/wios-crypto/src/signing.rs` | Ed25519 signatures |
| `crates/wios-crypto/src/hashing.rs` | Argon2id hashing |
| `crates/wios-crypto/src/keystore.rs` | Key management |
| `crates/wios-crypto/src/rbac.rs` | RBAC engine |
| `crates/wios-crypto/src/audit.rs` | Audit logging |
| `crates/wios-storage/src/sqlite.rs` | SQLite storage |
| `crates/wios-storage/src/migration.rs` | Schema migrations |
| `crates/wios-storage/src/sync_engine.rs` | CRDT sync |
| `crates/wios-network/src/mesh.rs` | Mesh node (libp2p) |
| `crates/wios-network/src/routing.rs` | Routing table |
| `crates/wios-ai/src/engine.rs` | Inference engine |
| `crates/wios-ai/src/pipeline.rs` | Inference pipeline |
| `crates/wios-compute/src/scheduler.rs` | Task scheduler |
| `crates/wios-compute/src/resource.rs` | Resource manager |
| `crates/wios-bridge/src/api.rs` | Flutter bridge API |
| `crates/wios-api/src/rest.rs` | REST API routes |

### Flutter Frontend
| File | Purpose |
|------|---------|
| `apps/wios_app/lib/main.dart` | App entry + Dashboard UI |
