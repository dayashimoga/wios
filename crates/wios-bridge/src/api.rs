//! Bridge API functions exposed to Flutter.
//!
//! These functions serve as the interface between Flutter and the Rust backend.
//! flutter_rust_bridge generates Dart bindings from these function signatures.

use serde::{Deserialize, Serialize};
use wios_core::config::WiosConfig;
use wios_core::types::{DeviceCapabilities, NodeInfo, Platform};

// ── App Info ──────────────────────────────────────────────────────

/// Application version information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub version: String,
    pub platform: String,
    pub rust_version: String,
    pub build_timestamp: String,
}

/// Get application info.
pub fn get_app_info() -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: Platform::current().to_string(),
        rust_version: "1.97.1".to_string(),
        build_timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

/// Initialize the WIOS backend with default configuration.
pub fn initialize() -> Result<String, String> {
    let config = WiosConfig::default();
    match config.validate() {
        Ok(()) => Ok("WIOS initialized successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// Get the default configuration as JSON.
pub fn get_default_config() -> String {
    let config = WiosConfig::default();
    serde_json::to_string_pretty(&config).unwrap_or_default()
}

/// Get local node information.
pub fn get_local_node_info(name: String) -> String {
    let info = NodeInfo::new_local(name);
    serde_json::to_string(&info).unwrap_or_default()
}

/// Get device capabilities.
pub fn get_device_capabilities() -> String {
    let caps = DeviceCapabilities {
        cpu_cores: num_cpus(),
        ram_mb: 0,
        storage_mb: 0,
        has_gpu: false,
        has_camera: false,
        has_microphone: false,
        has_display: true,
        has_keyboard: true,
        has_pointer: true,
        has_printer: false,
        has_accelerometer: false,
        has_gyroscope: false,
        has_gps: false,
        has_bluetooth: false,
        has_wifi_direct: false,
        has_nfc: false,
        battery_level: None,
        ai_engines: vec!["onnx".into(), "tflite".into(), "gguf".into()],
    };
    serde_json::to_string(&caps).unwrap_or_default()
}

// ── Crypto Operations ─────────────────────────────────────────────

/// Generate an Ed25519 keypair. Returns JSON with `private_key` and `public_key` (hex).
pub fn crypto_generate_keypair() -> String {
    let (sk, pk) = wios_crypto::SigningService::generate_keypair();
    serde_json::to_string(&serde_json::json!({
        "private_key": hex_encode(&sk),
        "public_key": hex_encode(&pk),
    }))
    .unwrap_or_default()
}

/// Sign data with an Ed25519 private key (hex). Returns hex signature.
pub fn crypto_sign(private_key_hex: String, data: Vec<u8>) -> Result<String, String> {
    let key = hex_decode(&private_key_hex).map_err(|e| e.to_string())?;
    let sig = wios_crypto::SigningService::sign(&key, &data).map_err(|e| e.to_string())?;
    Ok(hex_encode(&sig))
}

/// Verify an Ed25519 signature. All inputs hex-encoded.
pub fn crypto_verify(public_key_hex: String, data: Vec<u8>, signature_hex: String) -> Result<bool, String> {
    let key = hex_decode(&public_key_hex).map_err(|e| e.to_string())?;
    let sig = hex_decode(&signature_hex).map_err(|e| e.to_string())?;
    wios_crypto::SigningService::verify(&key, &data, &sig).map_err(|e| e.to_string())
}

/// Encrypt data with AES-256-GCM. Key is hex-encoded (64 hex chars = 32 bytes).
pub fn crypto_encrypt(key_hex: String, plaintext: Vec<u8>) -> Result<Vec<u8>, String> {
    let key_bytes = hex_decode(&key_hex).map_err(|e| e.to_string())?;
    let key: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| "Key must be 32 bytes (64 hex chars)".to_string())?;
    wios_crypto::EncryptionService::encrypt(&key, &plaintext).map_err(|e| e.to_string())
}

/// Decrypt AES-256-GCM ciphertext. Key is hex-encoded.
pub fn crypto_decrypt(key_hex: String, ciphertext: Vec<u8>) -> Result<Vec<u8>, String> {
    let key_bytes = hex_decode(&key_hex).map_err(|e| e.to_string())?;
    let key: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| "Key must be 32 bytes (64 hex chars)".to_string())?;
    wios_crypto::EncryptionService::decrypt(&key, &ciphertext).map_err(|e| e.to_string())
}

/// Generate a random AES-256 key. Returns hex string.
pub fn crypto_generate_aes_key() -> String {
    hex_encode(&wios_crypto::EncryptionService::generate_key())
}

/// Hash a password with Argon2id.
pub fn crypto_hash_password(password: String) -> Result<String, String> {
    wios_crypto::HashingService::hash_password(&password).map_err(|e| e.to_string())
}

/// Verify a password against an Argon2id hash.
pub fn crypto_verify_password(password: String, hash: String) -> Result<bool, String> {
    wios_crypto::HashingService::verify_password(&password, &hash).map_err(|e| e.to_string())
}

/// Generate X25519 keypair for key exchange. Returns JSON with secret/public (hex).
pub fn crypto_generate_kex_keypair() -> String {
    let kp = wios_crypto::KeyExchange::generate_keypair();
    serde_json::to_string(&serde_json::json!({
        "secret": hex_encode(&kp.secret),
        "public": hex_encode(&kp.public),
    }))
    .unwrap_or_default()
}

// ── AI Operations ─────────────────────────────────────────────────

/// List available AI models. Returns JSON array.
pub fn ai_list_models(models_dir: String) -> String {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let engine = wios_ai::InferenceEngine::new(models_dir, 2048);
    let models = rt.block_on(engine.list_available_models()).unwrap_or_default();
    serde_json::to_string(&models).unwrap_or_default()
}

// ── Helpers ───────────────────────────────────────────────────────

fn num_cpus() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".into());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_info() {
        let info = get_app_info();
        assert!(!info.version.is_empty());
        assert!(!info.platform.is_empty());
    }

    #[test]
    fn test_initialize() {
        assert!(initialize().is_ok());
    }

    #[test]
    fn test_default_config() {
        let config = get_default_config();
        assert!(config.contains("node"));
        assert!(config.contains("network"));
    }

    #[test]
    fn test_local_node_info() {
        let info = get_local_node_info("test-node".into());
        assert!(info.contains("test-node"));
    }

    #[test]
    fn test_device_capabilities() {
        let caps = get_device_capabilities();
        assert!(caps.contains("cpu_cores"));
    }

    #[test]
    fn test_crypto_sign_verify() {
        let kp_json = crypto_generate_keypair();
        let kp: serde_json::Value = serde_json::from_str(&kp_json).unwrap();
        let sk = kp["private_key"].as_str().unwrap().to_string();
        let pk = kp["public_key"].as_str().unwrap().to_string();

        let sig = crypto_sign(sk, b"hello".to_vec()).unwrap();
        assert!(crypto_verify(pk, b"hello".to_vec(), sig).unwrap());
    }

    #[test]
    fn test_crypto_encrypt_decrypt() {
        let key = crypto_generate_aes_key();
        let encrypted = crypto_encrypt(key.clone(), b"secret".to_vec()).unwrap();
        let decrypted = crypto_decrypt(key, encrypted).unwrap();
        assert_eq!(decrypted, b"secret");
    }

    #[test]
    fn test_crypto_password() {
        let hash = crypto_hash_password("pass123".into()).unwrap();
        assert!(crypto_verify_password("pass123".into(), hash.clone()).unwrap());
        assert!(!crypto_verify_password("wrong".into(), hash).unwrap());
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = vec![0u8, 1, 255, 128, 64];
        let hex = hex_encode(&original);
        let decoded = hex_decode(&hex).unwrap();
        assert_eq!(original, decoded);
    }
}
