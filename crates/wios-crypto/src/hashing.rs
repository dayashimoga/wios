//! Argon2 password hashing service.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use wios_core::error::{WiosError, WiosResult};

/// Argon2id password hashing and verification.
pub struct HashingService;

impl HashingService {
    /// Hash a password using Argon2id.
    pub fn hash_password(password: &str) -> WiosResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| WiosError::Crypto(format!("Password hashing failed: {}", e)))?;
        Ok(hash.to_string())
    }

    /// Verify a password against an Argon2id hash.
    pub fn verify_password(password: &str, hash: &str) -> WiosResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| WiosError::Crypto(format!("Invalid hash format: {}", e)))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "super_secret_p@ssw0rd!";
        let hash = HashingService::hash_password(password).unwrap();
        assert!(HashingService::verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_wrong_password() {
        let hash = HashingService::hash_password("correct").unwrap();
        assert!(!HashingService::verify_password("wrong", &hash).unwrap());
    }

    #[test]
    fn test_unique_hashes() {
        let h1 = HashingService::hash_password("same_password").unwrap();
        let h2 = HashingService::hash_password("same_password").unwrap();
        assert_ne!(h1, h2); // Different salts produce different hashes
    }
}
