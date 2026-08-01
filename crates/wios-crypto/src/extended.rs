//! Extended security — passkeys, device pairing, secrets vault.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

// ─── Passkeys (WebAuthn abstraction) ───

/// A passkey credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passkey {
    pub credential_id: String,
    pub user_id: String,
    pub public_key: Vec<u8>,
    pub sign_count: u32,
    pub created_at: u64,
    pub last_used: u64,
    pub device_name: String,
}

/// Passkey manager.
pub struct PasskeyManager {
    passkeys: HashMap<String, Passkey>,
}

impl PasskeyManager {
    pub fn new() -> Self { Self { passkeys: HashMap::new() } }

    pub fn register(&mut self, passkey: Passkey) -> String {
        let id = passkey.credential_id.clone();
        self.passkeys.insert(id.clone(), passkey);
        id
    }

    pub fn authenticate(&mut self, credential_id: &str) -> WiosResult<&Passkey> {
        let pk = self.passkeys.get_mut(credential_id)
            .ok_or(WiosError::NotFound { entity: "passkey".into(), id: credential_id.into() })?;
        pk.sign_count += 1;
        pk.last_used = now();
        Ok(pk)
    }

    pub fn list_for_user(&self, user_id: &str) -> Vec<&Passkey> {
        self.passkeys.values().filter(|p| p.user_id == user_id).collect()
    }

    pub fn revoke(&mut self, credential_id: &str) -> WiosResult<()> {
        self.passkeys.remove(credential_id)
            .map(|_| ())
            .ok_or(WiosError::NotFound { entity: "passkey".into(), id: credential_id.into() })
    }
}

impl Default for PasskeyManager {
    fn default() -> Self { Self::new() }
}

// ─── Secure Device Pairing ───

/// Pairing state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PairingState { Initiated, CodeExchanged, Verified, Paired, Rejected }

/// A device pairing session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingSession {
    pub id: String,
    pub initiator: NodeId,
    pub target: NodeId,
    pub code: String,
    pub state: PairingState,
    pub created_at: u64,
}

/// Pairing manager.
pub struct PairingManager {
    sessions: HashMap<String, PairingSession>,
    trusted_pairs: Vec<(NodeId, NodeId)>,
}

impl PairingManager {
    pub fn new() -> Self { Self { sessions: HashMap::new(), trusted_pairs: Vec::new() } }

    /// Start a pairing session. Returns session ID and pairing code.
    pub fn initiate(&mut self, initiator: NodeId, target: NodeId) -> (String, String) {
        let id = uuid::Uuid::new_v4().to_string();
        let code = format!("{:06}", rand::random::<u32>() % 1_000_000);
        self.sessions.insert(id.clone(), PairingSession {
            id: id.clone(), initiator, target, code: code.clone(),
            state: PairingState::Initiated, created_at: now(),
        });
        (id, code)
    }

    /// Verify pairing code.
    pub fn verify(&mut self, session_id: &str, code: &str) -> WiosResult<bool> {
        let session = self.sessions.get_mut(session_id)
            .ok_or(WiosError::NotFound { entity: "pairing".into(), id: session_id.into() })?;
        if session.code == code {
            session.state = PairingState::Verified;
            Ok(true)
        } else {
            session.state = PairingState::Rejected;
            Ok(false)
        }
    }

    /// Complete pairing (after verification).
    pub fn complete(&mut self, session_id: &str) -> WiosResult<()> {
        let session = self.sessions.get_mut(session_id)
            .ok_or(WiosError::NotFound { entity: "pairing".into(), id: session_id.into() })?;
        if session.state != PairingState::Verified {
            return Err(WiosError::Crypto("Pairing not verified".into()));
        }
        session.state = PairingState::Paired;
        self.trusted_pairs.push((session.initiator.clone(), session.target.clone()));
        Ok(())
    }

    pub fn is_paired(&self, a: &NodeId, b: &NodeId) -> bool {
        self.trusted_pairs.iter().any(|(x, y)| (x == a && y == b) || (x == b && y == a))
    }
}

impl Default for PairingManager {
    fn default() -> Self { Self::new() }
}

// ─── Secrets Vault ───

/// An encrypted secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub key: String,
    pub encrypted_value: Vec<u8>,
    pub created_at: u64,
    pub updated_at: u64,
    pub tags: Vec<String>,
}

/// Secrets vault.
pub struct SecretsVault {
    secrets: HashMap<String, Secret>,
}

impl SecretsVault {
    pub fn new() -> Self { Self { secrets: HashMap::new() } }

    pub fn store(&mut self, key: String, encrypted_value: Vec<u8>, tags: Vec<String>) {
        let ts = now();
        self.secrets.insert(key.clone(), Secret {
            key, encrypted_value, created_at: ts, updated_at: ts, tags,
        });
    }

    pub fn get(&self, key: &str) -> Option<&Secret> { self.secrets.get(key) }

    pub fn delete(&mut self, key: &str) -> bool { self.secrets.remove(key).is_some() }

    pub fn list_keys(&self) -> Vec<&str> { self.secrets.keys().map(|k| k.as_str()).collect() }

    pub fn find_by_tag(&self, tag: &str) -> Vec<&Secret> {
        self.secrets.values().filter(|s| s.tags.contains(&tag.to_string())).collect()
    }
}

impl Default for SecretsVault {
    fn default() -> Self { Self::new() }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default().as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passkey_lifecycle() {
        let mut mgr = PasskeyManager::new();
        let id = mgr.register(Passkey {
            credential_id: "cred1".into(), user_id: "user1".into(),
            public_key: vec![1, 2, 3], sign_count: 0, created_at: 0,
            last_used: 0, device_name: "Phone".into(),
        });

        let pk = mgr.authenticate(&id).unwrap();
        assert_eq!(pk.sign_count, 1);
        assert_eq!(mgr.list_for_user("user1").len(), 1);
    }

    #[test]
    fn test_device_pairing() {
        let mut mgr = PairingManager::new();
        let a = NodeId::new();
        let b = NodeId::new();

        let (sid, code) = mgr.initiate(a.clone(), b.clone());
        assert!(mgr.verify(&sid, &code).unwrap());
        mgr.complete(&sid).unwrap();
        assert!(mgr.is_paired(&a, &b));
    }

    #[test]
    fn test_secrets_vault() {
        let mut vault = SecretsVault::new();
        vault.store("api_key".into(), vec![0xDE, 0xAD], vec!["production".into()]);

        assert!(vault.get("api_key").is_some());
        assert_eq!(vault.find_by_tag("production").len(), 1);
        assert!(vault.delete("api_key"));
        assert!(vault.get("api_key").is_none());
    }
}
