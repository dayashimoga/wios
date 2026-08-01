//! Content versioning — version history for stored data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};

/// A version entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub version: u64,
    pub hash: String,
    pub size_bytes: u64,
    pub author: String,
    pub message: String,
    pub timestamp: u64,
    pub parent: Option<u64>,
}

/// Version history for a single key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionHistory {
    pub key: String,
    pub current: u64,
    pub versions: Vec<Version>,
}

/// Version manager.
pub struct VersionManager {
    histories: HashMap<String, VersionHistory>,
    max_versions: usize,
}

impl VersionManager {
    pub fn new(max_versions: usize) -> Self {
        Self {
            histories: HashMap::new(),
            max_versions,
        }
    }

    /// Commit a new version.
    pub fn commit(
        &mut self,
        key: &str,
        hash: String,
        size_bytes: u64,
        author: String,
        message: String,
    ) -> u64 {
        let history = self
            .histories
            .entry(key.to_string())
            .or_insert(VersionHistory {
                key: key.to_string(),
                current: 0,
                versions: Vec::new(),
            });
        let version = history.current + 1;
        let parent = if version > 1 { Some(version - 1) } else { None };
        history.versions.push(Version {
            version,
            hash,
            size_bytes,
            author,
            message,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            parent,
        });
        history.current = version;

        // Prune old versions
        if history.versions.len() > self.max_versions {
            history
                .versions
                .drain(..history.versions.len() - self.max_versions);
        }
        version
    }

    /// Get version history for a key.
    pub fn history(&self, key: &str) -> Option<&VersionHistory> {
        self.histories.get(key)
    }

    /// Rollback to a specific version.
    pub fn rollback(&mut self, key: &str, target_version: u64) -> WiosResult<&Version> {
        let history = self.histories.get_mut(key).ok_or(WiosError::NotFound {
            entity: "history".into(),
            id: key.into(),
        })?;
        let version = history
            .versions
            .iter()
            .find(|v| v.version == target_version)
            .ok_or(WiosError::NotFound {
                entity: "version".into(),
                id: target_version.to_string(),
            })?;
        history.current = target_version;
        Ok(version)
    }

    /// Get current version info.
    pub fn current(&self, key: &str) -> Option<&Version> {
        let history = self.histories.get(key)?;
        history
            .versions
            .iter()
            .find(|v| v.version == history.current)
    }

    pub fn count(&self) -> usize {
        self.histories.len()
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_versioning() {
        let mut mgr = VersionManager::new(10);
        let v1 = mgr.commit(
            "file.txt",
            "abc123".into(),
            100,
            "user".into(),
            "Initial".into(),
        );
        let v2 = mgr.commit(
            "file.txt",
            "def456".into(),
            150,
            "user".into(),
            "Update".into(),
        );

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        assert_eq!(mgr.current("file.txt").unwrap().hash, "def456");
    }

    #[test]
    fn test_rollback() {
        let mut mgr = VersionManager::new(10);
        mgr.commit("doc", "v1".into(), 100, "a".into(), "v1".into());
        mgr.commit("doc", "v2".into(), 200, "a".into(), "v2".into());

        mgr.rollback("doc", 1).unwrap();
        assert_eq!(mgr.history("doc").unwrap().current, 1);
    }

    #[test]
    fn test_pruning() {
        let mut mgr = VersionManager::new(3);
        for i in 1..=5 {
            mgr.commit(
                "key",
                format!("h{}", i),
                i * 10,
                "a".into(),
                format!("v{}", i),
            );
        }
        assert_eq!(mgr.history("key").unwrap().versions.len(), 3);
    }
}
