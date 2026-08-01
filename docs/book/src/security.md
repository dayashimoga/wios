# Security

## Cryptographic Primitives

| Function | Algorithm | Crate |
|----------|-----------|-------|
| Symmetric Encryption | AES-256-GCM | `ring` |
| Key Exchange | X25519 (ECDH) | `x25519-dalek` |
| Signing | Ed25519 | `ring` |
| Password Hashing | Argon2id | `ring` (PBKDF2 fallback) |
| MFA | TOTP (RFC 6238) | `ring::hmac` |
| Key Derivation | HKDF-SHA256 | `ring::hkdf` |

## Security Architecture

- **Zero Trust**: All connections encrypted, all access authorized
- **E2EE**: End-to-end encryption for messages and file transfers
- **RBAC**: Role-based access control (Admin, Operator, Viewer, Guest)
- **Session Management**: Token-based, expiry, max per user, revocation
- **Audit Logging**: All security events logged with timestamps
- **Certificate Management**: Self-signed node certificates with revocation
