//! mDNS + Kademlia discovery wiring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use wios_core::types::NodeId;

/// A discovered peer on the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    pub node_id: NodeId,
    pub addresses: Vec<String>,
    pub discovery_method: DiscoveryMethod,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// How a peer was discovered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Mdns,
    Kademlia,
    Manual,
    Bootstrap,
}

/// Peer discovery service combining mDNS (LAN) and Kademlia (WAN).
pub struct DiscoveryService {
    peers: Arc<RwLock<HashMap<String, DiscoveredPeer>>>,
    mdns_enabled: bool,
    kademlia_enabled: bool,
}

impl DiscoveryService {
    pub fn new(mdns: bool, kademlia: bool) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            mdns_enabled: mdns,
            kademlia_enabled: kademlia,
        }
    }

    /// Record a discovered peer.
    pub async fn on_peer_discovered(
        &self,
        node_id: &NodeId,
        addresses: Vec<String>,
        method: DiscoveryMethod,
        metadata: HashMap<String, String>,
    ) {
        let mut peers = self.peers.write().await;
        let key = node_id.as_str().to_string();
        let now = Utc::now();

        if let Some(existing) = peers.get_mut(&key) {
            existing.last_seen = now;
            // Merge new addresses
            for addr in &addresses {
                if !existing.addresses.contains(addr) {
                    existing.addresses.push(addr.clone());
                }
            }
        } else {
            peers.insert(key, DiscoveredPeer {
                node_id: node_id.clone(),
                addresses,
                discovery_method: method,
                first_seen: now,
                last_seen: now,
                metadata,
            });
        }
    }

    /// Remove a peer (e.g., mDNS expiry).
    pub async fn on_peer_expired(&self, node_id: &NodeId) {
        self.peers.write().await.remove(node_id.as_str());
    }

    /// Get all discovered peers.
    pub async fn discovered_peers(&self) -> Vec<DiscoveredPeer> {
        self.peers.read().await.values().cloned().collect()
    }

    /// Find a peer by node ID.
    pub async fn find_peer(&self, node_id: &NodeId) -> Option<DiscoveredPeer> {
        self.peers.read().await.get(node_id.as_str()).cloned()
    }

    /// Get peers discovered via a specific method.
    pub async fn peers_by_method(&self, method: DiscoveryMethod) -> Vec<DiscoveredPeer> {
        self.peers.read().await.values()
            .filter(|p| p.discovery_method == method)
            .cloned()
            .collect()
    }

    /// Purge peers not seen within `timeout_secs`.
    pub async fn purge_stale(&self, timeout_secs: i64) -> usize {
        let mut peers = self.peers.write().await;
        let now = Utc::now();
        let stale: Vec<String> = peers.iter()
            .filter(|(_, p)| (now - p.last_seen).num_seconds() > timeout_secs)
            .map(|(k, _)| k.clone())
            .collect();
        let count = stale.len();
        for key in stale {
            peers.remove(&key);
        }
        count
    }

    pub fn mdns_enabled(&self) -> bool { self.mdns_enabled }
    pub fn kademlia_enabled(&self) -> bool { self.kademlia_enabled }

    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_lifecycle() {
        let svc = DiscoveryService::new(true, true);
        let node = NodeId::new();

        svc.on_peer_discovered(
            &node,
            vec!["192.168.1.10:9090".into()],
            DiscoveryMethod::Mdns,
            HashMap::new(),
        ).await;

        assert_eq!(svc.peer_count().await, 1);
        assert!(svc.find_peer(&node).await.is_some());

        svc.on_peer_expired(&node).await;
        assert_eq!(svc.peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_address_merge() {
        let svc = DiscoveryService::new(true, false);
        let node = NodeId::new();

        svc.on_peer_discovered(&node, vec!["addr1".into()], DiscoveryMethod::Mdns, HashMap::new()).await;
        svc.on_peer_discovered(&node, vec!["addr2".into()], DiscoveryMethod::Mdns, HashMap::new()).await;

        let peer = svc.find_peer(&node).await.unwrap();
        assert_eq!(peer.addresses.len(), 2);
    }

    #[tokio::test]
    async fn test_peers_by_method() {
        let svc = DiscoveryService::new(true, true);
        let n1 = NodeId::new();
        let n2 = NodeId::new();

        svc.on_peer_discovered(&n1, vec![], DiscoveryMethod::Mdns, HashMap::new()).await;
        svc.on_peer_discovered(&n2, vec![], DiscoveryMethod::Kademlia, HashMap::new()).await;

        assert_eq!(svc.peers_by_method(DiscoveryMethod::Mdns).await.len(), 1);
        assert_eq!(svc.peers_by_method(DiscoveryMethod::Kademlia).await.len(), 1);
    }
}
