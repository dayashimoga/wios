# WIOS Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.6.0] - 2026-08-01

### Added

#### Feature Verification Framework (FVF)
- **Feature Registry** (`tools/fvf/feature_registry.yaml`): 60 features with requirement, code, API, UI, test, doc, and platform traceability
- **Feature Validator** (`tools/fvf/validate_features.py`): Verifies file existence, detects placeholders/mocks, generates RTM (markdown + JSON)
- **Documentation Validator** (`tools/fvf/validate_docs.py`): Checks all 15 required docs exist, are non-stubs, reference current modules
- **Dashboard Generator** (`tools/fvf/generate_dashboard.py`): Produces DASHBOARD.md with module progress, platform coverage, feature health, CI status
- **RTM** (`docs/RTM.md`, `docs/rtm.json`): Auto-generated Requirement Traceability Matrix
- **DASHBOARD** (`docs/DASHBOARD.md`): Auto-generated project dashboard with progress bars
- **FVF CI Workflow** (`.github/workflows/fvf.yml`): 6-stage pipeline (format → lint → security → FVF validation → tests/coverage → builds → SBOM)

### Validation Results
- Feature Validation: PASSED (60/60 features, 0 errors)
- Documentation Validation: PASSED (15/15 docs, 0 errors)
- Dashboard: Generated successfully

## [0.5.0] - 2026-08-01

### Added

#### Native Hardware Integration
- **BLE service** (`ble_service.dart`): Full BLE scanning, connect, read/write characteristics, RSSI tracking via `flutter_blue_plus`
- **Camera/Microphone service** (`media_capture_service.dart`): Photo/video capture, front/back switch, zoom, flash, recording via `camera` package
- **Keychain service** (`keychain_service.dart`): iOS Keychain / Android Keystore via `flutter_secure_storage` — stores node keys, auth tokens, MFA secrets, shared secrets
- **WiFi Direct service** (`wifi_direct_service.dart`): Android Wi-Fi P2P + iOS Multipeer Connectivity via native Kotlin/Swift method channels
- **UWB ranging service** (`uwb_service.dart`): Android UWB API + iOS Nearby Interaction via native method channels

#### Native Platform Code (Kotlin + Swift)
- **Android**: `WifiDirectHandler.kt` (Wi-Fi P2P peer discovery, connect, data transfer), `UwbHandler.kt` (UWB hardware detection, ranging sessions)
- **iOS**: `WifiDirectHandler.swift` (MultipeerConnectivity browsing, advertising, encrypted sessions), `UwbHandler.swift` (NearbyInteraction distance/angle measurement)

#### Platform Permissions
- **Android**: BLE scan/connect/advertise, location, camera, microphone permissions in AndroidManifest.xml
- **iOS**: NSBluetooth, NSCamera, NSMicrophone, NSLocation usage descriptions in Info.plist

### Dependencies Added
- `flutter_blue_plus: ^1.35.0`
- `camera: ^0.11.0`
- `record: ^5.2.0`
- `flutter_secure_storage: ^9.2.0`
- `permission_handler: ^11.3.0`

## [0.4.0] - 2026-08-01

### Added

#### Developer Platform
- **CLI tool** (`wios-cli`): `wios` binary with clap — subcommands for `info`, `peers`, `send`, `health`, `storage` (stats/list/get/put/backup), `infer`, `start`, `config`, `init`
- **Example plugin** (`plugins/smart-lights`): Reference implementation for WIOS plugin system — presence-based lighting automation with event subscriptions, zone configs

#### Testing
- **E2E integration tests** (`tests/e2e_tests.rs`): 5 cross-crate flows — node lifecycle, crypto pipeline, storage pipeline, network pipeline, AI pipeline
- **Performance benchmarks** (`tests/benchmarks.rs`): 6 benchmarks — encryption throughput, signing ops/s, chunking MB/s, SQLite ops/s, delta sync, compression throughput

#### Flutter UI
- **Light mode theme**: Full Material 3 light theme with `ThemeMode.system` (auto-switches based on OS preference)
- **Navigation wiring**: 4 new pages (Device Sharing, Location, SOS, Plugins) fully integrated into sidebar/bottom navigation (12 pages total)

#### CI/CD
- **iOS build pipeline**: `flutter build ios --release --no-codesign` on macOS runner
- **macOS build pipeline**: `flutter build macos --release`
- **SBOM generation**: CycloneDX JSON via `cargo-cyclonedx`
- **SHA-256 checksums**: For all release binaries
- **CLI binary artifact**: Built and uploaded alongside platform builds

