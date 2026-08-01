# WIOS Security

## Security Architecture

WIOS implements a **Zero Trust** security model where no connection is implicitly trusted.

### Cryptographic Primitives

| Algorithm | Purpose | Library |
|-----------|---------|---------|
| Ed25519 | Digital signatures, identity | ed25519-dalek |
| X25519 | Key exchange | x25519-dalek |
| AES-256-GCM | Symmetric encryption | aes-gcm |
| Argon2id | Password hashing | argon2 |
| Noise Protocol | Transport encryption | libp2p-noise |
| TLS 1.3 | API encryption | rustls |
| SHA-256 | Hashing | ring |

### Key Management
- Ed25519 keypairs for node identity
- X25519 for ephemeral key exchange
- AES-256 keys derived via HKDF
- Keys stored in encrypted keystore with expiry management
- Key rotation supported

### Authentication
- Certificate-based peer authentication
- Password authentication with Argon2id hashing
- MFA/Passkey abstraction layer (extensible)
- Session tokens with configurable timeout
- Account lockout after configurable failed attempts

### Authorization (RBAC)
- **Admin**: Full system access
- **User**: Standard operations (messaging, storage, AI, compute)
- **Guest**: Limited read-only access
- Custom roles with granular permissions
- Permission checking on all service operations

### Audit Logging
- All authentication events logged
- All data access events logged
- All administrative actions logged
- Configurable retention with FIFO eviction
- Structured log format for analysis

### Data Protection
- Encryption at rest (SQLite, RocksDB)
- Encryption in transit (Noise protocol, TLS)
- Secure key deletion
- No plaintext secrets in logs

## Threat Model

| Threat | Mitigation |
|--------|-----------|
| Man-in-the-Middle | E2EE with Noise protocol |
| Replay attacks | Message IDs + timestamps + TTL |
| Brute force | Argon2id + account lockout |
| Data exfiltration | Encryption at rest + RBAC |
| Unauthorized access | Zero Trust + MFA |
| Supply chain | SBOM + dependency scanning |
