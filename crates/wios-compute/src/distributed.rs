//! Mesh-distributed task scheduler — distributes compute tasks across peers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// A distributed task that can run on any capable mesh node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedTask {
    pub id: String,
    pub name: String,
    pub payload: Vec<u8>,
    pub required_cpu: u32,
    pub required_ram_mb: u64,
    pub requires_gpu: bool,
    pub submitted_by: NodeId,
    pub assigned_to: Option<NodeId>,
    pub status: DistributedTaskStatus,
    pub submitted_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<Vec<u8>>,
}

/// Status of a distributed task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributedTaskStatus {
    Queued,
    Assigned,
    Running,
    Completed,
    Failed,
}

/// Node hardware capabilities: (cpu_cores, ram_mb, has_gpu).
pub type NodeCapability = (u32, u64, bool);

/// Mesh task distributor — assigns tasks to capable nodes.
pub struct TaskDistributor {
    tasks: Arc<RwLock<HashMap<String, DistributedTask>>>,
    capabilities: Arc<RwLock<HashMap<String, NodeCapability>>>,
}

impl TaskDistributor {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            capabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a node's capabilities.
    pub async fn register_node(&self, node_id: &NodeId, cpu: u32, ram_mb: u64, gpu: bool) {
        self.capabilities
            .write()
            .await
            .insert(node_id.as_str().to_string(), (cpu, ram_mb, gpu));
    }

    /// Remove a node (e.g., when it goes offline).
    pub async fn unregister_node(&self, node_id: &NodeId) {
        self.capabilities.write().await.remove(node_id.as_str());
    }

    /// Submit a task for distribution.
    pub async fn submit(&self, task: DistributedTask) -> WiosResult<String> {
        let id = task.id.clone();
        self.tasks.write().await.insert(id.clone(), task);
        Ok(id)
    }

    /// Find the best node for a queued task and assign it.
    pub async fn assign_next(&self) -> Option<(String, NodeId)> {
        let mut tasks = self.tasks.write().await;
        let caps = self.capabilities.read().await;

        // Find first queued task
        let task_id = tasks
            .iter()
            .find(|(_, t)| t.status == DistributedTaskStatus::Queued)
            .map(|(id, _)| id.clone())?;

        let task = tasks.get(&task_id)?;

        // Find best capable node (most resources)
        let best_node = caps
            .iter()
            .filter(|(_, (cpu, ram, gpu))| {
                *cpu >= task.required_cpu
                    && *ram >= task.required_ram_mb
                    && (!task.requires_gpu || *gpu)
            })
            .max_by_key(|(_, (cpu, ram, _))| (*cpu as u64) * 1000 + ram)
            .map(|(id, _)| id.clone())?;

        let task = tasks.get_mut(&task_id)?;
        task.status = DistributedTaskStatus::Assigned;
        task.assigned_to = Some(NodeId::from_string(best_node.clone()));

        Some((task_id, NodeId::from_string(best_node)))
    }

    /// Mark a task as completed with result.
    pub async fn complete(&self, task_id: &str, result: Vec<u8>) -> WiosResult<()> {
        let mut tasks = self.tasks.write().await;
        let task = tasks
            .get_mut(task_id)
            .ok_or(WiosError::TaskNotFound(task_id.into()))?;
        task.status = DistributedTaskStatus::Completed;
        task.completed_at = Some(Utc::now());
        task.result = Some(result);
        Ok(())
    }

    /// Mark a task as failed.
    pub async fn fail(&self, task_id: &str) -> WiosResult<()> {
        let mut tasks = self.tasks.write().await;
        let task = tasks
            .get_mut(task_id)
            .ok_or(WiosError::TaskNotFound(task_id.into()))?;
        task.status = DistributedTaskStatus::Failed;
        task.assigned_to = None;
        Ok(())
    }

    /// Get task status.
    pub async fn status(&self, task_id: &str) -> Option<DistributedTaskStatus> {
        self.tasks
            .read()
            .await
            .get(task_id)
            .map(|t| t.status.clone())
    }

    /// Get mesh stats.
    pub async fn stats(&self) -> (usize, usize, usize) {
        let tasks = self.tasks.read().await;
        let queued = tasks
            .values()
            .filter(|t| t.status == DistributedTaskStatus::Queued)
            .count();
        let running = tasks
            .values()
            .filter(|t| {
                t.status == DistributedTaskStatus::Running
                    || t.status == DistributedTaskStatus::Assigned
            })
            .count();
        let completed = tasks
            .values()
            .filter(|t| t.status == DistributedTaskStatus::Completed)
            .count();
        (queued, running, completed)
    }
}

impl Default for TaskDistributor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_distribution() {
        let dist = TaskDistributor::new();
        let node1 = NodeId::new();
        let node2 = NodeId::new();

        dist.register_node(&node1, 4, 8192, false).await;
        dist.register_node(&node2, 16, 32768, true).await;

        let task = DistributedTask {
            id: "t1".into(),
            name: "heavy-compute".into(),
            payload: vec![],
            required_cpu: 8,
            required_ram_mb: 16384,
            requires_gpu: true,
            submitted_by: NodeId::new(),
            assigned_to: None,
            status: DistributedTaskStatus::Queued,
            submitted_at: Utc::now(),
            completed_at: None,
            result: None,
        };

        dist.submit(task).await.unwrap();
        let (task_id, assigned) = dist.assign_next().await.unwrap();
        assert_eq!(task_id, "t1");
        assert_eq!(assigned, node2); // node2 has GPU + more resources
    }

    #[tokio::test]
    async fn test_no_capable_node() {
        let dist = TaskDistributor::new();
        let node = NodeId::new();
        dist.register_node(&node, 2, 4096, false).await;

        let task = DistributedTask {
            id: "t1".into(),
            name: "gpu-task".into(),
            payload: vec![],
            required_cpu: 1,
            required_ram_mb: 1024,
            requires_gpu: true, // no GPU nodes
            submitted_by: NodeId::new(),
            assigned_to: None,
            status: DistributedTaskStatus::Queued,
            submitted_at: Utc::now(),
            completed_at: None,
            result: None,
        };

        dist.submit(task).await.unwrap();
        assert!(dist.assign_next().await.is_none());
    }

    #[tokio::test]
    async fn test_complete_task() {
        let dist = TaskDistributor::new();
        let node = NodeId::new();
        dist.register_node(&node, 8, 16384, false).await;

        let task = DistributedTask {
            id: "t1".into(),
            name: "test".into(),
            payload: vec![],
            required_cpu: 1,
            required_ram_mb: 1024,
            requires_gpu: false,
            submitted_by: NodeId::new(),
            assigned_to: None,
            status: DistributedTaskStatus::Queued,
            submitted_at: Utc::now(),
            completed_at: None,
            result: None,
        };
        dist.submit(task).await.unwrap();
        dist.assign_next().await;
        dist.complete("t1", b"result".to_vec()).await.unwrap();

        assert_eq!(
            dist.status("t1").await.unwrap(),
            DistributedTaskStatus::Completed
        );
    }
}
