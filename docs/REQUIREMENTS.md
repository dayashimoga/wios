# WIOS Requirements

## Functional Requirements

### FR-1: Mesh Networking
- FR-1.1: P2P device discovery via mDNS (LAN) and Kademlia DHT
- FR-1.2: Multi-hop message routing with AI-optimized path selection
- FR-1.3: Offline message queuing with store-and-forward delivery
- FR-1.4: Voice/video signaling via WebRTC abstraction
- FR-1.5: Encrypted file transfer (chunked, resumable)

### FR-2: Communication
- FR-2.1: Real-time text messaging (1:1 and group)
- FR-2.2: Voice communication over mesh
- FR-2.3: Video communication over mesh
- FR-2.4: Broadcast messaging to all mesh peers

### FR-3: Storage
- FR-3.1: Encrypted local storage (SQLite + RocksDB)
- FR-3.2: CRDT-based conflict-free sync across mesh
- FR-3.3: File chunking and deduplication
- FR-3.4: Storage quota management

### FR-4: Security
- FR-4.1: End-to-end encryption (E2EE) for all communications
- FR-4.2: Zero Trust architecture — all connections authenticated
- FR-4.3: Role-Based Access Control (RBAC)
- FR-4.4: MFA/Passkey support
- FR-4.5: Audit logging for all security events
- FR-4.6: Certificate-based identity management

### FR-5: AI & Inference
- FR-5.1: Local ONNX model inference
- FR-5.2: TFLite model inference
- FR-5.3: llama.cpp LLM text generation
- FR-5.4: AI-powered network optimization
- FR-5.5: Automation rule engine

### FR-6: Distributed Computing
- FR-6.1: Task scheduling and distribution across mesh
- FR-6.2: Resource discovery (CPU/GPU/RAM/storage)
- FR-6.3: Device capability sharing (camera, mic, display, etc.)
- FR-6.4: Result aggregation

### FR-7: Wireless Sensing
- FR-7.1: WiFi/BLE RSSI-based indoor positioning
- FR-7.2: RF heatmap generation
- FR-7.3: Motion/presence/occupancy detection abstractions
- FR-7.4: Signal analytics

### FR-8: Platform Support
- FR-8.1: Android (APK/AAB)
- FR-8.2: iOS (IPA)
- FR-8.3: Windows (EXE/MSI)
- FR-8.4: Linux (DEB/RPM/AppImage)
- FR-8.5: macOS (DMG)
- FR-8.6: Web (with graceful degradation)

## Non-Functional Requirements

### NFR-1: Performance
- Message delivery latency < 100ms on local mesh
- AI inference latency < 500ms for small models
- App startup time < 3 seconds

### NFR-2: Security
- All data encrypted at rest and in transit
- No plaintext secrets in memory longer than necessary
- Regular key rotation support

### NFR-3: Reliability
- >99.9% message delivery within TTL
- Automatic reconnection on connection loss
- Graceful degradation when peers disconnect

### NFR-4: Scalability
- Support 100+ concurrent mesh peers
- Handle 10,000+ messages/second
- Storage up to platform limits

### NFR-5: Testing
- >90% code coverage
- 100% test pass rate
- Unit, integration, widget, E2E tests
