# WIOS Requirement Traceability Matrix (RTM)

> Auto-generated: 2026-08-01 18:25

| Req | Feature ID | Name | Code | API | UI | Tests | Docs | Platforms | Status |
|-----|-----------|------|------|-----|----|-------|------|-----------|--------|
| REQ-001 | FEAT-COMM-001 | Mesh Networking | `crates/wios-network/src/mesh.rs` | GET /api/v1/peers, POST /api/v1/peers/connect | pages/mesh_page.dart | mesh::tests | ARCHITECTURE.md, API.md | android, ios, windows... | ✅ |
| REQ-002 | FEAT-COMM-002 | Offline Messaging | `crates/wios-network/src/offline_queue.rs` | POST /api/v1/messages | pages/messages_page.dart | offline_queue::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-003 | FEAT-COMM-003 | Voice/Video Calling | `crates/wios-network/src/signaling.rs` | — | pages/mesh_page.dart | signaling::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-004 | FEAT-COMM-004 | Group Communication | `crates/wios-network/src/gossip.rs` | POST /api/v1/messages | pages/messages_page.dart | gossip::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-005 | FEAT-COMM-005 | Multi-hop Routing | `crates/wios-network/src/routing.rs` | — | — | routing::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-006 | FEAT-COMM-006 | Device Discovery | `crates/wios-network/src/discovery.rs, crates/wios-network/src/discovery_service.rs` | GET /api/v1/peers | pages/mesh_page.dart | discovery::tests, discovery_service::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-007 | FEAT-COMM-007 | Emergency SOS | `crates/wios-network/src/emergency.rs` | — | pages/sos_page.dart | emergency::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-008 | FEAT-COMM-008 | Transport Providers | `crates/wios-network/src/transport_provider.rs` | — | — | transport_provider::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-009 | FEAT-COMM-009 | Compression | `crates/wios-network/src/compression.rs` | — | — | compression::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-010 | FEAT-COMM-010 | Delta Sync | `crates/wios-network/src/delta_sync.rs` | — | — | delta_sync::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-011 | FEAT-COMM-011 | File Transfer | `crates/wios-network/src/file_transfer.rs` | — | pages/storage_page.dart | file_transfer::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-012 | FEAT-SENS-001 | RF Heatmap & RSSI Positioning | `crates/wios-network/src/sensing.rs` | — | pages/sensing_page.dart | sensing::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-013 | FEAT-SENS-002 | Sensor Fusion | `crates/wios-network/src/sensor_fusion.rs` | — | pages/sensing_page.dart | sensor_fusion::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-014 | FEAT-SENS-003 | Automation Rules | `crates/wios-network/src/automation.rs` | — | pages/settings_page.dart | automation::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-015 | FEAT-LOC-001 | Floor Plan Navigation | `crates/wios-network/src/floor_plan.rs` | — | pages/location_page.dart | floor_plan::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-016 | FEAT-LOC-002 | Asset Tracking | `crates/wios-network/src/asset_tracker.rs` | — | pages/location_page.dart | asset_tracker::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-017 | FEAT-DEV-001 | Universal Device Bus | `crates/wios-compute/src/device_bus.rs` | — | pages/device_sharing_page.dart | device_bus::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-018 | FEAT-STOR-001 | SQLite Backend | `crates/wios-storage/src/sqlite.rs` | GET /api/v1/storage/stats | pages/storage_page.dart | sqlite::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-019 | FEAT-STOR-002 | CRDT Sync Engine | `crates/wios-storage/src/sync_engine.rs` | — | — | sync_engine::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-020 | FEAT-STOR-003 | Chunking & Dedup | `crates/wios-storage/src/chunking.rs` | — | — | chunking::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-021 | FEAT-STOR-004 | Storage Quota | `crates/wios-storage/src/quota.rs` | — | pages/storage_page.dart | quota::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-022 | FEAT-STOR-005 | Replication | `crates/wios-storage/src/replication.rs` | — | — | replication::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-023 | FEAT-STOR-006 | Content Versioning | `crates/wios-storage/src/versioning.rs` | — | — | versioning::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-024 | FEAT-STOR-007 | Backup & Restore | `crates/wios-storage/src/backup.rs` | — | — | backup::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-025 | FEAT-AI-001 | Inference Engine | `crates/wios-ai/src/engine.rs` | POST /api/v1/ai/infer | pages/ai_page.dart | engine::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-026 | FEAT-AI-002 | Model Manager | `crates/wios-ai/src/model_manager.rs` | — | pages/ai_page.dart | model_manager::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-027 | FEAT-AI-003 | Anomaly Detection | `crates/wios-ai/src/intelligence.rs` | — | pages/ai_page.dart | intelligence::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-028 | FEAT-AI-004 | NLP Intent Parser | `crates/wios-ai/src/intelligence.rs` | — | — | intelligence::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-029 | FEAT-AI-005 | Workflow Orchestration | `crates/wios-ai/src/intelligence.rs` | — | — | intelligence::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-030 | FEAT-COMP-001 | Task Scheduler | `crates/wios-compute/src/scheduler.rs` | GET /api/v1/compute/tasks | pages/compute_page.dart | scheduler::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-031 | FEAT-COMP-002 | Resource Manager | `crates/wios-compute/src/resource.rs` | — | pages/compute_page.dart | resource::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-032 | FEAT-COMP-003 | Distributed Computing | `crates/wios-compute/src/distributed.rs` | — | pages/compute_page.dart | distributed::tests | ARCHITECTURE.md | android, ios, windows... | ✅ |
| REQ-033 | FEAT-SEC-001 | AES-256-GCM Encryption | `crates/wios-crypto/src/encryption.rs` | — | — | encryption::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-034 | FEAT-SEC-002 | Ed25519 Signing | `crates/wios-crypto/src/signing.rs` | — | — | signing::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-035 | FEAT-SEC-003 | Key Exchange (X25519) | `crates/wios-crypto/src/kex.rs` | — | — | kex::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-036 | FEAT-SEC-004 | Password Hashing (Argon2) | `crates/wios-crypto/src/hashing.rs` | — | pages/auth_page.dart | hashing::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-037 | FEAT-SEC-005 | MFA/TOTP | `crates/wios-crypto/src/mfa.rs` | — | pages/auth_page.dart | mfa::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-038 | FEAT-SEC-006 | RBAC | `crates/wios-crypto/src/rbac.rs` | — | pages/settings_page.dart | rbac::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-039 | FEAT-SEC-007 | Session Management | `crates/wios-crypto/src/session.rs` | — | — | session::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-040 | FEAT-SEC-008 | Identity & Certificates | `crates/wios-crypto/src/identity.rs` | — | — | identity::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-041 | FEAT-SEC-009 | Audit Logging | `crates/wios-crypto/src/audit.rs` | — | — | audit::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-042 | FEAT-SEC-010 | Key Store | `crates/wios-crypto/src/keystore.rs` | — | — | keystore::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-043 | FEAT-SEC-011 | Passkeys (WebAuthn) | `crates/wios-crypto/src/extended.rs` | — | pages/auth_page.dart | extended::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-044 | FEAT-SEC-012 | Device Pairing | `crates/wios-crypto/src/extended.rs` | — | pages/device_sharing_page.dart | extended::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-045 | FEAT-SEC-013 | Secrets Vault | `crates/wios-crypto/src/extended.rs` | — | — | extended::tests | SECURITY.md | android, ios, windows... | ✅ |
| REQ-046 | FEAT-DEVP-001 | REST API | `crates/wios-api/src/rest.rs` | GET /api/v1/health, GET /api/v1/system | — | rest::tests | API.md | android, ios, windows... | ✅ |
| REQ-047 | FEAT-DEVP-002 | gRPC Service | `crates/wios-api/src/grpc.rs` | — | — | grpc::tests | API.md | android, ios, windows... | ✅ |
| REQ-048 | FEAT-DEVP-003 | Plugin System | `crates/wios-api/src/plugin.rs` | — | pages/plugins_page.dart | plugin::tests | SDK.md | android, ios, windows... | ✅ |
| REQ-049 | FEAT-DEVP-004 | CLI Tool | `crates/wios-cli/src/main.rs` | — | — | — | SDK.md | windows, linux, macos | ✅ |
| REQ-050 | FEAT-DEVP-005 | Bridge (FFI) | `crates/wios-bridge/src/lib.rs` | — | — | bridge::tests | SDK.md | android, ios, windows... | ✅ |
| REQ-051 | FEAT-HW-001 | BLE Scanning | `apps/wios_app/lib/services/ble_service.dart` | — | pages/mesh_page.dart | — | SDK.md | android, ios | ✅ |
| REQ-052 | FEAT-HW-002 | Camera/Microphone | `apps/wios_app/lib/services/media_capture_service.dart` | — | pages/device_sharing_page.dart | — | SDK.md | android, ios | ✅ |
| REQ-053 | FEAT-HW-003 | Platform Keychain | `apps/wios_app/lib/services/keychain_service.dart` | — | — | — | SECURITY.md | android, ios | ✅ |
| REQ-054 | FEAT-HW-004 | WiFi Direct | `apps/wios_app/lib/services/wifi_direct_service.dart, apps/wios_app/android/app/src/main/kotlin/com/wios/wios_app/WifiDirectHandler.kt` | — | pages/mesh_page.dart | — | SDK.md | android, ios | ✅ |
| REQ-055 | FEAT-HW-005 | UWB Ranging | `apps/wios_app/lib/services/uwb_service.dart, apps/wios_app/android/app/src/main/kotlin/com/wios/wios_app/UwbHandler.kt` | — | pages/location_page.dart | — | SDK.md | android, ios | ✅ |
| REQ-056 | FEAT-UI-001 | Dashboard | `apps/wios_app/lib/main.dart` | — | Dashboard (_DashboardPage) | — | — | android, ios, windows... | ✅ |
| REQ-057 | FEAT-UI-002 | Light/Dark Theme | `apps/wios_app/lib/main.dart` | — | ThemeMode.system | — | — | android, ios, windows... | ✅ |
| REQ-058 | FEAT-UI-003 | Responsive Navigation | `apps/wios_app/lib/main.dart` | — | NavigationRail (desktop) | — | — | android, ios, windows... | ✅ |
| REQ-059 | FEAT-INFRA-001 | CI/CD Pipeline | `.github/workflows/ci.yml` | — | — | — | DEPLOYMENT.md | android, ios, windows... | ✅ |
| REQ-060 | FEAT-INFRA-002 | Docker | `Dockerfile.dev, Dockerfile.prod` | — | — | — | DEPLOYMENT.md | linux | ✅ |

**Total features: 60** | **Complete: 60** | **Pending: 0**
