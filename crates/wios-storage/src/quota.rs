//! Storage quota management with alerts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;
use wios_core::error::{WiosError, WiosResult};

/// Quota alert levels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuotaAlert {
    /// Usage normal (< 75%).
    Normal,
    /// Warning threshold (75-90%).
    Warning,
    /// Critical threshold (90-99%).
    Critical,
    /// Quota exceeded (100%).
    Exceeded,
}

/// Quota usage snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaUsage {
    pub used_bytes: u64,
    pub max_bytes: u64,
    pub percentage: f64,
    pub alert: QuotaAlert,
    pub last_checked: DateTime<Utc>,
}

/// Storage quota manager.
pub struct QuotaManager {
    state: Arc<RwLock<QuotaState>>,
}

struct QuotaState {
    used_bytes: u64,
    max_bytes: u64,
    warning_threshold: f64,
    critical_threshold: f64,
}

impl QuotaManager {
    /// Create a new quota manager.
    /// `max_bytes` of 0 means unlimited.
    pub fn new(max_bytes: u64) -> Self {
        Self {
            state: Arc::new(RwLock::new(QuotaState {
                used_bytes: 0,
                max_bytes,
                warning_threshold: 0.75,
                critical_threshold: 0.90,
            })),
        }
    }

    /// Check if there's enough space for `bytes` additional data.
    pub async fn check_space(&self, bytes: u64) -> WiosResult<()> {
        let state = self.state.read().await;
        if state.max_bytes == 0 {
            return Ok(()); // unlimited
        }
        if state.used_bytes + bytes > state.max_bytes {
            return Err(WiosError::Storage(format!(
                "Storage quota exceeded: {} + {} > {} bytes",
                state.used_bytes, bytes, state.max_bytes
            )));
        }
        Ok(())
    }

    /// Record bytes written.
    pub async fn record_write(&self, bytes: u64) {
        let mut state = self.state.write().await;
        state.used_bytes = state.used_bytes.saturating_add(bytes);
        self.check_alerts(&state);
    }

    /// Record bytes freed.
    pub async fn record_delete(&self, bytes: u64) {
        let mut state = self.state.write().await;
        state.used_bytes = state.used_bytes.saturating_sub(bytes);
    }

    /// Set used bytes directly (e.g., after a scan).
    pub async fn set_used(&self, bytes: u64) {
        self.state.write().await.used_bytes = bytes;
    }

    /// Set max quota.
    pub async fn set_max(&self, bytes: u64) {
        self.state.write().await.max_bytes = bytes;
    }

    /// Get current usage.
    pub async fn usage(&self) -> QuotaUsage {
        let state = self.state.read().await;
        let percentage = if state.max_bytes == 0 {
            0.0
        } else {
            (state.used_bytes as f64 / state.max_bytes as f64) * 100.0
        };

        let alert = if state.max_bytes == 0 {
            QuotaAlert::Normal
        } else if state.used_bytes >= state.max_bytes {
            QuotaAlert::Exceeded
        } else if percentage / 100.0 >= state.critical_threshold {
            QuotaAlert::Critical
        } else if percentage / 100.0 >= state.warning_threshold {
            QuotaAlert::Warning
        } else {
            QuotaAlert::Normal
        };

        QuotaUsage {
            used_bytes: state.used_bytes,
            max_bytes: state.max_bytes,
            percentage,
            alert,
            last_checked: Utc::now(),
        }
    }

    fn check_alerts(&self, state: &QuotaState) {
        if state.max_bytes == 0 {
            return;
        }
        let pct = state.used_bytes as f64 / state.max_bytes as f64;
        if pct >= 1.0 {
            warn!("Storage quota EXCEEDED: {}/{} bytes", state.used_bytes, state.max_bytes);
        } else if pct >= state.critical_threshold {
            warn!("Storage quota CRITICAL: {:.1}% used", pct * 100.0);
        } else if pct >= state.warning_threshold {
            warn!("Storage quota WARNING: {:.1}% used", pct * 100.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quota_lifecycle() {
        let mgr = QuotaManager::new(1000);
        mgr.record_write(500).await;
        let usage = mgr.usage().await;
        assert_eq!(usage.used_bytes, 500);
        assert_eq!(usage.alert, QuotaAlert::Normal);
    }

    #[tokio::test]
    async fn test_quota_warning() {
        let mgr = QuotaManager::new(1000);
        mgr.record_write(800).await;
        assert_eq!(mgr.usage().await.alert, QuotaAlert::Warning);
    }

    #[tokio::test]
    async fn test_quota_critical() {
        let mgr = QuotaManager::new(1000);
        mgr.record_write(950).await;
        assert_eq!(mgr.usage().await.alert, QuotaAlert::Critical);
    }

    #[tokio::test]
    async fn test_quota_exceeded() {
        let mgr = QuotaManager::new(1000);
        mgr.record_write(1000).await;
        assert_eq!(mgr.usage().await.alert, QuotaAlert::Exceeded);
        assert!(mgr.check_space(1).await.is_err());
    }

    #[tokio::test]
    async fn test_unlimited_quota() {
        let mgr = QuotaManager::new(0);
        mgr.record_write(999999).await;
        assert_eq!(mgr.usage().await.alert, QuotaAlert::Normal);
        assert!(mgr.check_space(999999).await.is_ok());
    }
}
