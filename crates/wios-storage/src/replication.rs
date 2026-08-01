//! Replication — multi-node data replication with consistency tracking.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// Replication state for a data item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicaState {
    pub key: String,
    pub version: u64,
    pub replicas: HashMap<String, ReplicaInfo>,
    pub desired_replicas: u32,
}

/// Info about a specific replica.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicaInfo {
    pub node_id: NodeId,
    pub version: u64,
    pub synced: bool,
    pub last_sync: u64,
}

/// Replication manager.
pub struct ReplicationManager {
    replicas: HashMap<String, ReplicaState>,
    local_node: NodeId,
}

impl ReplicationManager {
    pub fn new(local_node: NodeId) -> Self {
        Self { replicas: HashMap::new(), local_node }
    }

    /// Register data for replication.
    pub fn register(&mut self, key: String, desired_replicas: u32) {
        let mut state = ReplicaState {
            key: key.clone(), version: 1, replicas: HashMap::new(), desired_replicas,
        };
        state.replicas.insert(self.local_node.as_str().to_string(), ReplicaInfo {
            node_id: self.local_node.clone(), version: 1, synced: true, last_sync: now(),
        });
        self.replicas.insert(key, state);
    }

    /// Update local version.
    pub fn update(&mut self, key: &str) -> WiosResult<u64> {
        let state = self.replicas.get_mut(key)
            .ok_or(WiosError::NotFound { entity: "replica".into(), id: key.into() })?;
        state.version += 1;
        if let Some(local) = state.replicas.get_mut(self.local_node.as_str()) {
            local.version = state.version;
            local.last_sync = now();
        }
        Ok(state.version)
    }

    /// Record that a remote node has synced.
    pub fn ack_sync(&mut self, key: &str, node: NodeId, version: u64) -> WiosResult<()> {
        let state = self.replicas.get_mut(key)
            .ok_or(WiosError::NotFound { entity: "replica".into(), id: key.into() })?;
        let info = state.replicas.entry(node.as_str().to_string()).or_insert(ReplicaInfo {
            node_id: node, version: 0, synced: false, last_sync: 0,
        });
        info.version = version;
        info.synced = version >= state.version;
        info.last_sync = now();
        Ok(())
    }

    /// Get items needing more replicas.
    pub fn under_replicated(&self) -> Vec<&ReplicaState> {
        self.replicas.values()
            .filter(|s| (s.replicas.len() as u32) < s.desired_replicas)
            .collect()
    }

    /// Get items with stale replicas.
    pub fn stale_replicas(&self, key: &str) -> Vec<&ReplicaInfo> {
        self.replicas.get(key).map(|s| {
            s.replicas.values().filter(|r| !r.synced).collect()
        }).unwrap_or_default()
    }

    pub fn status(&self, key: &str) -> Option<&ReplicaState> { self.replicas.get(key) }
    pub fn count(&self) -> usize { self.replicas.len() }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default().as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replication_lifecycle() {
        let local = NodeId::new();
        let mut mgr = ReplicationManager::new(local);

        mgr.register("config".into(), 3);
        assert_eq!(mgr.under_replicated().len(), 1); // Only 1 replica, wants 3

        let remote = NodeId::new();
        mgr.ack_sync("config", remote, 1).unwrap();
        assert_eq!(mgr.status("config").unwrap().replicas.len(), 2);
    }

    #[test]
    fn test_version_update() {
        let local = NodeId::new();
        let mut mgr = ReplicationManager::new(local);
        mgr.register("data".into(), 2);

        let v = mgr.update("data").unwrap();
        assert_eq!(v, 2);

        let remote = NodeId::new();
        mgr.ack_sync("data", remote, 1).unwrap(); // Remote is behind
        assert_eq!(mgr.stale_replicas("data").len(), 1);
    }
}
