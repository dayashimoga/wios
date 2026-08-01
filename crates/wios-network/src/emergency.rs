//! Emergency SOS broadcast system.
//!
//! Provides priority-based emergency broadcasting across the mesh network
//! with automatic location attachment and escalation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// Emergency severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SosSeverity {
    /// Informational alert (weather, maintenance)
    Info = 0,
    /// Warning level (potential danger)
    Warning = 1,
    /// Critical emergency (immediate danger)
    Critical = 2,
    /// Life-threatening (medical, fire, active threat)
    LifeThreatening = 3,
}

/// An SOS alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SosAlert {
    pub id: String,
    pub sender: NodeId,
    pub severity: SosSeverity,
    pub message: String,
    pub location: Option<(f64, f64)>, // (x, y) or (lat, lon)
    pub alert_type: SosAlertType,
    pub timestamp: u64,
    pub acknowledged_by: Vec<NodeId>,
    pub resolved: bool,
}

/// Types of SOS alerts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SosAlertType {
    Medical,
    Fire,
    Security,
    NaturalDisaster,
    FallDetected,
    ManualTrigger,
    Custom(String),
}

/// SOS broadcast manager.
pub struct SosManager {
    alerts: Arc<RwLock<HashMap<String, SosAlert>>>,
    /// Maximum active alerts before oldest are pruned
    max_active: usize,
}

impl SosManager {
    pub fn new(max_active: usize) -> Self {
        Self {
            alerts: Arc::new(RwLock::new(HashMap::new())),
            max_active,
        }
    }

    /// Broadcast an SOS alert. Returns the alert ID.
    pub async fn broadcast(
        &self,
        sender: NodeId,
        severity: SosSeverity,
        message: String,
        alert_type: SosAlertType,
        location: Option<(f64, f64)>,
    ) -> WiosResult<String> {
        let alert = SosAlert {
            id: uuid::Uuid::new_v4().to_string(),
            sender,
            severity,
            message,
            location,
            alert_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            acknowledged_by: Vec::new(),
            resolved: false,
        };
        let id = alert.id.clone();
        let mut alerts = self.alerts.write().await;

        // Prune oldest if at capacity
        if alerts.len() >= self.max_active {
            if let Some(oldest_id) = alerts.values()
                .filter(|a| a.resolved)
                .min_by_key(|a| a.timestamp)
                .map(|a| a.id.clone())
            {
                alerts.remove(&oldest_id);
            }
        }

        alerts.insert(id.clone(), alert);
        Ok(id)
    }

    /// Acknowledge an alert from a peer.
    pub async fn acknowledge(&self, alert_id: &str, node: NodeId) -> WiosResult<()> {
        let mut alerts = self.alerts.write().await;
        let alert = alerts.get_mut(alert_id)
            .ok_or(WiosError::NotFound { entity: "alert".into(), id: alert_id.into() })?;
        if !alert.acknowledged_by.contains(&node) {
            alert.acknowledged_by.push(node);
        }
        Ok(())
    }

    /// Resolve an alert.
    pub async fn resolve(&self, alert_id: &str) -> WiosResult<()> {
        let mut alerts = self.alerts.write().await;
        let alert = alerts.get_mut(alert_id)
            .ok_or(WiosError::NotFound { entity: "alert".into(), id: alert_id.into() })?;
        alert.resolved = true;
        Ok(())
    }

    /// Get all active (unresolved) alerts, sorted by severity (highest first).
    pub async fn active_alerts(&self) -> Vec<SosAlert> {
        let alerts = self.alerts.read().await;
        let mut active: Vec<_> = alerts.values()
            .filter(|a| !a.resolved)
            .cloned()
            .collect();
        active.sort_by(|a, b| b.severity.cmp(&a.severity));
        active
    }

    /// Get alert count by severity.
    pub async fn count_by_severity(&self) -> HashMap<SosSeverity, usize> {
        let alerts = self.alerts.read().await;
        let mut counts = HashMap::new();
        for alert in alerts.values().filter(|a| !a.resolved) {
            *counts.entry(alert.severity).or_insert(0) += 1;
        }
        counts
    }
}

impl Default for SosManager {
    fn default() -> Self { Self::new(100) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sos_broadcast_and_acknowledge() {
        let mgr = SosManager::new(10);
        let sender = NodeId::new();
        let responder = NodeId::new();

        let id = mgr.broadcast(
            sender, SosSeverity::Critical, "Fire in building A".into(),
            SosAlertType::Fire, Some((10.0, 20.0)),
        ).await.unwrap();

        let active = mgr.active_alerts().await;
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].severity, SosSeverity::Critical);

        mgr.acknowledge(&id, responder).await.unwrap();
        let alert = &mgr.alerts.read().await[&id];
        assert_eq!(alert.acknowledged_by.len(), 1);
    }

    #[tokio::test]
    async fn test_sos_resolve() {
        let mgr = SosManager::new(10);
        let id = mgr.broadcast(
            NodeId::new(), SosSeverity::Warning, "Test".into(),
            SosAlertType::ManualTrigger, None,
        ).await.unwrap();

        assert_eq!(mgr.active_alerts().await.len(), 1);
        mgr.resolve(&id).await.unwrap();
        assert_eq!(mgr.active_alerts().await.len(), 0);
    }

    #[tokio::test]
    async fn test_severity_ordering() {
        let mgr = SosManager::new(10);
        mgr.broadcast(NodeId::new(), SosSeverity::Info, "Low".into(), SosAlertType::Custom("test".into()), None).await.unwrap();
        mgr.broadcast(NodeId::new(), SosSeverity::LifeThreatening, "High".into(), SosAlertType::Medical, None).await.unwrap();
        mgr.broadcast(NodeId::new(), SosSeverity::Warning, "Mid".into(), SosAlertType::Security, None).await.unwrap();

        let active = mgr.active_alerts().await;
        assert_eq!(active[0].severity, SosSeverity::LifeThreatening);
        assert_eq!(active[1].severity, SosSeverity::Warning);
        assert_eq!(active[2].severity, SosSeverity::Info);
    }
}
