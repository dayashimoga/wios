//! Audit logging framework.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use wios_core::types::NodeId;

/// An audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique entry ID.
    pub id: String,
    /// When the action occurred.
    pub timestamp: DateTime<Utc>,
    /// Who performed the action.
    pub actor: NodeId,
    /// What action was performed.
    pub action: AuditAction,
    /// Target resource.
    pub target: Option<String>,
    /// Whether the action succeeded.
    pub success: bool,
    /// Additional details.
    pub details: Option<String>,
    /// Source IP/address.
    pub source_address: Option<String>,
}

/// Categorized audit actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    // Auth
    Login,
    Logout,
    LoginFailed,
    PasswordChanged,
    KeyGenerated,

    // Network
    PeerConnected,
    PeerDisconnected,
    MessageSent,
    FileTransferred,

    // Storage
    DataCreated,
    DataUpdated,
    DataDeleted,

    // Admin
    RoleAssigned,
    RoleRevoked,
    ConfigChanged,
    PluginInstalled,
    PluginRemoved,

    // Custom
    Custom(String),
}

/// Audit logger that records security-relevant events.
pub struct AuditLogger {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
    max_entries: usize,
}

impl AuditLogger {
    /// Create a new audit logger.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            max_entries,
        }
    }

    /// Log an audit event.
    pub async fn log(
        &self,
        actor: NodeId,
        action: AuditAction,
        target: Option<String>,
        success: bool,
        details: Option<String>,
    ) {
        let entry = AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            actor: actor.clone(),
            action: action.clone(),
            target: target.clone(),
            success,
            details: details.clone(),
            source_address: None,
        };

        info!(
            actor = %actor,
            action = ?action,
            target = ?target,
            success = success,
            "Audit: {:?}",
            action
        );

        let mut entries = self.entries.write().await;
        entries.push(entry);

        // Enforce max entries (FIFO)
        if entries.len() > self.max_entries {
            let drain_count = entries.len() - self.max_entries;
            entries.drain(0..drain_count);
        }
    }

    /// Query audit log entries.
    pub async fn query(
        &self,
        actor: Option<&NodeId>,
        limit: usize,
    ) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries
            .iter()
            .rev()
            .filter(|e| actor.map_or(true, |a| &e.actor == a))
            .take(limit)
            .cloned()
            .collect()
    }

    /// Get total entry count.
    pub async fn count(&self) -> usize {
        self.entries.read().await.len()
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new(100_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_logging() {
        let logger = AuditLogger::new(100);
        let node = NodeId::new();

        logger
            .log(node.clone(), AuditAction::Login, None, true, None)
            .await;

        assert_eq!(logger.count().await, 1);

        let entries = logger.query(Some(&node), 10).await;
        assert_eq!(entries.len(), 1);
        assert!(entries[0].success);
    }

    #[tokio::test]
    async fn test_max_entries() {
        let logger = AuditLogger::new(5);
        let node = NodeId::new();

        for _ in 0..10 {
            logger
                .log(node.clone(), AuditAction::MessageSent, None, true, None)
                .await;
        }

        assert_eq!(logger.count().await, 5);
    }
}
