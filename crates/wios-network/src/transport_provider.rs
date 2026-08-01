//! Common transport provider trait.
//!
//! Abstracts multiple wireless transports (WiFi Direct, BLE, QUIC, LoRa, UWB)
//! behind a unified interface — the "AI Radio Translator" concept.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::WiosResult;
use wios_core::types::NodeId;

/// Supported transport types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportType {
    Tcp,
    Quic,
    WifiDirect,
    BluetoothLe,
    BluetoothMesh,
    WifiAware,
    Uwb,
    LoRa,
    WebSocket,
    Custom(String),
}

/// Transport capabilities and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportInfo {
    pub transport_type: TransportType,
    pub available: bool,
    pub max_bandwidth_kbps: u64,
    pub max_range_meters: f64,
    pub supports_broadcast: bool,
    pub supports_mesh: bool,
    pub power_consumption: PowerLevel,
}

/// Power consumption level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PowerLevel {
    UltraLow,
    Low,
    Medium,
    High,
}

/// Message to send over a transport.
#[derive(Debug, Clone)]
pub struct TransportMessage {
    pub destination: NodeId,
    pub payload: Vec<u8>,
    pub reliable: bool,
    pub priority: u8,
}

/// Common trait all transport providers must implement.
#[async_trait]
pub trait TransportProvider: Send + Sync {
    /// Transport type identifier.
    fn transport_type(&self) -> TransportType;

    /// Get transport capabilities info.
    fn info(&self) -> TransportInfo;

    /// Whether the transport is currently available on this device.
    fn is_available(&self) -> bool;

    /// Start the transport (bind, listen).
    async fn start(&mut self) -> WiosResult<()>;

    /// Stop the transport.
    async fn stop(&mut self) -> WiosResult<()>;

    /// Send a message to a specific node.
    async fn send(&self, message: TransportMessage) -> WiosResult<()>;

    /// Broadcast to all reachable nodes.
    async fn broadcast(&self, payload: &[u8]) -> WiosResult<u32>;

    /// Discover peers via this transport.
    async fn discover(&self) -> WiosResult<Vec<NodeId>>;
}

/// Transport registry managing multiple providers.
pub struct TransportRegistry {
    providers: HashMap<TransportType, Box<dyn TransportProvider>>,
}

impl TransportRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a transport provider.
    pub fn register(&mut self, provider: Box<dyn TransportProvider>) {
        let tt = provider.transport_type();
        self.providers.insert(tt, provider);
    }

    /// Get available transports.
    pub fn available(&self) -> Vec<TransportInfo> {
        self.providers
            .values()
            .filter(|p| p.is_available())
            .map(|p| p.info())
            .collect()
    }

    /// Select best transport for a message based on priority and bandwidth.
    pub fn select_best(&self, _reliable: bool) -> Option<TransportType> {
        let mut candidates: Vec<_> = self
            .providers
            .values()
            .filter(|p| p.is_available())
            .map(|p| p.info())
            .collect();
        candidates.sort_by_key(|b| std::cmp::Reverse(b.max_bandwidth_kbps));
        candidates.first().map(|c| c.transport_type.clone())
    }

    /// Get a specific provider.
    pub fn get(&self, tt: &TransportType) -> Option<&dyn TransportProvider> {
        self.providers.get(tt).map(|p| p.as_ref())
    }

    /// Count of registered providers.
    pub fn count(&self) -> usize {
        self.providers.len()
    }
}

impl Default for TransportRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub TCP transport for testing.
pub struct StubTransport {
    transport_type: TransportType,
    available: bool,
    started: bool,
}

impl StubTransport {
    pub fn new(tt: TransportType, available: bool) -> Self {
        Self {
            transport_type: tt,
            available,
            started: false,
        }
    }
}

#[async_trait]
impl TransportProvider for StubTransport {
    fn transport_type(&self) -> TransportType {
        self.transport_type.clone()
    }

    fn info(&self) -> TransportInfo {
        TransportInfo {
            transport_type: self.transport_type.clone(),
            available: self.available,
            max_bandwidth_kbps: match &self.transport_type {
                TransportType::Tcp => 100_000,
                TransportType::Quic => 100_000,
                TransportType::BluetoothLe => 2_000,
                TransportType::LoRa => 50,
                _ => 50_000,
            },
            max_range_meters: match &self.transport_type {
                TransportType::BluetoothLe => 100.0,
                TransportType::LoRa => 15_000.0,
                TransportType::WifiDirect => 200.0,
                _ => 1000.0,
            },
            supports_broadcast: matches!(
                self.transport_type,
                TransportType::BluetoothMesh | TransportType::LoRa
            ),
            supports_mesh: matches!(self.transport_type, TransportType::BluetoothMesh),
            power_consumption: match &self.transport_type {
                TransportType::BluetoothLe => PowerLevel::UltraLow,
                TransportType::LoRa => PowerLevel::Low,
                _ => PowerLevel::Medium,
            },
        }
    }

    fn is_available(&self) -> bool {
        self.available
    }

    async fn start(&mut self) -> WiosResult<()> {
        self.started = true;
        Ok(())
    }
    async fn stop(&mut self) -> WiosResult<()> {
        self.started = false;
        Ok(())
    }

    async fn send(&self, _message: TransportMessage) -> WiosResult<()> {
        Ok(())
    }
    async fn broadcast(&self, _payload: &[u8]) -> WiosResult<u32> {
        Ok(0)
    }
    async fn discover(&self) -> WiosResult<Vec<NodeId>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_registry() {
        let mut reg = TransportRegistry::new();
        reg.register(Box::new(StubTransport::new(TransportType::Tcp, true)));
        reg.register(Box::new(StubTransport::new(
            TransportType::BluetoothLe,
            true,
        )));
        reg.register(Box::new(StubTransport::new(TransportType::LoRa, false)));

        assert_eq!(reg.count(), 3);
        assert_eq!(reg.available().len(), 2); // LoRa unavailable
    }

    #[test]
    fn test_select_best_transport() {
        let mut reg = TransportRegistry::new();
        reg.register(Box::new(StubTransport::new(TransportType::Tcp, true)));
        reg.register(Box::new(StubTransport::new(
            TransportType::BluetoothLe,
            true,
        )));

        let best = reg.select_best(true);
        assert_eq!(best, Some(TransportType::Tcp)); // TCP has higher bandwidth
    }

    #[test]
    fn test_transport_info() {
        let stub = StubTransport::new(TransportType::LoRa, true);
        let info = stub.info();
        assert_eq!(info.max_bandwidth_kbps, 50);
        assert_eq!(info.max_range_meters, 15_000.0);
        assert!(info.supports_broadcast);
    }
}
