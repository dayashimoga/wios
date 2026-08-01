# WIOS Testing Strategy

## Test Categories

### Unit Tests
- **Rust**: `cargo test --workspace` — Tests for all crate modules
- **Dart**: `flutter test` — Tests for domain logic, use cases

### Integration Tests
- Storage: SQLite + RocksDB operations
- Crypto: End-to-end encryption flows
- Network: Peer discovery + message exchange (simulated)

### Widget/UI Tests
- All Flutter screens
- Navigation flows
- Responsive layout verification

### E2E Tests
- Full application flows (start, discover, connect, message, sync)

### Performance Tests
- Message throughput benchmarks
- Storage read/write benchmarks
- AI inference latency benchmarks

## Coverage Target
- **>90% code coverage** for all crates and packages
- 100% passing tests at all times

## Running Tests

```bash
# Rust tests
cd crates && cargo test --workspace

# Flutter tests
melos run test

# All tests
./scripts/test.sh
```
