import 'package:flutter_secure_storage/flutter_secure_storage.dart';

/// Secure storage service — wraps platform keychain/keystore.
/// iOS: Keychain Services, Android: EncryptedSharedPreferences / Keystore.
/// Wires into the Rust SecretsVault trait via flutter_rust_bridge.
class KeychainService {
  static const _storage = FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
    iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock_this_device),
  );

  /// Store a secret securely.
  Future<void> store(String key, String value) async {
    await _storage.write(key: key, value: value);
  }

  /// Retrieve a secret.
  Future<String?> get(String key) async {
    return await _storage.read(key: key);
  }

  /// Delete a secret.
  Future<void> delete(String key) async {
    await _storage.delete(key: key);
  }

  /// Check if a key exists.
  Future<bool> contains(String key) async {
    return await _storage.containsKey(key: key);
  }

  /// List all stored keys.
  Future<List<String>> listKeys() async {
    final all = await _storage.readAll();
    return all.keys.toList();
  }

  /// Delete all secrets.
  Future<void> deleteAll() async {
    await _storage.deleteAll();
  }

  // ── Convenience methods for WIOS-specific secrets ──

  /// Store the node's Ed25519 private key.
  Future<void> storeNodeKey(String privateKeyHex) async {
    await store('wios_node_private_key', privateKeyHex);
  }

  /// Retrieve the node's Ed25519 private key.
  Future<String?> getNodeKey() async {
    return await get('wios_node_private_key');
  }

  /// Store an API token for mesh authentication.
  Future<void> storeAuthToken(String token) async {
    await store('wios_auth_token', token);
  }

  /// Retrieve the auth token.
  Future<String?> getAuthToken() async {
    return await get('wios_auth_token');
  }

  /// Store the TOTP MFA secret.
  Future<void> storeMfaSecret(String secret) async {
    await store('wios_mfa_secret', secret);
  }

  /// Retrieve the MFA secret.
  Future<String?> getMfaSecret() async {
    return await get('wios_mfa_secret');
  }

  /// Store a shared secret derived from X25519 key exchange.
  Future<void> storeSharedSecret(String peerId, String secretHex) async {
    await store('wios_shared_$peerId', secretHex);
  }

  /// Retrieve a shared secret for a peer.
  Future<String?> getSharedSecret(String peerId) async {
    return await get('wios_shared_$peerId');
  }
}
