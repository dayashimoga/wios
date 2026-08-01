//! Session management with token-based authentication.

use chrono::{DateTime, Duration, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// A session token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub node_id: NodeId,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub is_active: bool,
}

/// Session manager for handling authentication tokens.
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    session_timeout_secs: i64,
    max_sessions_per_node: usize,
}

impl SessionManager {
    /// Create a new session manager.
    pub fn new(session_timeout_secs: u64, max_sessions_per_node: usize) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_timeout_secs: session_timeout_secs as i64,
            max_sessions_per_node,
        }
    }

    /// Create a new session for a node.
    pub async fn create_session(
        &self,
        node_id: &NodeId,
        ip_address: Option<String>,
    ) -> WiosResult<Session> {
        let mut sessions = self.sessions.write().await;

        // Enforce max sessions per node
        let node_sessions: usize = sessions
            .values()
            .filter(|s| s.node_id == *node_id && s.is_active)
            .count();
        if node_sessions >= self.max_sessions_per_node {
            return Err(WiosError::AuthFailed(
                "Maximum active sessions reached".into(),
            ));
        }

        let token = generate_token();
        let now = Utc::now();
        let session = Session {
            token: token.clone(),
            node_id: node_id.clone(),
            created_at: now,
            expires_at: now + Duration::seconds(self.session_timeout_secs),
            last_activity: now,
            ip_address,
            is_active: true,
        };

        sessions.insert(token, session.clone());
        Ok(session)
    }

    /// Validate a session token and refresh its expiry.
    pub async fn validate_and_refresh(&self, token: &str) -> WiosResult<Session> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(token)
            .ok_or(WiosError::TokenExpired)?;

        if !session.is_active || Utc::now() > session.expires_at {
            session.is_active = false;
            return Err(WiosError::TokenExpired);
        }

        session.last_activity = Utc::now();
        session.expires_at = Utc::now() + Duration::seconds(self.session_timeout_secs);
        Ok(session.clone())
    }

    /// Revoke a session.
    pub async fn revoke(&self, token: &str) -> WiosResult<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(token) {
            session.is_active = false;
            Ok(())
        } else {
            Err(WiosError::TokenExpired)
        }
    }

    /// Revoke all sessions for a node.
    pub async fn revoke_all(&self, node_id: &NodeId) {
        let mut sessions = self.sessions.write().await;
        for session in sessions.values_mut() {
            if session.node_id == *node_id {
                session.is_active = false;
            }
        }
    }

    /// Cleanup expired sessions.
    pub async fn cleanup_expired(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let now = Utc::now();
        let expired: Vec<String> = sessions
            .iter()
            .filter(|(_, s)| !s.is_active || now > s.expires_at)
            .map(|(k, _)| k.clone())
            .collect();
        let count = expired.len();
        for token in expired {
            sessions.remove(&token);
        }
        count
    }

    /// Get active session count.
    pub async fn active_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.values().filter(|s| s.is_active).count()
    }
}

/// Generate a cryptographically secure random token.
fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex_encode(&bytes)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_lifecycle() {
        let mgr = SessionManager::new(3600, 5);
        let node = NodeId::new();

        let session = mgr.create_session(&node, None).await.unwrap();
        assert!(session.is_active);

        let validated = mgr.validate_and_refresh(&session.token).await.unwrap();
        assert_eq!(validated.node_id, node);

        mgr.revoke(&session.token).await.unwrap();
        assert!(mgr.validate_and_refresh(&session.token).await.is_err());
    }

    #[tokio::test]
    async fn test_max_sessions() {
        let mgr = SessionManager::new(3600, 2);
        let node = NodeId::new();

        mgr.create_session(&node, None).await.unwrap();
        mgr.create_session(&node, None).await.unwrap();
        assert!(mgr.create_session(&node, None).await.is_err());
    }

    #[tokio::test]
    async fn test_revoke_all() {
        let mgr = SessionManager::new(3600, 10);
        let node = NodeId::new();

        mgr.create_session(&node, None).await.unwrap();
        mgr.create_session(&node, None).await.unwrap();
        assert_eq!(mgr.active_count().await, 2);

        mgr.revoke_all(&node).await;
        assert_eq!(mgr.active_count().await, 0);
    }
}