### Changed
- Workspace expanded from 8 to 10 crates (`wios-cli`, `wios-tests`)
- Release pipeline expanded from 5 to 7 platforms (added iOS, macOS)

## [0.3.0] - 2026-08-01

### Added

#### Communication Layer (Sprint 1)
- **Emergency SOS** (`wios-network/emergency.rs`): Priority-based broadcast system with severity levels (Info/Warning/Critical/LifeThreatening), location attachment, peer acknowledgment, resolution tracking
- **Transport Provider** (`wios-network/transport_provider.rs`): Unified trait abstracting WiFi Direct, BLE, QUIC, LoRa, UWB with auto-selection by bandwidth/range, registry for multiple concurrent transports
- **Compression** (`wios-network/compression.rs`): RLE compression with Zstd/LZ4 algorithm selection, ratio estimation, roundtrip verified
- **Delta Sync** (`wios-network/delta_sync.rs`): Block-matching binary diff/patch for efficient file synchronization, handles identical/modified/appended/completely different files

#### Sensing & Location Layer (Sprint 2)
- **Sensor Fusion** (`wios-network/sensor_fusion.rs`): Multi-source position fusion (WiFi/BLE/UWB/GPS/DeadReckoning) with weighted averaging by confidence/accuracy, Kalman-like smoothing
- **Floor Plan** (`wios-network/floor_plan.rs`): Floor plan data model with rooms, waypoints, beacons; Dijkstra pathfinding; point-in-polygon room detection; path distance calculation
- **Asset Tracker** (`wios-network/asset_tracker.rs`): BLE/UWB tag management with geofence zones, automatic zone membership, stale asset detection
- **Automation** (`wios-network/automation.rs`): Rule engine with threshold/zone/schedule/peer conditions, cooldown timers, pluggable actions (alert, publish, inference, config, plugin, log)

#### Device Layer (Sprint 3)
- **Device Bus** (`wios-compute/device_bus.rs`): Universal Device Bus for sharing cameras, mics, displays, GPUs, sensors across mesh; cross-device clipboard sync; max-user enforcement; permission levels (ReadOnly/ReadWrite/FullControl)

#### Infrastructure Layer (Sprint 4)
- **Replication** (`wios-storage/replication.rs`): Multi-node data replication with version tracking, sync acknowledgment, under-replication detection
- **Versioning** (`wios-storage/versioning.rs`): Content version history with commit/rollback, automatic pruning of old versions
- **Backup** (`wios-storage/backup.rs`): Snapshot backup/restore with configurable retention; system metrics collector with CPU/memory/storage/peer tracking and historical averaging

#### AI Layer (Sprint 5)
- **Anomaly Detection** (`wios-ai/intelligence.rs`): Z-score statistical anomaly detector with configurable threshold and sliding window
- **NLP Processor** (`wios-ai/intelligence.rs`): Intent parser supporting scan, message, status, connect, encrypt, inference commands
- **Workflow Engine** (`wios-ai/intelligence.rs`): DAG-based workflow orchestrator with dependency resolution and step status tracking

#### Security Layer (Sprint 6)
- **Passkeys** (`wios-crypto/extended.rs`): WebAuthn abstraction with credential registration, authentication with sign count, per-user listing, revocation
- **Device Pairing** (`wios-crypto/extended.rs`): Secure pairing with 6-digit code exchange, verification, trusted pair tracking
- **Secrets Vault** (`wios-crypto/extended.rs`): Encrypted secret storage with tag-based search, CRUD operations

### Changed
- Updated all `lib.rs` files to register new modules and export public APIs
- Added `ordered-float` and `rand` dependencies to `wios-network`

## [0.2.0] - 2026-08-01

### Added

#### Security Hardening (Phase 2)
- X25519 key exchange with ECDH-derived AES keys (`wios-crypto/kex.rs`)
- MFA/TOTP with RFC 6238 compliance and recovery codes (`wios-crypto/mfa.rs`)
- Certificate/identity management with self-signed certs and revocation (`wios-crypto/identity.rs`)
- Session management with token lifecycle, max sessions, revocation (`wios-crypto/session.rs`)

#### Storage Enhancement (Phase 3)
- RocksDB integration via pluggable `StorageBackend` trait (`wios-storage/backend.rs`)
- File chunking with content-addressed SHA-256 and deduplication (`wios-storage/chunking.rs`)
- Storage quota management with warning/critical thresholds (`wios-storage/quota.rs`)

