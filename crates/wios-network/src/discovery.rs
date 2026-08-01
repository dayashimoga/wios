//! Peer discovery mechanisms.

/// Discovery configuration for various methods.
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Enable mDNS for LAN discovery.
    pub mdns_enabled: bool,
    /// Enable DHT for internet-scale discovery.
    pub dht_enabled: bool,
    /// Bootstrap peer addresses.
    pub bootstrap_peers: Vec<String>,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            mdns_enabled: true,
            dht_enabled: true,
            bootstrap_peers: vec![],
        }
    }
}
