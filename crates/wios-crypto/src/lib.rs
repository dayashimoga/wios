//! # WIOS Crypto
//!
//! Cryptographic services for the Wireless Intelligence Operating System.
//!
//! Provides:
//! - Ed25519 key generation and digital signatures
//! - X25519 Diffie-Hellman key exchange
//! - AES-256-GCM symmetric encryption
//! - Argon2 password hashing
//! - Key management and keystore
//! - RBAC (Role-Based Access Control) engine
//! - Audit logging framework
//! - Session management
//! - Certificate/identity management
//! - Passkeys (WebAuthn abstraction)
//! - Secure device pairing
//! - Secrets vault

pub mod audit;
pub mod encryption;
pub mod extended;
pub mod hashing;
pub mod identity;
pub mod kex;
pub mod keystore;
pub mod mfa;
pub mod rbac;
pub mod session;
pub mod signing;

pub use audit::AuditLogger;
pub use encryption::EncryptionService;
pub use extended::{PairingManager, PasskeyManager, SecretsVault};
pub use hashing::HashingService;
pub use identity::CertificateManager;
pub use kex::KeyExchange;
pub use keystore::KeyStore;
pub use mfa::{RecoveryCodes, Totp};
pub use rbac::{Permission, RbacEngine, Role};
pub use session::SessionManager;
pub use signing::SigningService;
