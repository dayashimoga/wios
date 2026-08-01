//! Ed25519 digital signature service.

use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use wios_core::error::{WiosError, WiosResult};

/// Ed25519 signing and verification.
pub struct SigningService;

impl SigningService {
    /// Generate a new Ed25519 keypair.
    ///
    /// Returns (private_key_bytes, public_key_bytes).
    pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        (signing_key.to_bytes().to_vec(), verifying_key.to_bytes().to_vec())
    }

    /// Sign data with an Ed25519 private key.
    pub fn sign(private_key: &[u8], data: &[u8]) -> WiosResult<Vec<u8>> {
        let key_bytes: [u8; 32] = private_key
            .try_into()
            .map_err(|_| WiosError::Crypto("Invalid private key length".into()))?;
        let signing_key = SigningKey::from_bytes(&key_bytes);
        let signature = signing_key.sign(data);
        Ok(signature.to_bytes().to_vec())
    }

    /// Verify an Ed25519 signature.
    pub fn verify(public_key: &[u8], data: &[u8], signature: &[u8]) -> WiosResult<bool> {
        let key_bytes: [u8; 32] = public_key
            .try_into()
            .map_err(|_| WiosError::Crypto("Invalid public key length".into()))?;
        let verifying_key = VerifyingKey::from_bytes(&key_bytes)
            .map_err(|e| WiosError::Crypto(e.to_string()))?;

        let sig_bytes: [u8; 64] = signature
            .try_into()
            .map_err(|_| WiosError::Crypto("Invalid signature length".into()))?;
        let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes);

        Ok(verifying_key.verify(data, &sig).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let (private_key, public_key) = SigningService::generate_keypair();
        let data = b"Hello, WIOS!";

        let signature = SigningService::sign(&private_key, data).unwrap();
        assert!(SigningService::verify(&public_key, data, &signature).unwrap());
    }

    #[test]
    fn test_wrong_key_verification() {
        let (private_key, _) = SigningService::generate_keypair();
        let (_, wrong_public_key) = SigningService::generate_keypair();
        let data = b"test data";

        let signature = SigningService::sign(&private_key, data).unwrap();
        assert!(!SigningService::verify(&wrong_public_key, data, &signature).unwrap());
    }

    #[test]
    fn test_tampered_data() {
        let (private_key, public_key) = SigningService::generate_keypair();
        let data = b"original data";

        let signature = SigningService::sign(&private_key, data).unwrap();
        assert!(!SigningService::verify(&public_key, b"tampered data", &signature).unwrap());
    }
}
