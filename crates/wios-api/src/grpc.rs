//! gRPC server — service trait and implementation.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wios_core::error::WiosResult;

/// gRPC service request/response types (mirrors proto definitions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub uptime_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoRequest {
    pub node_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoResponse {
    pub node_id: String,
    pub name: String,
    pub platform: String,
    pub cpu_cores: u32,
    pub ram_mb: u64,
}

/// WIOS gRPC service trait.
#[async_trait]
pub trait WiosGrpcService: Send + Sync + 'static {
    async fn health_check(&self) -> WiosResult<HealthCheckResponse>;
    async fn get_node_info(&self, name: String) -> WiosResult<NodeInfoResponse>;
}

/// Default implementation.
pub struct WiosGrpcServer {
    start_time: std::time::Instant,
}

impl WiosGrpcServer {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
}

impl Default for WiosGrpcServer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WiosGrpcService for WiosGrpcServer {
    async fn health_check(&self) -> WiosResult<HealthCheckResponse> {
        Ok(HealthCheckResponse {
            status: "healthy".into(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_secs: self.start_time.elapsed().as_secs(),
        })
    }

    async fn get_node_info(&self, name: String) -> WiosResult<NodeInfoResponse> {
        let info = wios_core::types::NodeInfo::new_local(name);
        let caps_cores = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(1);
        Ok(NodeInfoResponse {
            node_id: info.node_id.to_string(),
            name: info.name,
            platform: info.platform.to_string(),
            cpu_cores: caps_cores,
            ram_mb: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let server = WiosGrpcServer::new();
        let resp = server.health_check().await.unwrap();
        assert_eq!(resp.status, "healthy");
        assert!(!resp.version.is_empty());
    }

    #[tokio::test]
    async fn test_node_info() {
        let server = WiosGrpcServer::new();
        let resp = server.get_node_info("test-node".into()).await.unwrap();
        assert_eq!(resp.name, "test-node");
        assert!(resp.cpu_cores >= 1);
    }
}
