//! Core mesh node implementation.

use libp2p::{gossipsub, identify, kad, mdns, ping, swarm::NetworkBehaviour, PeerId};
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::info;
use wios_core::config::NetworkConfig;
use wios_core::error::WiosResult;

/// Combined network behaviour for the mesh node.
#[derive(NetworkBehaviour)]
pub struct MeshBehaviour {
    /// Gossipsub for pub/sub messaging.
    pub gossipsub: gossipsub::Behaviour,
    /// Kademlia for distributed routing.
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    /// mDNS for local peer discovery.
    pub mdns: mdns::tokio::Behaviour,
    /// Identify for peer information exchange.
    pub identify: identify::Behaviour,
    /// Ping for connection liveness.
    pub ping: ping::Behaviour,
}

/// Represents a peer in the mesh network.
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub addresses: Vec<libp2p::Multiaddr>,
    pub latency_ms: Option<u64>,
    pub connected_since: Option<chrono::DateTime<chrono::Utc>>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// The core mesh networking node.
pub struct MeshNode {
    local_peer_id: PeerId,
    peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,
    config: NetworkConfig,
}

impl MeshNode {
    /// Create a new mesh node with the given configuration.
    pub fn new(config: NetworkConfig) -> WiosResult<Self> {
        // Generate a new Ed25519 keypair for libp2p identity
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        info!("Mesh node created with PeerId: {}", local_peer_id);

        Ok(Self {
            local_peer_id,
            peers: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Get the local peer ID.
    pub fn peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    /// Get the current peer count.
    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.len()
    }

    /// Get all connected peers.
    pub async fn connected_peers(&self) -> Vec<PeerInfo> {
        self.peers.read().await.values().cloned().collect()
    }

    /// Add a discovered peer.
    pub async fn add_peer(&self, peer_id: PeerId, addresses: Vec<libp2p::Multiaddr>) {
        let mut peers = self.peers.write().await;
        peers.insert(
            peer_id,
            PeerInfo {
                peer_id,
                addresses,
                latency_ms: None,
                connected_since: Some(chrono::Utc::now()),
                last_seen: chrono::Utc::now(),
            },
        );
        info!("Peer added: {} (total: {})", peer_id, peers.len());
    }

    /// Remove a peer.
    pub async fn remove_peer(&self, peer_id: &PeerId) {
        let mut peers = self.peers.write().await;
        if peers.remove(peer_id).is_some() {
            info!("Peer removed: {} (total: {})", peer_id, peers.len());
        }
    }

    /// Update peer latency.
    pub async fn update_latency(&self, peer_id: &PeerId, latency_ms: u64) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.latency_ms = Some(latency_ms);
            peer.last_seen = chrono::Utc::now();
        }
    }

    /// Get network statistics.
    pub async fn stats(&self) -> NetworkStats {
        let peers = self.peers.read().await;
        let avg_latency = if peers.is_empty() {
            0.0
        } else {
            let sum: u64 = peers.values().filter_map(|p| p.latency_ms).sum();
            let count = peers.values().filter(|p| p.latency_ms.is_some()).count();
            if count > 0 {
                sum as f64 / count as f64
            } else {
                0.0
            }
        };

        NetworkStats {
            local_peer_id: self.local_peer_id.to_string(),
            connected_peers: peers.len(),
            avg_latency_ms: avg_latency,
            config: self.config.clone(),
        }
    }
}

/// Network statistics snapshot.
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub local_peer_id: String,
    pub connected_peers: usize,
    pub avg_latency_ms: f64,
    pub config: NetworkConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_node_creation() {
        let config = NetworkConfig {
            listen_addresses: vec!["/ip4/0.0.0.0/tcp/0".into()],
            bootstrap_peers: vec![],
            mdns_enabled: true,
            dht_enabled: true,
            gossipsub_enabled: true,
            max_peers: 64,
            connection_timeout_secs: 30,
            relay_enabled: false,
        };
        let node = MeshNode::new(config).unwrap();
        assert!(!node.peer_id().to_string().is_empty());
    }

    #[tokio::test]
    async fn test_peer_management() {
        let config = NetworkConfig {
            listen_addresses: vec![],
            bootstrap_peers: vec![],
            mdns_enabled: false,
            dht_enabled: false,
            gossipsub_enabled: false,
            max_peers: 10,
            connection_timeout_secs: 30,
            relay_enabled: false,
        };
        let node = MeshNode::new(config).unwrap();

        let peer_id = PeerId::random();
        node.add_peer(peer_id, vec![]).await;
        assert_eq!(node.peer_count().await, 1);

        node.update_latency(&peer_id, 42).await;
        let peers = node.connected_peers().await;
        assert_eq!(peers[0].latency_ms, Some(42));

        node.remove_peer(&peer_id).await;
        assert_eq!(node.peer_count().await, 0);
    }
}
