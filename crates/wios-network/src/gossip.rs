//! Gossipsub messaging API wrapper.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// A gossipsub topic.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub name: String,
}

impl Topic {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// A published message on the gossipsub network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipMessage {
    pub id: String,
    pub topic: String,
    pub source: String,
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

/// Local message broker for gossipsub topics (wraps libp2p gossipsub).
pub struct MessageBroker {
    subscriptions: Arc<RwLock<HashSet<String>>>,
    inbox: Arc<RwLock<HashMap<String, Vec<GossipMessage>>>>,
    published_count: Arc<RwLock<u64>>,
}

impl MessageBroker {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(RwLock::new(HashSet::new())),
            inbox: Arc::new(RwLock::new(HashMap::new())),
            published_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Subscribe to a topic.
    pub async fn subscribe(&self, topic: &str) {
        self.subscriptions.write().await.insert(topic.to_string());
        self.inbox
            .write()
            .await
            .entry(topic.to_string())
            .or_default();
    }

    /// Unsubscribe from a topic.
    pub async fn unsubscribe(&self, topic: &str) {
        self.subscriptions.write().await.remove(topic);
    }

    /// Check if subscribed to a topic.
    pub async fn is_subscribed(&self, topic: &str) -> bool {
        self.subscriptions.read().await.contains(topic)
    }

    /// Publish a message to a topic.
    pub async fn publish(&self, topic: &str, source: &str, data: Vec<u8>) -> String {
        let msg = GossipMessage {
            id: uuid::Uuid::new_v4().to_string(),
            topic: topic.to_string(),
            source: source.to_string(),
            data,
            timestamp: Utc::now(),
        };
        let id = msg.id.clone();

        // Deliver to local inbox if subscribed
        let mut inbox = self.inbox.write().await;
        if let Some(msgs) = inbox.get_mut(topic) {
            msgs.push(msg);
        }
        *self.published_count.write().await += 1;
        id
    }

    /// Receive messages from a topic's inbox (drains).
    pub async fn receive(&self, topic: &str) -> Vec<GossipMessage> {
        let mut inbox = self.inbox.write().await;
        inbox.remove(topic).unwrap_or_default()
    }

    /// Deliver an incoming message from the network.
    pub async fn deliver(&self, msg: GossipMessage) {
        let mut inbox = self.inbox.write().await;
        if let Some(msgs) = inbox.get_mut(&msg.topic) {
            msgs.push(msg);
        }
    }

    /// List subscribed topics.
    pub async fn subscriptions(&self) -> Vec<String> {
        self.subscriptions.read().await.iter().cloned().collect()
    }

    /// Get published message count.
    pub async fn published_count(&self) -> u64 {
        *self.published_count.read().await
    }
}

impl Default for MessageBroker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pub_sub() {
        let broker = MessageBroker::new();
        broker.subscribe("chat").await;
        assert!(broker.is_subscribed("chat").await);

        broker.publish("chat", "node-1", b"hello".to_vec()).await;
        let msgs = broker.receive("chat").await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].data, b"hello");
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let broker = MessageBroker::new();
        broker.subscribe("topic1").await;
        broker.unsubscribe("topic1").await;
        assert!(!broker.is_subscribed("topic1").await);
    }

    #[tokio::test]
    async fn test_multiple_topics() {
        let broker = MessageBroker::new();
        broker.subscribe("a").await;
        broker.subscribe("b").await;

        broker.publish("a", "n1", b"msg-a".to_vec()).await;
        broker.publish("b", "n1", b"msg-b".to_vec()).await;

        assert_eq!(broker.receive("a").await.len(), 1);
        assert_eq!(broker.receive("b").await.len(), 1);
        assert_eq!(broker.published_count().await, 2);
    }
}
