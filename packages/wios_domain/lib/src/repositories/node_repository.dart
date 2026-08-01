import 'package:wios_common/wios_common.dart';

/// Repository interface for node operations.
abstract class NodeRepository {
  /// Initialize the WIOS backend.
  Future<String> initialize();

  /// Get application info from the Rust backend.
  Future<AppInfo> getAppInfo();

  /// Get local node information.
  Future<NodeInfo> getLocalNodeInfo(String name);

  /// Get device capabilities.
  Future<DeviceCapabilities> getDeviceCapabilities();

  /// Get the default configuration as JSON string.
  Future<String> getDefaultConfig();
}
