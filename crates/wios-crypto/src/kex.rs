//! X25519 Diffie-Hellman key exchange.

use rand::rngs::OsRng;
use wios_core::error::{WiosError, WiosResult};
use x25519_dalek::{PublicKey, SharedSecret};

/// X25519 key exchange service.
pub struct KeyExchange;

/// A key exchange keypair (static secret stored as bytes for persistence).
#[derive(Clone)]
pub struct KexKeypair {
    /// 32-byte secret key.
    pub secret: [u8; 32],
    /// 32-byte public key.
    pub public: [u8; 32],
}

impl KeyExchange {
    /// Generate a new X25519 keypair.
    pub fn generate_keypair() -> KexKeypair {
        let secret = x25519_dalek::StaticSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);
        KexKeypair {
            secret: secret.to_bytes(),
            public: public.to_bytes(),
        }
    }

    /// Perform key exchange: compute shared secret from our secret + their public.
    pub fn derive_shared_secret(
        our_secret: &[u8; 32],
        their_public: &[u8; 32],
    ) -> WiosResult<[u8; 32]> {
        let secret = x25519_dalek::StaticSecret::from(*our_secret);
        let public = PublicKey::from(*their_public);
        let shared: SharedSecret = secret.diffie_hellman(&public);
        let bytes = shared.to_bytes();
        // Reject low-order points (all zeros = identity element)
        if bytes.iter().all(|&b| b == 0) {
            return Err(WiosError::Crypto(
                "Key exchange produced zero shared secret (low-order point)".into(),
            ));
        }
        Ok(bytes)
    }

    /// Derive an AES-256 encryption key from a shared secret using HKDF-SHA256.
    pub fn derive_aes_key(shared_secret: &[u8; 32], info: &[u8]) -> WiosResult<[u8; 32]> {
        use ring::hkdf;
        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"wios-kex-v1");
        let prk = salt.extract(shared_secret);
        let info_refs = [info];
        let okm = prk
            .expand(&info_refs, &ring::aead::AES_256_GCM)
            .map_err(|_| WiosError::Crypto("HKDF expansion failed".into()))?;
        let mut key = [0u8; 32];
        okm.fill(&mut key)
            .map_err(|_| WiosError::Crypto("HKDF fill failed".into()))?;
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_exchange() {
        let alice = KeyExchange::generate_keypair();
        let bob = KeyExchange::generate_keypair();

        let shared_a = KeyExchange::derive_shared_secret(&alice.secret, &bob.public).unwrap();
        let shared_b = KeyExchange::derive_shared_secret(&bob.secret, &alice.public).unwrap();
        assert_eq!(shared_a, shared_b);
    }

    #[test]
    fn test_derive_aes_key() {
        let alice = KeyExchange::generate_keypair();
        let bob = KeyExchange::generate_keypair();

        let shared = KeyExchange::derive_shared_secret(&alice.secret, &bob.public).unwrap();
        let key = KeyExchange::derive_aes_key(&shared, b"test-context").unwrap();
        assert_ne!(key, [0u8; 32]);
        assert_eq!(key.len(), 32);
    }
}
