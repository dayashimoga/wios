//! Backup/restore — snapshot-based data backup.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};

/// A snapshot of data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub name: String,
    pub keys: Vec<String>,
    pub total_bytes: u64,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}

/// Backup manager.
pub struct BackupManager {
    snapshots: Vec<Snapshot>,
    max_snapshots: usize,
}

impl BackupManager {
    pub fn new(max_snapshots: usize) -> Self {
        Self { snapshots: Vec::new(), max_snapshots }
    }

    /// Create a snapshot.
    pub fn create_snapshot(&mut self, name: String, keys: Vec<String>, total_bytes: u64) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let snapshot = Snapshot {
            id: id.clone(), name, keys, total_bytes,
            created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default().as_secs(),
            metadata: HashMap::new(),
        };
        self.snapshots.push(snapshot);
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }
        id
    }

    /// List all snapshots.
    pub fn list(&self) -> &[Snapshot] { &self.snapshots }

    /// Get a snapshot by ID.
    pub fn get(&self, id: &str) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }

    /// Delete a snapshot.
    pub fn delete(&mut self, id: &str) -> WiosResult<()> {
        let pos = self.snapshots.iter().position(|s| s.id == id)
            .ok_or(WiosError::NotFound { entity: "snapshot".into(), id: id.into() })?;
        self.snapshots.remove(pos);
        Ok(())
    }

    /// Get total backup size.
    pub fn total_size(&self) -> u64 {
        self.snapshots.iter().map(|s| s.total_bytes).sum()
    }
}

impl Default for BackupManager {
    fn default() -> Self { Self::new(10) }
}

/// System monitoring metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub storage_used_mb: u64,
    pub storage_total_mb: u64,
    pub active_peers: u32,
    pub messages_per_sec: f64,
    pub active_tasks: u32,
    pub uptime_secs: u64,
}

/// Metrics collector.
pub struct MetricsCollector {
    history: Vec<(u64, SystemMetrics)>,
    max_entries: usize,
}

impl MetricsCollector {
    pub fn new(max_entries: usize) -> Self {
        Self { history: Vec::new(), max_entries }
    }

    pub fn record(&mut self, metrics: SystemMetrics) {
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default().as_secs();
        self.history.push((ts, metrics));
        if self.history.len() > self.max_entries {
            self.history.remove(0);
        }
    }

    pub fn latest(&self) -> Option<&SystemMetrics> {
        self.history.last().map(|(_, m)| m)
    }

    pub fn history(&self, last_n: usize) -> Vec<&SystemMetrics> {
        self.history.iter().rev().take(last_n).map(|(_, m)| m).collect()
    }

    pub fn average_cpu(&self, last_n: usize) -> f64 {
        let samples: Vec<_> = self.history.iter().rev().take(last_n).collect();
        if samples.is_empty() { return 0.0; }
        samples.iter().map(|(_, m)| m.cpu_usage).sum::<f64>() / samples.len() as f64
    }
}

impl Default for MetricsCollector {
    fn default() -> Self { Self::new(1000) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_lifecycle() {
        let mut mgr = BackupManager::new(5);
        let id = mgr.create_snapshot("daily".into(), vec!["config".into(), "data".into()], 1024);
        assert_eq!(mgr.list().len(), 1);
        assert_eq!(mgr.get(&id).unwrap().keys.len(), 2);

        mgr.delete(&id).unwrap();
        assert_eq!(mgr.list().len(), 0);
    }

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new(100);
        collector.record(SystemMetrics {
            cpu_usage: 45.0, memory_used_mb: 4096, memory_total_mb: 8192,
            storage_used_mb: 1024, storage_total_mb: 51200,
            active_peers: 3, messages_per_sec: 128.0, active_tasks: 2, uptime_secs: 3600,
        });
        collector.record(SystemMetrics {
            cpu_usage: 55.0, memory_used_mb: 4500, memory_total_mb: 8192,
            storage_used_mb: 1100, storage_total_mb: 51200,
            active_peers: 4, messages_per_sec: 150.0, active_tasks: 3, uptime_secs: 7200,
        });

        assert_eq!(collector.latest().unwrap().cpu_usage, 55.0);
        assert!((collector.average_cpu(2) - 50.0).abs() < 0.01);
    }
}
