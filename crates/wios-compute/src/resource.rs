//! Resource discovery and management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::types::{DeviceCapabilities, NodeId};

/// Manages available compute resources across the mesh.
pub struct ResourceManager {
    resources: Arc<RwLock<HashMap<String, DeviceCapabilities>>>,
}

impl ResourceManager {
    /// Create a new resource manager.
    pub fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a node's capabilities.
    pub async fn register(&self, node_id: &NodeId, capabilities: DeviceCapabilities) {
        self.resources.write().await.insert(node_id.as_str().to_string(), capabilities);
    }

    /// Unregister a node.
    pub async fn unregister(&self, node_id: &NodeId) {
        self.resources.write().await.remove(node_id.as_str());
    }

    /// Get all available resources.
    pub async fn available(&self) -> HashMap<String, DeviceCapabilities> {
        self.resources.read().await.clone()
    }

    /// Find nodes with specific capabilities.
    pub async fn find_capable(&self, requires_gpu: bool, min_ram_mb: u64) -> Vec<String> {
        self.resources.read().await
            .iter()
            .filter(|(_, cap)| {
                (!requires_gpu || cap.has_gpu) && cap.ram_mb >= min_ram_mb
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get total mesh resources.
    pub async fn total_resources(&self) -> MeshResources {
        let resources = self.resources.read().await;
        MeshResources {
            total_nodes: resources.len(),
            total_cpu_cores: resources.values().map(|c| c.cpu_cores).sum(),
            total_ram_mb: resources.values().map(|c| c.ram_mb).sum(),
            total_storage_mb: resources.values().map(|c| c.storage_mb).sum(),
            gpu_nodes: resources.values().filter(|c| c.has_gpu).count(),
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregate mesh resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshResources {
    pub total_nodes: usize,
    pub total_cpu_cores: u32,
    pub total_ram_mb: u64,
    pub total_storage_mb: u64,
    pub gpu_nodes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_management() {
        let rm = ResourceManager::new();
        let node_id = NodeId::new();
        let caps = DeviceCapabilities {
            cpu_cores: 8,
            ram_mb: 16384,
            storage_mb: 512000,
            has_gpu: true,
            ..Default::default()
        };

        rm.register(&node_id, caps).await;
        let total = rm.total_resources().await;
        assert_eq!(total.total_nodes, 1);
        assert_eq!(total.total_cpu_cores, 8);
        assert_eq!(total.gpu_nodes, 1);

        let gpu_nodes = rm.find_capable(true, 8192).await;
        assert_eq!(gpu_nodes.len(), 1);
    }
}
