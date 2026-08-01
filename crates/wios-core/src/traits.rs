//! Service trait definitions for WIOS.
//!
//! These traits define the interfaces that all service implementations
//! must satisfy, enabling dependency inversion and testability.

use crate::error::WiosResult;
use crate::types::*;
use async_trait::async_trait;
use std::collections::HashMap;

// We use async_trait until Rust's native async traits are fully stabilized
// for dyn dispatch use cases.

/// Cryptographic service interface.
#[async_trait]
pub trait CryptoService: Send + Sync {
    /// Generate a new Ed25519 keypair.
    async fn generate_keypair(&self) -> WiosResult<(Vec<u8>, Vec<u8>)>;

    /// Sign data with the node's private key.
    async fn sign(&self, data: &[u8]) -> WiosResult<Vec<u8>>;

    /// Verify a signature against a public key.
    async fn verify(&self, data: &[u8], signature: &[u8], public_key: &[u8]) -> WiosResult<bool>;

    /// Encrypt data for a specific recipient.
    async fn encrypt(&self, plaintext: &[u8], recipient_public_key: &[u8]) -> WiosResult<Vec<u8>>;

    /// Decrypt data with the node's private key.
    async fn decrypt(&self, ciphertext: &[u8]) -> WiosResult<Vec<u8>>;

    /// Hash a password using Argon2.
    async fn hash_password(&self, password: &str) -> WiosResult<String>;

    /// Verify a password against an Argon2 hash.
    async fn verify_password(&self, password: &str, hash: &str) -> WiosResult<bool>;
}

/// Storage service interface.
#[async_trait]
pub trait StorageService: Send + Sync {
    /// Store a key-value pair.
    async fn put(&self, namespace: &str, key: &str, value: &[u8]) -> WiosResult<()>;

    /// Retrieve a value by key.
    async fn get(&self, namespace: &str, key: &str) -> WiosResult<Option<Vec<u8>>>;

    /// Delete a key.
    async fn delete(&self, namespace: &str, key: &str) -> WiosResult<bool>;

    /// List all keys in a namespace.
    async fn list_keys(&self, namespace: &str, prefix: Option<&str>) -> WiosResult<Vec<String>>;

    /// Check if a key exists.
    async fn exists(&self, namespace: &str, key: &str) -> WiosResult<bool>;

    /// Get storage statistics.
    async fn stats(&self) -> WiosResult<StorageStats>;
}

/// Storage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_keys: u64,
    pub total_bytes: u64,
    pub namespaces: Vec<String>,
}

use serde::{Deserialize, Serialize};

/// Network/mesh service interface.
#[async_trait]
pub trait NetworkService: Send + Sync {
    /// Start the mesh network.
    async fn start(&self) -> WiosResult<()>;

    /// Stop the mesh network.
    async fn stop(&self) -> WiosResult<()>;

    /// Get list of connected peers.
    async fn peers(&self) -> WiosResult<Vec<NodeInfo>>;

    /// Send a message to a peer.
    async fn send_message(&self, message: MeshMessage) -> WiosResult<()>;

    /// Broadcast a message to all peers.
    async fn broadcast(&self, topic: &str, payload: Vec<u8>) -> WiosResult<()>;

    /// Get network statistics.
    async fn network_stats(&self) -> WiosResult<NetworkStats>;
}

/// Network statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub connected_peers: usize,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub uptime_secs: u64,
}

/// AI/inference service interface.
#[async_trait]
pub trait AiService: Send + Sync {
    /// List available models.
    async fn list_models(&self) -> WiosResult<Vec<ModelInfo>>;

    /// Load a model for inference.
    async fn load_model(&self, model_id: &str) -> WiosResult<()>;

    /// Unload a model.
    async fn unload_model(&self, model_id: &str) -> WiosResult<()>;

    /// Run inference on a model.
    async fn infer(&self, model_id: &str, input: &[u8]) -> WiosResult<Vec<u8>>;

    /// Run text generation (LLM).
    async fn generate_text(
        &self,
        model_id: &str,
        prompt: &str,
        max_tokens: u32,
    ) -> WiosResult<String>;
}

/// Model information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub model_type: ModelType,
    pub size_bytes: u64,
    pub is_loaded: bool,
}

/// Model type categories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    Onnx,
    TfLite,
    Gguf,
    Custom(String),
}

/// Compute service interface.
#[async_trait]
pub trait ComputeService: Send + Sync {
    /// Submit a distributed task.
    async fn submit_task(&self, task: ComputeTask) -> WiosResult<String>;

    /// Get task status.
    async fn task_status(&self, task_id: &str) -> WiosResult<TaskStatus>;

    /// Cancel a task.
    async fn cancel_task(&self, task_id: &str) -> WiosResult<()>;

    /// List all tasks.
    async fn list_tasks(&self) -> WiosResult<Vec<TaskStatus>>;

    /// Get available compute resources across the mesh.
    async fn available_resources(&self) -> WiosResult<HashMap<NodeId, DeviceCapabilities>>;
}

/// A distributed compute task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeTask {
    pub name: String,
    pub payload: Vec<u8>,
    pub required_capabilities: Vec<String>,
    pub priority: MessagePriority,
    pub timeout_secs: u64,
}

/// Task execution status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub task_id: String,
    pub name: String,
    pub state: TaskState,
    pub progress: f32,
    pub assigned_node: Option<NodeId>,
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
    pub submitted_at: Timestamp,
    pub completed_at: Option<Timestamp>,
}

/// Task lifecycle states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    Pending,
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}
