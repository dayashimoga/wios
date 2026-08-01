/// Application info model.
class AppInfo {
  final String version;
  final String platform;
  final String rustVersion;
  final String buildTimestamp;

  const AppInfo({
    required this.version,
    required this.platform,
    required this.rustVersion,
    required this.buildTimestamp,
  });

  factory AppInfo.fromJson(Map<String, dynamic> json) => AppInfo(
        version: json['version'] as String,
        platform: json['platform'] as String,
        rustVersion: json['rust_version'] as String,
        buildTimestamp: json['build_timestamp'] as String,
      );

  Map<String, dynamic> toJson() => {
        'version': version,
        'platform': platform,
        'rust_version': rustVersion,
        'build_timestamp': buildTimestamp,
      };
}
