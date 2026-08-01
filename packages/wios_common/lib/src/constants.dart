/// WIOS application constants.
class WiosConstants {
  WiosConstants._();

  static const String appName = 'WIOS';
  static const String appVersion = '0.1.0';
  static const String appDescription =
      'Wireless Intelligence Operating System';

  // Network defaults
  static const int defaultMeshPort = 9090;
  static const int defaultApiPort = 8080;
  static const int maxPeerConnections = 256;
  static const Duration peerTimeout = Duration(seconds: 30);
  static const Duration discoveryInterval = Duration(seconds: 10);

  // Storage defaults
  static const int defaultChunkSizeBytes = 256 * 1024; // 256 KB
  static const int maxStorageMb = 10 * 1024; // 10 GB
  static const double quotaWarningThreshold = 0.75;
  static const double quotaCriticalThreshold = 0.90;

  // Crypto defaults
  static const int sessionTimeoutSecs = 3600;
  static const int maxSessionsPerNode = 5;
  static const int certificateValidityDays = 365;

  // AI defaults
  static const int maxInferenceQueueSize = 100;
  static const int modelCacheMaxMb = 2048;
}
