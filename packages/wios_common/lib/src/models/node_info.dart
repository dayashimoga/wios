/// Node information model.
class NodeInfo {
  final String nodeId;
  final String name;
  final String platform;
  final DateTime createdAt;
  final Map<String, dynamic>? metadata;

  const NodeInfo({
    required this.nodeId,
    required this.name,
    required this.platform,
    required this.createdAt,
    this.metadata,
  });

  factory NodeInfo.fromJson(Map<String, dynamic> json) => NodeInfo(
        nodeId: json['node_id'] as String,
        name: json['name'] as String,
        platform: json['platform'] as String,
        createdAt: DateTime.parse(json['created_at'] as String),
        metadata: json['metadata'] as Map<String, dynamic>?,
      );

  Map<String, dynamic> toJson() => {
        'node_id': nodeId,
        'name': name,
        'platform': platform,
        'created_at': createdAt.toIso8601String(),
        if (metadata != null) 'metadata': metadata,
      };
}
