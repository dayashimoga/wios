import 'package:wios_common/wios_common.dart';
import 'package:wios_domain/wios_domain.dart';

/// Mock implementation of [NodeRepository] for development/testing
/// without the Rust FFI bridge.
class MockNodeRepository implements NodeRepository {
  @override
  Future<String> initialize() async => 'WIOS initialized (mock)';

  @override
  Future<AppInfo> getAppInfo() async => AppInfo(
        version: '0.1.0',
        platform: 'mock',
        rustVersion: '1.97.1',
        buildTimestamp: DateTime.now().toIso8601String(),
      );

  @override
  Future<NodeInfo> getLocalNodeInfo(String name) async => NodeInfo(
        nodeId: 'mock-${DateTime.now().millisecondsSinceEpoch}',
        name: name,
        platform: 'mock',
        createdAt: DateTime.now(),
      );

  @override
  Future<DeviceCapabilities> getDeviceCapabilities() async =>
      const DeviceCapabilities(
        cpuCores: 8,
        ramMb: 16384,
        storageMb: 512000,
        hasGpu: true,
        aiEngines: ['onnx', 'tflite', 'gguf'],
      );

  @override
  Future<String> getDefaultConfig() async => '''{
  "node": { "name": "wios-mock", "auto_start": true },
  "network": { "mesh_port": 9090 },
  "storage": { "max_storage_mb": 10240 }
}''';
}
