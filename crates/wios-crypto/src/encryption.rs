//! AES-256-GCM encryption service.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use wios_core::error::{WiosError, WiosResult};

/// AES-256-GCM encryption and decryption.
pub struct EncryptionService;

impl EncryptionService {
    /// Encrypt plaintext with AES-256-GCM.
    ///
    /// Returns nonce (12 bytes) prepended to ciphertext.
    pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> WiosResult<Vec<u8>> {
        let cipher =
            Aes256Gcm::new_from_slice(key).map_err(|e| WiosError::Encryption(e.to_string()))?;

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| WiosError::Encryption(e.to_string()))?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt ciphertext with AES-256-GCM.
    ///
    /// Expects nonce (12 bytes) prepended to ciphertext.
    pub fn decrypt(key: &[u8; 32], data: &[u8]) -> WiosResult<Vec<u8>> {
        if data.len() < 12 {
            return Err(WiosError::Decryption("Data too short".into()));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let cipher =
            Aes256Gcm::new_from_slice(key).map_err(|e| WiosError::Decryption(e.to_string()))?;
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| WiosError::Decryption(e.to_string()))
    }

    /// Generate a random 256-bit key.
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = EncryptionService::generate_key();
        let plaintext = b"Hello, WIOS! This is a secret message.";

        let encrypted = EncryptionService::encrypt(&key, plaintext).unwrap();
        assert_ne!(&encrypted[12..], plaintext);

        let decrypted = EncryptionService::decrypt(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_key() {
        let key1 = EncryptionService::generate_key();
        let key2 = EncryptionService::generate_key();
        let plaintext = b"secret";

        let encrypted = EncryptionService::encrypt(&key1, plaintext).unwrap();
        assert!(EncryptionService::decrypt(&key2, &encrypted).is_err());
    }

    #[test]
    fn test_empty_plaintext() {
        let key = EncryptionService::generate_key();
        let encrypted = EncryptionService::encrypt(&key, b"").unwrap();
        let decrypted = EncryptionService::decrypt(&key, &encrypted).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn test_data_too_short() {
        let key = EncryptionService::generate_key();
        assert!(EncryptionService::decrypt(&key, &[0u8; 5]).is_err());
    }
}
