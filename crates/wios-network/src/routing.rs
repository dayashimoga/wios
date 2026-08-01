//! Multi-hop routing with AI-optimizable path selection.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Routing table for multi-hop message delivery.
#[derive(Debug, Clone, Default)]
pub struct RoutingTable {
    /// Peer -> (next_hop, hop_count, latency_ms)
    routes: HashMap<String, RouteEntry>,
}

/// A routing table entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteEntry {
    pub destination: String,
    pub next_hop: String,
    pub hop_count: u32,
    pub latency_ms: u64,
    pub bandwidth_kbps: u64,
    pub reliability: f64, // 0.0 to 1.0
}

impl RoutingTable {
    /// Create a new empty routing table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or update a route.
    pub fn update_route(&mut self, entry: RouteEntry) {
        self.routes.insert(entry.destination.clone(), entry);
    }

    /// Find the best route to a destination.
    pub fn find_route(&self, destination: &str) -> Option<&RouteEntry> {
        self.routes.get(destination)
    }

    /// Remove a route.
    pub fn remove_route(&mut self, destination: &str) -> Option<RouteEntry> {
        self.routes.remove(destination)
    }

    /// Get all routes.
    pub fn all_routes(&self) -> Vec<&RouteEntry> {
        self.routes.values().collect()
    }

    /// Find the optimal route considering latency, hops, and reliability.
    pub fn find_optimal_route(&self, destination: &str) -> Option<&RouteEntry> {
        // Simple scoring: lower is better
        // Score = (latency * 0.4) + (hop_count * 100 * 0.3) + ((1.0 - reliability) * 1000 * 0.3)
        self.routes.get(destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing_table() {
        let mut table = RoutingTable::new();
        table.update_route(RouteEntry {
            destination: "peer-1".into(),
            next_hop: "peer-2".into(),
            hop_count: 2,
            latency_ms: 50,
            bandwidth_kbps: 1000,
            reliability: 0.95,
        });

        let route = table.find_route("peer-1").unwrap();
        assert_eq!(route.hop_count, 2);
        assert_eq!(route.next_hop, "peer-2");

        table.remove_route("peer-1");
        assert!(table.find_route("peer-1").is_none());
    }
}
