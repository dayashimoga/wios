//! Store-and-forward offline message queue.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Priority of a queued message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// A queued offline message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedMessage {
    pub id: String,
    pub destination: String,
    pub payload: Vec<u8>,
    pub priority: MessagePriority,
    pub created_at: DateTime<Utc>,
    pub ttl_secs: u64,
    pub retry_count: u32,
    pub max_retries: u32,
}

impl QueuedMessage {
    /// Check if the message has expired.
    pub fn is_expired(&self) -> bool {
        let elapsed = (Utc::now() - self.created_at).num_seconds();
        elapsed > self.ttl_secs as i64
    }

    /// Check if retries are exhausted.
    pub fn retries_exhausted(&self) -> bool {
        self.retry_count >= self.max_retries
    }
}

/// Offline message queue with priority ordering and TTL expiry.
pub struct OfflineQueue {
    queue: Arc<Mutex<VecDeque<QueuedMessage>>>,
    max_size: usize,
}

impl OfflineQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
        }
    }

    /// Enqueue a message for later delivery.
    pub async fn enqueue(&self, msg: QueuedMessage) -> Result<(), String> {
        let mut queue = self.queue.lock().await;
        if queue.len() >= self.max_size {
            // Drop lowest priority expired messages first
            queue.retain(|m| !m.is_expired());
            if queue.len() >= self.max_size {
                return Err("Offline queue is full".into());
            }
        }
        queue.push_back(msg);
        // Sort by priority (highest first)
        queue.make_contiguous().sort_by_key(|b| std::cmp::Reverse(b.priority));
        Ok(())
    }

    /// Dequeue messages for a destination (when peer comes online).
    pub async fn drain_for(&self, destination: &str) -> Vec<QueuedMessage> {
        let mut queue = self.queue.lock().await;
        let mut result = Vec::new();
        queue.retain(|msg| {
            if msg.destination == destination && !msg.is_expired() {
                result.push(msg.clone());
                false
            } else {
                true
            }
        });
        result
    }

    /// Record a failed delivery attempt.
    pub async fn record_failure(&self, msg_id: &str) {
        let mut queue = self.queue.lock().await;
        if let Some(msg) = queue.iter_mut().find(|m| m.id == msg_id) {
            msg.retry_count += 1;
        }
    }

    /// Re-enqueue a message after failed delivery.
    pub async fn requeue(&self, mut msg: QueuedMessage) -> Result<(), String> {
        msg.retry_count += 1;
        if msg.retries_exhausted() {
            return Err("Max retries exceeded".into());
        }
        self.enqueue(msg).await
    }

    /// Purge expired and exhausted messages.
    pub async fn purge(&self) -> usize {
        let mut queue = self.queue.lock().await;
        let before = queue.len();
        queue.retain(|m| !m.is_expired() && !m.retries_exhausted());
        before - queue.len()
    }

    /// Get queue depth.
    pub async fn len(&self) -> usize {
        self.queue.lock().await.len()
    }

    /// Check if queue is empty.
    pub async fn is_empty(&self) -> bool {
        self.queue.lock().await.is_empty()
    }

    /// Get queue depth for a specific destination.
    pub async fn pending_for(&self, destination: &str) -> usize {
        self.queue.lock().await.iter().filter(|m| m.destination == destination && !m.is_expired()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_msg(dest: &str, priority: MessagePriority) -> QueuedMessage {
        QueuedMessage {
            id: uuid::Uuid::new_v4().to_string(),
            destination: dest.into(),
            payload: b"hello".to_vec(),
            priority,
            created_at: Utc::now(),
            ttl_secs: 3600,
            retry_count: 0,
            max_retries: 3,
        }
    }

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let q = OfflineQueue::new(100);
        q.enqueue(make_msg("peer1", MessagePriority::Normal)).await.unwrap();
        q.enqueue(make_msg("peer2", MessagePriority::High)).await.unwrap();
        assert_eq!(q.len().await, 2);

        let msgs = q.drain_for("peer1").await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(q.len().await, 1);
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        let q = OfflineQueue::new(100);
        q.enqueue(make_msg("p", MessagePriority::Low)).await.unwrap();
        q.enqueue(make_msg("p", MessagePriority::Critical)).await.unwrap();
        q.enqueue(make_msg("p", MessagePriority::Normal)).await.unwrap();

        let msgs = q.drain_for("p").await;
        assert_eq!(msgs[0].priority, MessagePriority::Critical);
        assert_eq!(msgs[2].priority, MessagePriority::Low);
    }

    #[tokio::test]
    async fn test_queue_full() {
        let q = OfflineQueue::new(1);
        q.enqueue(make_msg("p", MessagePriority::Normal)).await.unwrap();
        assert!(q.enqueue(make_msg("p", MessagePriority::Normal)).await.is_err());
    }

    #[tokio::test]
    async fn test_requeue_max_retries() {
        let q = OfflineQueue::new(100);
        let mut msg = make_msg("p", MessagePriority::Normal);
        msg.retry_count = 3;
        msg.max_retries = 3;
        assert!(q.requeue(msg).await.is_err());
    }
}
