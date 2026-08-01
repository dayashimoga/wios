# WIOS Known Limitations

## Current Limitations (v0.1.0)

### Rust Build on Windows
- Requires MSVC Build Tools or MSYS2/MinGW-w64 installed
- `ring` crate requires a C compiler for assembly routines
- CI builds on Ubuntu/GitHub Actions work without this issue

### Mesh Networking
- libp2p mesh requires at least 2 devices to test real P2P functionality
- mDNS discovery only works on LAN (same network segment)
- NAT traversal requires relay nodes

### Wireless Sensing
- Motion/presence/fall detection requires physical accelerometer/gyroscope
- Indoor positioning requires multiple WiFi/BLE access points
- RF heatmaps require platform-specific WiFi scan APIs
- These features provide complete abstraction layers for plug-and-play when hardware is available

### AI Inference
- On-device LLM inference requires models downloaded separately (not bundled)
- GPU acceleration requires platform-specific setup (CUDA, Metal, Vulkan)
- Model quantization affects quality/performance tradeoff

### Device Sharing
- Camera/mic sharing requires user permission on all platforms
- Display sharing not available on iOS (platform restriction)
- USB peripherals (printer, etc.) require platform-specific drivers

### Web Platform
- WebRTC required for P2P (no raw TCP/UDP in browsers)
- No filesystem access (IndexedDB used instead)
- Limited Rust via WASM (some native features unavailable)
- No background processing in some browsers
