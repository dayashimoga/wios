//! Transport layer configuration.

use serde::{Deserialize, Serialize};

/// Transport protocol options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportProtocol {
    Tcp,
    Quic,
    WebSocket,
    WebRtc,
}

/// Transport configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    pub protocol: TransportProtocol,
    pub listen_address: String,
    pub max_connections: usize,
    pub timeout_secs: u64,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            protocol: TransportProtocol::Tcp,
            listen_address: "/ip4/0.0.0.0/tcp/0".into(),
            max_connections: 256,
            timeout_secs: 30,
        }
    }
}
