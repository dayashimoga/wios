# WIOS SDK Guide

## Overview
The WIOS SDK enables third-party developers to build plugins and integrate with the WIOS platform.

## Supported Languages
- **Dart/Flutter** — Native SDK package
- **Rust** — Crate dependency
- **REST API** — Language-agnostic HTTP/JSON
- **gRPC** — High-performance typed API

## Quick Start

### Dart SDK
```dart
import 'package:wios_sdk/wios_sdk.dart';

final wios = WiosClient();
await wios.connect();
final peers = await wios.listPeers();
await wios.sendMessage(peerId, 'Hello!');
```

### Rust SDK
```rust
use wios_core::prelude::*;

let config = WiosConfig::default();
let node = WiosNode::new(config)?;
node.start().await?;
```

### REST API
```bash
curl http://localhost:8080/api/v1/health
```

## Plugin Development
See [Plugin SDK documentation](docs/SDK.md#plugins) for creating custom plugins.
