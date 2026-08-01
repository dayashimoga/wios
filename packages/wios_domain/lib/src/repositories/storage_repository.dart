/// Repository interface for storage operations.
abstract class StorageRepository {
  /// Store a key-value pair.
  Future<void> put(String namespace, String key, List<int> value);

  /// Retrieve a value by key.
  Future<List<int>?> get(String namespace, String key);

  /// Delete a key.
  Future<void> delete(String namespace, String key);

  /// List all keys in a namespace.
  Future<List<String>> listKeys(String namespace);

  /// Get storage statistics.
  Future<Map<String, dynamic>> getStats();
}
