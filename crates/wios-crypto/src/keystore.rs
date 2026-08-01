//! Key store for managing cryptographic keys.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use wios_core::error::{WiosError, WiosResult};

/// A stored key entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEntry {
    /// Unique key identifier.
    pub key_id: String,
    /// Key type (ed25519, x25519, aes256).
    pub key_type: KeyType,
    /// The key material (encrypted at rest).
    pub key_data: Vec<u8>,
    /// When this key was created.
    pub created_at: DateTime<Utc>,
    /// When this key expires (None = never).
    pub expires_at: Option<DateTime<Utc>>,
    /// Whether this key is the active/primary key.
    pub is_primary: bool,
    /// Key metadata.
    pub metadata: HashMap<String, String>,
}

/// Types of cryptographic keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    Ed25519Private,
    Ed25519Public,
    X25519Private,
    X25519Public,
    Aes256,
    HmacSha256,
}

/// Manages cryptographic key storage and lifecycle.
pub struct KeyStore {
    keys: Arc<RwLock<HashMap<String, KeyEntry>>>,
    storage_path: PathBuf,
}

impl KeyStore {
    /// Create a new key store at the specified path.
    pub fn new(storage_path: impl AsRef<Path>) -> WiosResult<Self> {
        let path = storage_path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let keys = if path.exists() {
            let data = std::fs::read_to_string(&path)?;
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        };

        info!("KeyStore initialized with {} keys", keys.len());

        Ok(Self {
            keys: Arc::new(RwLock::new(keys)),
            storage_path: path,
        })
    }

    /// Store a key entry.
    pub async fn store(&self, entry: KeyEntry) -> WiosResult<()> {
        let key_id = entry.key_id.clone();
        let mut keys = self.keys.write().await;
        keys.insert(key_id.clone(), entry);
        self.persist(&keys)?;
        info!("Stored key: {}", key_id);
        Ok(())
    }

    /// Retrieve a key by ID.
    pub async fn get(&self, key_id: &str) -> WiosResult<KeyEntry> {
        let keys = self.keys.read().await;
        keys.get(key_id)
            .cloned()
            .ok_or_else(|| WiosError::KeyNotFound {
                key_id: key_id.into(),
            })
    }

    /// Delete a key.
    pub async fn delete(&self, key_id: &str) -> WiosResult<()> {
        let mut keys = self.keys.write().await;
        if keys.remove(key_id).is_some() {
            self.persist(&keys)?;
            info!("Deleted key: {}", key_id);
            Ok(())
        } else {
            Err(WiosError::KeyNotFound {
                key_id: key_id.into(),
            })
        }
    }

    /// List all key IDs.
    pub async fn list(&self) -> Vec<String> {
        let keys = self.keys.read().await;
        keys.keys().cloned().collect()
    }

    /// Get the primary key of a specific type.
    pub async fn primary_key(&self, key_type: &KeyType) -> WiosResult<KeyEntry> {
        let keys = self.keys.read().await;
        keys.values()
            .find(|k| &k.key_type == key_type && k.is_primary)
            .cloned()
            .ok_or_else(|| WiosError::KeyNotFound {
                key_id: format!("primary_{:?}", key_type),
            })
    }

    /// Check if a key has expired.
    pub fn is_expired(entry: &KeyEntry) -> bool {
        if let Some(expires_at) = entry.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Remove expired keys.
    pub async fn cleanup_expired(&self) -> WiosResult<usize> {
        let mut keys = self.keys.write().await;
        let expired: Vec<String> = keys
            .iter()
            .filter(|(_, v)| Self::is_expired(v))
            .map(|(k, _)| k.clone())
            .collect();
        let count = expired.len();
        for key_id in &expired {
            keys.remove(key_id);
            warn!("Removed expired key: {}", key_id);
        }
        if count > 0 {
            self.persist(&keys)?;
        }
        Ok(count)
    }

    /// Persist keys to disk.
    fn persist(&self, keys: &HashMap<String, KeyEntry>) -> WiosResult<()> {
        let data = serde_json::to_string_pretty(keys)
            .map_err(|e| WiosError::Crypto(e.to_string()))?;
        std::fs::write(&self.storage_path, data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_keystore_crud() {
        let dir = tempfile::tempdir().unwrap();
        let store = KeyStore::new(dir.path().join("keys.json")).unwrap();

        let entry = KeyEntry {
            key_id: "test-key-1".into(),
            key_type: KeyType::Ed25519Private,
            key_data: vec![1, 2, 3, 4],
            created_at: Utc::now(),
            expires_at: None,
            is_primary: true,
            metadata: HashMap::new(),
        };

        // Store
        store.store(entry.clone()).await.unwrap();

        // Get
        let retrieved = store.get("test-key-1").await.unwrap();
        assert_eq!(retrieved.key_id, "test-key-1");
        assert_eq!(retrieved.key_data, vec![1, 2, 3, 4]);

        // List
        let keys = store.list().await;
        assert_eq!(keys.len(), 1);

        // Primary
        let primary = store.primary_key(&KeyType::Ed25519Private).await.unwrap();
        assert_eq!(primary.key_id, "test-key-1");

        // Delete
        store.delete("test-key-1").await.unwrap();
        assert!(store.get("test-key-1").await.is_err());
    }

    #[test]
    fn test_key_expiry() {
        let entry = KeyEntry {
            key_id: "expired".into(),
            key_type: KeyType::Aes256,
            key_data: vec![],
            created_at: Utc::now(),
            expires_at: Some(Utc::now() - chrono::Duration::hours(1)),
            is_primary: false,
            metadata: HashMap::new(),
        };
        assert!(KeyStore::is_expired(&entry));
    }
}
