//! Integration tests — cross-crate interactions.

use wios_core::error::WiosResult;
use wios_core::types::{DeviceCapabilities, NodeId, NodeInfo, Platform};

/// Test crypto → storage: encrypt data, store it, retrieve and decrypt.
#[tokio::test]
async fn test_crypto_storage_integration() -> WiosResult<()> {
    // Generate key
    let key = wios_crypto::encryption::generate_key();
    let plaintext = b"sensitive mesh routing data";

    // Encrypt
    let ciphertext = wios_crypto::encryption::encrypt(&key, plaintext)?;

    // Store encrypted data
    let store = wios_storage::SqliteStore::open_in_memory()?;
    store.put("encrypted", "route-data", &ciphertext).await?;

    // Retrieve and decrypt
    let retrieved = store.get("encrypted", "route-data").await?.unwrap();
    let decrypted = wios_crypto::encryption::decrypt(&key, &retrieved)?;
    assert_eq!(decrypted, plaintext);
    Ok(())
}

/// Test storage → chunking: store chunked file, retrieve all chunks.
#[tokio::test]
async fn test_storage_chunking_roundtrip() -> WiosResult<()> {
    let data = vec![42u8; 10_000]; // 10KB test file
    let engine = wios_storage::ChunkEngine::new(4096);

    let chunks = engine.chunk(&data);
    assert!(chunks.len() >= 2);

    let reassembled = engine.reassemble(&chunks)?;
    assert_eq!(reassembled, data);
    Ok(())
}

/// Test compute → core: node capabilities matching.
#[test]
fn test_node_capabilities_matching() {
    let caps = DeviceCapabilities {
        cpu_cores: 4,
        ram_mb: 8192,
        storage_mb: 51200,
        has_gpu: false,
        has_bluetooth: true,
        has_wifi: true,
        has_camera: false,
        has_microphone: false,
        platform: Platform::current(),
    };

    assert!(caps.cpu_cores >= 2);
    assert!(caps.ram_mb >= 4096);
    assert!(caps.has_wifi);
}

/// Test crypto key exchange → session establishment.
#[tokio::test]
async fn test_kex_to_session() -> WiosResult<()> {
    let (pub_a, secret_a) = wios_crypto::kex::generate_keypair();
    let (pub_b, secret_b) = wios_crypto::kex::generate_keypair();

    let shared_a = wios_crypto::kex::compute_shared_secret(&secret_a, &pub_b);
    let shared_b = wios_crypto::kex::compute_shared_secret(&secret_b, &pub_a);
    assert_eq!(shared_a.as_bytes(), shared_b.as_bytes());

    // Derive AES key from shared secret
    let key_a = wios_crypto::kex::derive_aes_key(&shared_a, b"session-v1")?;
    let key_b = wios_crypto::kex::derive_aes_key(&shared_b, b"session-v1")?;
    assert_eq!(key_a, key_b);

    // Use derived key for encrypted communication
    let msg = b"authenticated session data";
    let encrypted = wios_crypto::encryption::encrypt(&key_a, msg)?;
    let decrypted = wios_crypto::encryption::decrypt(&key_b, &encrypted)?;
    assert_eq!(decrypted, msg);
    Ok(())
}

/// Test full node info + signing workflow.
#[test]
fn test_node_identity_signing() {
    let node = NodeInfo::new_local("integration-test-node".to_string());
    let (signing_key, verify_key) = wios_crypto::signing::generate_keypair();

    // Sign node ID
    let signature = wios_crypto::signing::sign(&signing_key, node.node_id.as_str().as_bytes());
    let valid = wios_crypto::signing::verify(&verify_key, node.node_id.as_str().as_bytes(), &signature);
    assert!(valid);
}

/// Test storage quota with chunked writes.
#[tokio::test]
async fn test_quota_with_storage() -> WiosResult<()> {
    let quota = wios_storage::QuotaManager::new(100_000); // 100KB quota

    quota.record_usage(50_000);
    assert!(!quota.is_exceeded());

    let status = quota.status();
    assert_eq!(status.used_bytes, 50_000);
    assert_eq!(status.total_bytes, 100_000);
    assert!(!status.is_warning);

    quota.record_usage(40_000); // 90% - warning
    assert!(quota.status().is_warning);
    Ok(())
}