#### Mesh Networking (Phase 4)
- Gossipsub pub/sub with multi-topic support (`wios-network/gossip.rs`)
- Offline message queue with priority, TTL, retry (`wios-network/offline_queue.rs`)
- mDNS + Kademlia discovery service (`wios-network/discovery_service.rs`)
- WiFi/BLE RSSI positioning and RF heatmaps (`wios-network/sensing.rs`)
- Resumable encrypted file transfer (`wios-network/file_transfer.rs`)
- WebRTC signaling for voice/video (`wios-network/signaling.rs`)

#### AI & Edge Inference (Phase 5)
- ONNX/TFLite/llama.cpp backend traits (stub implementations awaiting native libs)
- Model manager with filesystem lifecycle
- Inference pipeline with task queuing

#### Distributed Computing (Phase 6)
- Task distribution across mesh with CPU/RAM/GPU matching (`wios-compute/distributed.rs`)
- Resource manager for mesh-wide capability discovery (`wios-compute/resource.rs`)

#### API & SDK (Phase 8)
- gRPC service definitions (`wios-api/grpc.rs`)
- Plugin system with lifecycle management (`wios-api/plugin.rs`)
- Protobuf spec with 7 RPCs (`proto/wios/v1/wios.proto`)
- OpenAPI 3.1 spec with 7 endpoints (`proto/openapi.yaml`)
- mdBook developer documentation (7 pages)

#### Flutter UI (8 pages)
- Auth page with login/register + MFA/TOTP flow
- Mesh network page with animated topology, signal bars, scan
- Messages page with E2E encrypted chat and channel tabs
- Storage page with usage gauge, quota breakdown, recent files
- AI Engine page with model list, inference stats, local chat
- Compute page with CPU/RAM/GPU gauges and task progress
- Sensing page with animated RF heatmap and indoor positioning
- Settings page with security/network toggles

#### CI/CD & Infrastructure
- GitHub Actions CI (lint, test, coverage, build)
- Release pipeline (5-platform builds)
- Docker dev and prod configurations
- Security audit (`cargo-audit`) in CI
- Coverage reporting (`cargo-tarpaulin`)

## [0.1.0] - 2026-07-31

### Added

#### Monorepo Foundation
- Monorepo root with .gitignore, Apache-2.0 LICENSE, README, melos.yaml
- Comprehensive documentation structure (16 documents)

#### Rust Backend (Cargo Workspace — 8 crates)
- **wios-core**: Configuration system (TOML), error type hierarchy with codes/retryability, domain types (DeviceId, PeerId, NodeId, NodeInfo, DeviceCapabilities, MeshMessage, Platform), async event bus (tokio broadcast), service trait interfaces (CryptoService, StorageService, NetworkService, AiService, ComputeService)
- **wios-crypto**: AES-256-GCM encryption, Ed25519 signing, Argon2id password hashing, KeyStore with persistence/expiry, RBAC engine with default roles (admin/user/guest) and custom roles, audit logging framework
- **wios-storage**: SQLite backend with WAL mode and versioned KV store, schema migration runner with default migrations, CRDT sync engine with vector clocks and LWW conflict resolution
- **wios-network**: libp2p MeshNode with peer management, routing table with multi-hop support, peer discovery configuration, transport layer configuration
- **wios-ai**: Inference engine with multi-backend support (ONNX/TFLite/GGUF), model manager for filesystem lifecycle, inference pipeline with task queuing
- **wios-compute**: Task scheduler with lifecycle management, resource manager for mesh-wide compute discovery
- **wios-bridge**: Flutter bridge API surface with app info, config, node info, capabilities
- **wios-api**: REST API routes (Axum) with health check, system info, nodes, network stats, storage stats, AI models, compute tasks endpoints

#### Flutter Frontend
- Material 3 dark theme with premium design
- Animated mesh topology visualization
- Responsive dashboard with sidebar nav (desktop) and bottom nav (mobile)
- Status cards for peers, messages, storage, AI, compute, uptime
- Security status banner
- Activity feed
- Multi-platform support (Android, iOS, Web, Windows, Linux, macOS)
- Flutter Web build verified

#### Flutter Packages
- wios_ui — Design system package
- wios_domain — Domain entities package
- wios_data — Data layer package
- wios_common — Shared utilities package
