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

pub mod keystore;
pub mod encryption;
pub mod signing;
pub mod hashing;
pub mod kex;
pub mod rbac;
pub mod audit;
pub mod session;
pub mod identity;
pub mod mfa;
pub mod extended;

pub use keystore::KeyStore;
pub use encryption::EncryptionService;
pub use signing::SigningService;
pub use hashing::HashingService;
pub use kex::KeyExchange;
pub use rbac::{RbacEngine, Role, Permission};
pub use audit::AuditLogger;
pub use session::SessionManager;
pub use identity::CertificateManager;
pub use mfa::{Totp, RecoveryCodes};
pub use extended::{PasskeyManager, PairingManager, SecretsVault};
