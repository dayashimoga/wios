//! Universal Device Bus — resource sharing across mesh devices.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// Shareable device capability.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceResource {
    Camera, Microphone, Display, Keyboard, Mouse,
    Storage, Printer, Scanner, Gpu, Cpu, Ram, Sensor(String),
}

/// Permission level for shared resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharePermission {
    ReadOnly, ReadWrite, FullControl,
}

/// A shared resource offering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResource {
    pub resource: DeviceResource,
    pub owner: NodeId,
    pub permission: SharePermission,
    pub active_users: Vec<NodeId>,
    pub max_users: u32,
    pub metadata: HashMap<String, String>,
}

/// Cross-device clipboard entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub content_type: String,
    pub data: Vec<u8>,
    pub source: NodeId,
    pub timestamp: u64,
}

/// Device Bus managing resource sharing and clipboard.
pub struct DeviceBus {
    shared: Arc<RwLock<HashMap<String, SharedResource>>>,
    clipboard: Arc<RwLock<Option<ClipboardEntry>>>,
}

impl DeviceBus {
    pub fn new() -> Self {
        Self {
            shared: Arc::new(RwLock::new(HashMap::new())),
            clipboard: Arc::new(RwLock::new(None)),
        }
    }

    /// Offer a resource for sharing.
    pub async fn share_resource(&self, resource: DeviceResource, owner: NodeId, permission: SharePermission, max_users: u32) -> String {
        let id = format!("{}-{:?}", owner.as_str(), resource);
        let shared = SharedResource {
            resource, owner, permission, active_users: Vec::new(), max_users,
            metadata: HashMap::new(),
        };
        self.shared.write().await.insert(id.clone(), shared);
        id
    }

    /// Request access to a shared resource.
    pub async fn request_access(&self, resource_id: &str, requester: NodeId) -> WiosResult<()> {
        let mut shared = self.shared.write().await;
        let res = shared.get_mut(resource_id)
            .ok_or(WiosError::NotFound { entity: "resource".into(), id: resource_id.into() })?;
        if res.active_users.len() as u32 >= res.max_users {
            return Err(WiosError::Storage("Resource at max capacity".into()));
        }
        if !res.active_users.contains(&requester) {
            res.active_users.push(requester);
        }
        Ok(())
    }

    /// Release access to a shared resource.
    pub async fn release_access(&self, resource_id: &str, node: &NodeId) -> WiosResult<()> {
        let mut shared = self.shared.write().await;
        if let Some(res) = shared.get_mut(resource_id) {
            res.active_users.retain(|n| n != node);
        }
        Ok(())
    }

    /// Unshare a resource (owner only).
    pub async fn unshare(&self, resource_id: &str, owner: &NodeId) -> WiosResult<()> {
        let mut shared = self.shared.write().await;
        let res = shared.get(resource_id)
            .ok_or(WiosError::NotFound { entity: "resource".into(), id: resource_id.into() })?;
        if res.owner != *owner {
            return Err(WiosError::Crypto("Only owner can unshare".into()));
        }
        shared.remove(resource_id);
        Ok(())
    }

    /// Set clipboard content.
    pub async fn set_clipboard(&self, entry: ClipboardEntry) {
        *self.clipboard.write().await = Some(entry);
    }

    /// Get clipboard content.
    pub async fn get_clipboard(&self) -> Option<ClipboardEntry> {
        self.clipboard.read().await.clone()
    }

    /// List all available shared resources.
    pub async fn list_resources(&self) -> Vec<SharedResource> {
        self.shared.read().await.values().cloned().collect()
    }

    /// Find resources by type.
    pub async fn find_by_type(&self, resource_type: &DeviceResource) -> Vec<SharedResource> {
        self.shared.read().await.values()
            .filter(|r| r.resource == *resource_type)
            .cloned()
            .collect()
    }
}

impl Default for DeviceBus {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_share_and_access() {
        let bus = DeviceBus::new();
        let owner = NodeId::new();
        let user = NodeId::new();

        let id = bus.share_resource(DeviceResource::Camera, owner.clone(), SharePermission::ReadOnly, 2).await;
        bus.request_access(&id, user.clone()).await.unwrap();

        let resources = bus.list_resources().await;
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].active_users.len(), 1);

        bus.release_access(&id, &user).await.unwrap();
        let resources = bus.list_resources().await;
        assert_eq!(resources[0].active_users.len(), 0);
    }

    #[tokio::test]
    async fn test_max_users() {
        let bus = DeviceBus::new();
        let owner = NodeId::new();
        let id = bus.share_resource(DeviceResource::Display, owner, SharePermission::ReadWrite, 1).await;

        bus.request_access(&id, NodeId::new()).await.unwrap();
        assert!(bus.request_access(&id, NodeId::new()).await.is_err());
    }

    #[tokio::test]
    async fn test_clipboard() {
        let bus = DeviceBus::new();
        assert!(bus.get_clipboard().await.is_none());

        bus.set_clipboard(ClipboardEntry {
            content_type: "text/plain".into(),
            data: b"hello".to_vec(),
            source: NodeId::new(),
            timestamp: 12345,
        }).await;

        let clip = bus.get_clipboard().await.unwrap();
        assert_eq!(clip.data, b"hello");
    }
}
