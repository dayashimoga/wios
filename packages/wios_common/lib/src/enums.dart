/// WIOS shared enumerations.

/// Connection state of a peer.
enum ConnectionState {
  disconnected,
  connecting,
  connected,
  authenticated,
  error;

  bool get isOnline => this == connected || this == authenticated;
}

/// Supported platforms.
enum WiosPlatform {
  windows,
  macos,
  linux,
  android,
  ios,
  web,
  unknown;

  bool get isMobile => this == android || this == ios;
  bool get isDesktop => this == windows || this == macos || this == linux;
}

/// Storage quota alert level.
enum QuotaAlert {
  normal,
  warning,
  critical,
  exceeded;

  bool get needsAttention => this != normal;
}

/// Task execution status.
enum TaskStatus {
  pending,
  running,
  completed,
  failed,
  cancelled;

  bool get isTerminal =>
      this == completed || this == failed || this == cancelled;
}

/// RBAC permission level.
enum PermissionLevel {
  guest,
  user,
  admin,
  superAdmin;
}

/// Sync operation type.
enum SyncOperation {
  put,
  delete,
}

/// Merge conflict resolution result.
enum MergeResult {
  acceptRemote,
  keepLocal,
}
