/// Hex encoding/decoding utilities.
class HexUtils {
  HexUtils._();

  /// Encode bytes to hex string.
  static String encode(List<int> bytes) =>
      bytes.map((b) => b.toRadixString(16).padLeft(2, '0')).join();

  /// Decode hex string to bytes.
  static List<int> decode(String hex) {
    if (hex.length % 2 != 0) {
      throw FormatException('Hex string must have even length', hex);
    }
    return List.generate(
      hex.length ~/ 2,
      (i) => int.parse(hex.substring(i * 2, i * 2 + 2), radix: 16),
    );
  }
}
