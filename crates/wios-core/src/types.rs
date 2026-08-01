//! Core domain types for WIOS.
//!
//! These types are shared across all subsystems and provide the
//! fundamental identifiers and structures used throughout the platform.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a device in the mesh network.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    /// Generate a new random device ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create from an existing string.
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Get the underlying string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for DeviceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a peer in the P2P network.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId(pub String);

impl PeerId {
    /// Create from an existing string (typically from libp2p PeerId).
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a node (combines device and peer identity).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// UTC timestamp wrapper for consistency.
pub type Timestamp = DateTime<Utc>;

/// Comprehensive information about a node in the mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique node identifier.
    pub node_id: NodeId,
    /// Device identifier.
    pub device_id: DeviceId,
    /// Peer identifier (assigned by libp2p).
    pub peer_id: Option<PeerId>,
    /// Human-readable node name.
    pub name: String,
    /// Platform (android, ios, windows, linux, macos, web).
    pub platform: Platform,
    /// Device capabilities.
    pub capabilities: DeviceCapabilities,
    /// Node version string.
    pub version: String,
    /// When this node was first seen.
    pub first_seen: Timestamp,
    /// When this node was last seen.
    pub last_seen: Timestamp,
    /// Whether this node is currently online.
    pub is_online: bool,
    /// Signal strength (RSSI) if available.
    pub signal_strength: Option<i32>,
    /// Latency to this node in milliseconds.
    pub latency_ms: Option<u64>,
}

impl NodeInfo {
    /// Create a new NodeInfo for the local device.
    pub fn new_local(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            node_id: NodeId::new(),
            device_id: DeviceId::new(),
            peer_id: None,
            name: name.into(),
            platform: Platform::current(),
            capabilities: DeviceCapabilities::default(),
            version: env!("CARGO_PKG_VERSION").into(),
            first_seen: now,
            last_seen: now,
            is_online: true,
            signal_strength: None,
            latency_ms: None,
        }
    }
}

/// Device capabilities that can be shared across the mesh.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// Available CPU cores.
    pub cpu_cores: u32,
    /// Available RAM in MB.
    pub ram_mb: u64,
    /// Available storage in MB.
    pub storage_mb: u64,
    /// Has GPU acceleration.
    pub has_gpu: bool,
    /// Has camera.
    pub has_camera: bool,
    /// Has microphone.
    pub has_microphone: bool,
    /// Has display.
    pub has_display: bool,
    /// Has keyboard.
    pub has_keyboard: bool,
    /// Has mouse/touchpad.
    pub has_pointer: bool,
    /// Has printer access.
    pub has_printer: bool,
    /// Has accelerometer.
    pub has_accelerometer: bool,
    /// Has gyroscope.
    pub has_gyroscope: bool,
    /// Has GPS.
    pub has_gps: bool,
    /// Has Bluetooth.
    pub has_bluetooth: bool,
    /// Has WiFi Direct.
    pub has_wifi_direct: bool,
    /// Has NFC.
    pub has_nfc: bool,
    /// Battery level (0-100, None if plugged in).
    pub battery_level: Option<u8>,
    /// Supported AI inference engines.
    pub ai_engines: Vec<String>,
}

/// Supported platforms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Android,
    Ios,
    Windows,
    Linux,
    Macos,
    Web,
    Unknown,
}

impl Platform {
    /// Detect the current platform at compile time.
    pub fn current() -> Self {
        #[cfg(target_os = "android")]
        return Self::Android;
        #[cfg(target_os = "ios")]
        return Self::Ios;
        #[cfg(target_os = "windows")]
        return Self::Windows;
        #[cfg(target_os = "linux")]
        return Self::Linux;
        #[cfg(target_os = "macos")]
        return Self::Macos;
        #[cfg(target_arch = "wasm32")]
        return Self::Web;
        #[cfg(not(any(
            target_os = "android",
            target_os = "ios",
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_arch = "wasm32"
        )))]
        return Self::Unknown;
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Android => write!(f, "android"),
            Self::Ios => write!(f, "ios"),
            Self::Windows => write!(f, "windows"),
            Self::Linux => write!(f, "linux"),
            Self::Macos => write!(f, "macos"),
            Self::Web => write!(f, "web"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Connection state for a peer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticated,
    Failed { reason: String },
}

/// Message priority levels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for MessagePriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// A message in the mesh network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshMessage {
    /// Unique message ID.
    pub id: String,
    /// Sender node ID.
    pub sender: NodeId,
    /// Recipient node ID (None = broadcast).
    pub recipient: Option<NodeId>,
    /// Message topic/channel.
    pub topic: String,
    /// Message payload (encrypted).
    pub payload: Vec<u8>,
    /// Message priority.
    pub priority: MessagePriority,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Time-to-live in seconds (0 = infinite).
    pub ttl_secs: u64,
    /// Number of hops this message has traversed.
    pub hop_count: u32,
}

impl MeshMessage {
    /// Create a new mesh message.
    pub fn new(
        sender: NodeId,
        recipient: Option<NodeId>,
        topic: impl Into<String>,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sender,
            recipient,
            topic: topic.into(),
            payload,
            priority: MessagePriority::Normal,
            created_at: Utc::now(),
            ttl_secs: 300,
            hop_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id_generation() {
        let id1 = DeviceId::new();
        let id2 = DeviceId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_node_info_creation() {
        let info = NodeInfo::new_local("test-node");
        assert_eq!(info.name, "test-node");
        assert!(info.is_online);
        assert_eq!(info.platform, Platform::current());
    }

    #[test]
    fn test_mesh_message() {
        let msg = MeshMessage::new(
            NodeId::new(),
            None,
            "test-topic",
            b"hello".to_vec(),
        );
        assert_eq!(msg.topic, "test-topic");
        assert_eq!(msg.hop_count, 0);
        assert_eq!(msg.priority, MessagePriority::Normal);
    }

    #[test]
    fn test_platform_current() {
        let platform = Platform::current();
        #[cfg(target_os = "windows")]
        assert_eq!(platform, Platform::Windows);
        #[cfg(target_os = "linux")]
        assert_eq!(platform, Platform::Linux);
        #[cfg(target_os = "macos")]
        assert_eq!(platform, Platform::Macos);
    }

    #[test]
    fn test_serialization() {
        let id = DeviceId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: DeviceId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }
}
