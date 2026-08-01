/// Repository interface for cryptographic operations.
abstract class CryptoRepository {
  /// Generate an Ed25519 signing keypair.
  Future<Map<String, String>> generateKeypair();

  /// Sign data with a private key.
  Future<String> sign(String privateKeyHex, List<int> data);

  /// Verify a signature.
  Future<bool> verify(String publicKeyHex, List<int> data, String signatureHex);

  /// Encrypt data with AES-256-GCM.
  Future<List<int>> encrypt(String keyHex, List<int> plaintext);

  /// Decrypt AES-256-GCM ciphertext.
  Future<List<int>> decrypt(String keyHex, List<int> ciphertext);

  /// Generate a random AES-256 key (hex string).
  Future<String> generateAesKey();

  /// Hash a password with Argon2id.
  Future<String> hashPassword(String password);

  /// Verify a password against an Argon2id hash.
  Future<bool> verifyPassword(String password, String hash);

  /// Generate X25519 keypair for key exchange.
  Future<Map<String, String>> generateKexKeypair();
}
