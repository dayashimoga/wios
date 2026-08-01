//! # WIOS Network
//!
//! AI-managed P2P mesh networking engine built on libp2p.
//!
//! Provides:
//! - TCP/QUIC transports with Noise encryption
//! - mDNS peer discovery for LAN
//! - Kademlia DHT for distributed routing
//! - Gossipsub for pub/sub messaging
//! - Multi-hop routing with AI optimization
//! - Offline message queuing (store-and-forward)
//! - Emergency SOS broadcast system
//! - Resumable encrypted file transfer with compression
//! - WebRTC signaling for voice/video
//! - Sensor fusion and indoor positioning
//! - Floor plan navigation with Dijkstra pathfinding
//! - Asset tracking with geofencing
//! - Automation rule engine with sensor triggers
//! - Pluggable transport providers (WiFi, BLE, QUIC, LoRa, UWB)

pub mod asset_tracker;
pub mod automation;
pub mod compression;
pub mod delta_sync;
pub mod discovery;
pub mod discovery_service;
pub mod emergency;
pub mod file_transfer;
pub mod floor_plan;
pub mod gossip;
pub mod mesh;
pub mod offline_queue;
pub mod routing;
pub mod sensing;
pub mod sensor_fusion;
pub mod signaling;
pub mod transport;
pub mod transport_provider;

pub use asset_tracker::AssetTracker;
pub use automation::RuleEngine;
pub use discovery_service::DiscoveryService;
pub use emergency::SosManager;
pub use file_transfer::TransferManager;
pub use gossip::MessageBroker;
pub use mesh::MeshNode;
pub use offline_queue::OfflineQueue;
pub use sensor_fusion::FusedPosition;
pub use signaling::SignalingServer;
pub use transport_provider::{TransportProvider, TransportRegistry};
