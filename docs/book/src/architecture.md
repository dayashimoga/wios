# Architecture

## System Layers

```
┌─────────────────────────────────────────┐
│              Flutter UI                  │
│  Dashboard │ Mesh │ Messages │ AI │ ... │
├─────────────────────────────────────────┤
│         flutter_rust_bridge              │
├─────────────────────────────────────────┤
│              Rust Backend                │
│  wios-core    │ wios-crypto   │ wios-ai │
│  wios-storage │ wios-network  │ wios-api│
│  wios-compute │ wios-bridge              │
└─────────────────────────────────────────┘
```

## Crate Dependencies

| Crate | Depends On |
|-------|-----------|
| wios-core | (none) |
| wios-crypto | wios-core |
| wios-storage | wios-core, ring |
| wios-network | wios-core |
| wios-ai | wios-core |
| wios-compute | wios-core |
| wios-api | all crates |
| wios-bridge | all crates |

## Data Flow

1. **User action** in Flutter → BLoC event
2. BLoC calls **repository** → FFI bridge
3. Bridge invokes **Rust function**
4. Rust processes via **crate modules**
5. Result returns through bridge → BLoC state → UI
