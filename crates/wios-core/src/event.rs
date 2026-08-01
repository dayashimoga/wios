//! Event system for inter-component communication.
//!
//! Provides a typed, async pub/sub event bus that all WIOS subsystems
//! use to communicate state changes without tight coupling.

use crate::types::{ConnectionState, DeviceId, MeshMessage, NodeId, NodeInfo, PeerId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

/// All events that can occur in the WIOS system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WiosEvent {
    // ── Network Events ─────────────────────────────────────────
    /// A new peer was discovered.
    PeerDiscovered { peer_id: PeerId, info: NodeInfo },
    /// A peer went offline.
    PeerLost { peer_id: PeerId },
    /// Connection state changed.
    ConnectionChanged {
        peer_id: PeerId,
        state: ConnectionState,
    },
    /// A message was received from the mesh.
    MessageReceived { message: MeshMessage },
    /// A message was sent successfully.
    MessageSent { message_id: String },
    /// Message delivery failed.
    MessageFailed { message_id: String, reason: String },
    /// Network topology changed.
    TopologyChanged { peer_count: usize },

    // ── Storage Events ─────────────────────────────────────────
    /// Data was synced with a peer.
    DataSynced {
        peer_id: PeerId,
        records_synced: u64,
    },
    /// Sync conflict detected.
    SyncConflict { entity: String, id: String },
    /// Storage quota warning.
    StorageQuotaWarning { used_bytes: u64, max_bytes: u64 },

    // ── Security Events ────────────────────────────────────────
    /// Authentication succeeded.
    AuthSuccess { node_id: NodeId },
    /// Authentication failed.
    AuthFailure { node_id: NodeId, reason: String },
    /// Security audit event.
    AuditLog {
        action: String,
        actor: NodeId,
        target: Option<String>,
        timestamp: DateTime<Utc>,
    },

    // ── AI Events ──────────────────────────────────────────────
    /// Model loaded successfully.
    ModelLoaded { model_id: String },
    /// Inference completed.
    InferenceComplete { model_id: String, duration_ms: u64 },

    // ── Compute Events ─────────────────────────────────────────
    /// Distributed task submitted.
    TaskSubmitted { task_id: String },
    /// Task completed.
    TaskCompleted { task_id: String, duration_ms: u64 },
    /// Task failed.
    TaskFailed { task_id: String, reason: String },

    // ── Device Events ──────────────────────────────────────────
    /// Device capability shared.
    CapabilityShared {
        device_id: DeviceId,
        capability: String,
    },
    /// Device capability revoked.
    CapabilityRevoked {
        device_id: DeviceId,
        capability: String,
    },

    // ── System Events ──────────────────────────────────────────
    /// System started.
    SystemStarted,
    /// System shutting down.
    SystemShutdown,
    /// Configuration changed.
    ConfigChanged { key: String },
    /// Error occurred in a subsystem.
    SubsystemError { subsystem: String, error: String },
}

/// Event bus for publishing and subscribing to WIOS events.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Arc<WiosEvent>>,
}

impl EventBus {
    /// Create a new event bus with the specified channel capacity.
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Publish an event to all subscribers.
    pub fn publish(&self, event: WiosEvent) -> usize {
        // Returns the number of receivers that got the event.
        // Ignores errors (no subscribers) gracefully.
        self.sender.send(Arc::new(event)).unwrap_or(0)
    }

    /// Subscribe to events.
    pub fn subscribe(&self) -> broadcast::Receiver<Arc<WiosEvent>> {
        self.sender.subscribe()
    }

    /// Get the current number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_bus_pub_sub() {
        let bus = EventBus::new(16);
        let mut rx = bus.subscribe();

        bus.publish(WiosEvent::SystemStarted);

        let event = rx.recv().await.unwrap();
        assert!(matches!(*event, WiosEvent::SystemStarted));
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let bus = EventBus::new(16);
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();

        assert_eq!(bus.subscriber_count(), 2);
        bus.publish(WiosEvent::SystemStarted);

        assert!(matches!(
            *rx1.recv().await.unwrap(),
            WiosEvent::SystemStarted
        ));
        assert!(matches!(
            *rx2.recv().await.unwrap(),
            WiosEvent::SystemStarted
        ));
    }

    #[test]
    fn test_no_subscribers() {
        let bus = EventBus::new(16);
        let count = bus.publish(WiosEvent::SystemStarted);
        assert_eq!(count, 0);
    }
}
