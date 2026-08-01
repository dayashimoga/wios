//! Role-Based Access Control (RBAC) engine.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// A named role with a set of permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub permissions: HashSet<Permission>,
    pub created_at: DateTime<Utc>,
}

/// Granular permissions for WIOS operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Network
    NetworkJoin,
    NetworkAdmin,
    PeerManage,
    MessageSend,
    MessageBroadcast,

    // Storage
    StorageRead,
    StorageWrite,
    StorageDelete,
    StorageAdmin,

    // AI
    AiInfer,
    AiModelManage,

    // Compute
    ComputeSubmit,
    ComputeAdmin,

    // Device
    DeviceShare,
    DeviceAccess,

    // Admin
    UserManage,
    RoleManage,
    ConfigManage,
    AuditView,
    SystemAdmin,

    // Plugin
    PluginInstall,
    PluginExecute,

    // Custom permission string
    Custom(String),
}

/// RBAC engine for managing roles and permissions.
pub struct RbacEngine {
    roles: Arc<RwLock<HashMap<String, Role>>>,
    assignments: Arc<RwLock<HashMap<String, HashSet<String>>>>, // node_id -> role_names
}

impl RbacEngine {
    /// Create a new RBAC engine with default roles.
    pub fn new() -> Self {
        let mut roles = HashMap::new();

        // Default admin role
        roles.insert(
            "admin".into(),
            Role {
                name: "admin".into(),
                description: "Full system administrator".into(),
                permissions: [
                    Permission::NetworkJoin,
                    Permission::NetworkAdmin,
                    Permission::PeerManage,
                    Permission::MessageSend,
                    Permission::MessageBroadcast,
                    Permission::StorageRead,
                    Permission::StorageWrite,
                    Permission::StorageDelete,
                    Permission::StorageAdmin,
                    Permission::AiInfer,
                    Permission::AiModelManage,
                    Permission::ComputeSubmit,
                    Permission::ComputeAdmin,
                    Permission::DeviceShare,
                    Permission::DeviceAccess,
                    Permission::UserManage,
                    Permission::RoleManage,
                    Permission::ConfigManage,
                    Permission::AuditView,
                    Permission::SystemAdmin,
                    Permission::PluginInstall,
                    Permission::PluginExecute,
                ]
                .into_iter()
                .collect(),
                created_at: Utc::now(),
            },
        );

        // Default user role
        roles.insert(
            "user".into(),
            Role {
                name: "user".into(),
                description: "Standard user".into(),
                permissions: [
                    Permission::NetworkJoin,
                    Permission::MessageSend,
                    Permission::StorageRead,
                    Permission::StorageWrite,
                    Permission::AiInfer,
                    Permission::ComputeSubmit,
                    Permission::DeviceShare,
                    Permission::PluginExecute,
                ]
                .into_iter()
                .collect(),
                created_at: Utc::now(),
            },
        );

        // Default guest role
        roles.insert(
            "guest".into(),
            Role {
                name: "guest".into(),
                description: "Limited guest access".into(),
                permissions: [
                    Permission::NetworkJoin,
                    Permission::MessageSend,
                    Permission::StorageRead,
                ]
                .into_iter()
                .collect(),
                created_at: Utc::now(),
            },
        );

        Self {
            roles: Arc::new(RwLock::new(roles)),
            assignments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if a node has a specific permission.
    pub async fn has_permission(&self, node_id: &NodeId, permission: &Permission) -> bool {
        let assignments = self.assignments.read().await;
        let roles = self.roles.read().await;

        if let Some(role_names) = assignments.get(node_id.as_str()) {
            for role_name in role_names {
                if let Some(role) = roles.get(role_name) {
                    if role.permissions.contains(permission) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Require a permission, returning an error if not granted.
    pub async fn require_permission(
        &self,
        node_id: &NodeId,
        permission: &Permission,
    ) -> WiosResult<()> {
        if self.has_permission(node_id, permission).await {
            Ok(())
        } else {
            Err(WiosError::AccessDenied(format!(
                "Node {} lacks permission {:?}",
                node_id, permission
            )))
        }
    }

    /// Assign a role to a node.
    pub async fn assign_role(&self, node_id: &NodeId, role_name: &str) -> WiosResult<()> {
        let roles = self.roles.read().await;
        if !roles.contains_key(role_name) {
            return Err(WiosError::InvalidPermissions(format!(
                "Role '{}' does not exist",
                role_name
            )));
        }
        drop(roles);

        let mut assignments = self.assignments.write().await;
        assignments
            .entry(node_id.as_str().to_string())
            .or_default()
            .insert(role_name.to_string());
        Ok(())
    }

    /// Revoke a role from a node.
    pub async fn revoke_role(&self, node_id: &NodeId, role_name: &str) -> WiosResult<()> {
        let mut assignments = self.assignments.write().await;
        if let Some(roles) = assignments.get_mut(node_id.as_str()) {
            roles.remove(role_name);
        }
        Ok(())
    }

    /// Create a custom role.
    pub async fn create_role(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        permissions: HashSet<Permission>,
    ) -> WiosResult<()> {
        let name = name.into();
        let mut roles = self.roles.write().await;
        roles.insert(
            name.clone(),
            Role {
                name,
                description: description.into(),
                permissions,
                created_at: Utc::now(),
            },
        );
        Ok(())
    }

    /// List all roles.
    pub async fn list_roles(&self) -> Vec<Role> {
        let roles = self.roles.read().await;
        roles.values().cloned().collect()
    }
}

impl Default for RbacEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_roles() {
        let rbac = RbacEngine::new();
        let roles = rbac.list_roles().await;
        assert!(roles.len() >= 3);
    }

    #[tokio::test]
    async fn test_permission_check() {
        let rbac = RbacEngine::new();
        let node_id = NodeId::new();

        // No role assigned yet
        assert!(
            !rbac
                .has_permission(&node_id, &Permission::MessageSend)
                .await
        );

        // Assign user role
        rbac.assign_role(&node_id, "user").await.unwrap();
        assert!(
            rbac.has_permission(&node_id, &Permission::MessageSend)
                .await
        );
        assert!(
            !rbac
                .has_permission(&node_id, &Permission::SystemAdmin)
                .await
        );

        // Assign admin role
        rbac.assign_role(&node_id, "admin").await.unwrap();
        assert!(
            rbac.has_permission(&node_id, &Permission::SystemAdmin)
                .await
        );
    }

    #[tokio::test]
    async fn test_require_permission() {
        let rbac = RbacEngine::new();
        let node_id = NodeId::new();
        rbac.assign_role(&node_id, "guest").await.unwrap();

        assert!(rbac
            .require_permission(&node_id, &Permission::StorageRead)
            .await
            .is_ok());
        assert!(rbac
            .require_permission(&node_id, &Permission::StorageWrite)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_custom_role() {
        let rbac = RbacEngine::new();
        let perms: HashSet<Permission> = [Permission::AiInfer, Permission::AiModelManage]
            .into_iter()
            .collect();
        rbac.create_role("ai_operator", "AI operations only", perms)
            .await
            .unwrap();

        let node_id = NodeId::new();
        rbac.assign_role(&node_id, "ai_operator").await.unwrap();
        assert!(rbac.has_permission(&node_id, &Permission::AiInfer).await);
        assert!(
            !rbac
                .has_permission(&node_id, &Permission::StorageRead)
                .await
        );
    }
}
