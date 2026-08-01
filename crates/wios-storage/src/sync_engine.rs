//! CRDT-based sync engine for offline-first conflict resolution.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A sync vector clock for tracking causal ordering.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorClock {
    pub clocks: HashMap<String, u64>,
}

impl VectorClock {
    /// Increment the clock for a node.
    pub fn increment(&mut self, node_id: &str) {
        let counter = self.clocks.entry(node_id.to_string()).or_insert(0);
        *counter += 1;
    }

    /// Merge with another vector clock (take max of each).
    pub fn merge(&mut self, other: &VectorClock) {
        for (node, &count) in &other.clocks {
            let entry = self.clocks.entry(node.clone()).or_insert(0);
            *entry = (*entry).max(count);
        }
    }

    /// Check if this clock happened before another.
    pub fn happened_before(&self, other: &VectorClock) -> bool {
        let mut at_least_one_less = false;
        for (node, &count) in &other.clocks {
            let our_count = self.clocks.get(node).copied().unwrap_or(0);
            if our_count > count {
                return false;
            }
            if our_count < count {
                at_least_one_less = true;
            }
        }
        // Also check nodes we have that other doesn't
        for (node, &count) in &self.clocks {
            if !other.clocks.contains_key(node) && count > 0 {
                return false;
            }
        }
        at_least_one_less
    }

    /// Check if two clocks are concurrent (neither happened before the other).
    pub fn is_concurrent(&self, other: &VectorClock) -> bool {
        !self.happened_before(other) && !other.happened_before(self) && self.clocks != other.clocks
    }
}

/// A sync operation record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    pub namespace: String,
    pub key: String,
    pub value: Option<Vec<u8>>,
    pub operation: SyncOperation,
    pub clock: VectorClock,
    pub node_id: String,
    pub timestamp: DateTime<Utc>,
}

/// Type of sync operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOperation {
    Put,
    Delete,
}

/// Sync engine that manages offline-first replication.
pub struct SyncEngine {
    node_id: String,
    clock: VectorClock,
    pending: Vec<SyncRecord>,
}

impl SyncEngine {
    /// Create a new sync engine for a node.
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            clock: VectorClock::default(),
            pending: Vec::new(),
        }
    }

    /// Record a local write operation.
    pub fn record_write(&mut self, namespace: &str, key: &str, value: Vec<u8>) -> SyncRecord {
        self.clock.increment(&self.node_id);
        let record = SyncRecord {
            namespace: namespace.into(),
            key: key.into(),
            value: Some(value),
            operation: SyncOperation::Put,
            clock: self.clock.clone(),
            node_id: self.node_id.clone(),
            timestamp: Utc::now(),
        };
        self.pending.push(record.clone());
        record
    }

    /// Record a local delete operation.
    pub fn record_delete(&mut self, namespace: &str, key: &str) -> SyncRecord {
        self.clock.increment(&self.node_id);
        let record = SyncRecord {
            namespace: namespace.into(),
            key: key.into(),
            value: None,
            operation: SyncOperation::Delete,
            clock: self.clock.clone(),
            node_id: self.node_id.clone(),
            timestamp: Utc::now(),
        };
        self.pending.push(record.clone());
        record
    }

    /// Get pending sync records.
    pub fn pending_records(&self) -> &[SyncRecord] {
        &self.pending
    }

    /// Clear pending records after successful sync.
    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }

    /// Merge a remote sync record, resolving conflicts using LWW (Last Writer Wins).
    pub fn merge_remote(&mut self, remote: &SyncRecord) -> MergeResult {
        let result = if remote.clock.happened_before(&self.clock) {
            // Remote change is older, keep local
            MergeResult::KeepLocal
        } else if self.clock.happened_before(&remote.clock) {
            // Remote is newer, accept remote
            MergeResult::AcceptRemote
        } else {
            // Concurrent: use LWW with node_id as tiebreaker
            if remote.node_id > self.node_id {
                MergeResult::AcceptRemote
            } else {
                MergeResult::KeepLocal
            }
        };

        // Merge clocks AFTER comparison to preserve causal ordering check
        self.clock.merge(&remote.clock);
        result
    }

    /// Get current vector clock.
    pub fn current_clock(&self) -> &VectorClock {
        &self.clock
    }
}

/// Result of merging a remote sync record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeResult {
    AcceptRemote,
    KeepLocal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_clock_increment() {
        let mut clock = VectorClock::default();
        clock.increment("node1");
        assert_eq!(clock.clocks.get("node1"), Some(&1));
        clock.increment("node1");
        assert_eq!(clock.clocks.get("node1"), Some(&2));
    }

    #[test]
    fn test_vector_clock_merge() {
        let mut c1 = VectorClock::default();
        c1.increment("node1");
        c1.increment("node1");

        let mut c2 = VectorClock::default();
        c2.increment("node2");

        c1.merge(&c2);
        assert_eq!(c1.clocks.get("node1"), Some(&2));
        assert_eq!(c1.clocks.get("node2"), Some(&1));
    }

    #[test]
    fn test_happened_before() {
        let mut c1 = VectorClock::default();
        c1.increment("node1");

        let mut c2 = c1.clone();
        c2.increment("node1");

        assert!(c1.happened_before(&c2));
        assert!(!c2.happened_before(&c1));
    }

    #[test]
    fn test_concurrent() {
        let mut c1 = VectorClock::default();
        c1.increment("node1");

        let mut c2 = VectorClock::default();
        c2.increment("node2");

        assert!(c1.is_concurrent(&c2));
    }

    #[test]
    fn test_sync_engine_write() {
        let mut engine = SyncEngine::new("node1");
        engine.record_write("test", "key1", b"value".to_vec());
        assert_eq!(engine.pending_records().len(), 1);
    }

    #[test]
    fn test_sync_engine_merge() {
        let mut engine1 = SyncEngine::new("node1");
        let mut engine2 = SyncEngine::new("node2");

        let _record1 = engine1.record_write("test", "key1", b"v1".to_vec());
        let record2 = engine2.record_write("test", "key1", b"v2".to_vec());

        // Concurrent writes — LWW with node_id tiebreaker
        let result = engine1.merge_remote(&record2);
        assert_eq!(result, MergeResult::AcceptRemote); // "node2" > "node1"
    }
}
