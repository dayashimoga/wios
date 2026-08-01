//! Error types for all WIOS subsystems.
//!
//! Provides a unified error hierarchy that all crates can use for
//! consistent error handling and propagation.

use thiserror::Error;

/// Top-level WIOS error type.
#[derive(Error, Debug)]
pub enum WiosError {
    // ── Configuration ──────────────────────────────────────────────
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: String },

    #[error("Invalid configuration value for '{key}': {reason}")]
    ConfigInvalid { key: String, reason: String },

    // ── Cryptography ───────────────────────────────────────────────
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("Key generation failed: {0}")]
    KeyGeneration(String),

    #[error("Encryption failed: {0}")]
    Encryption(String),

    #[error("Decryption failed: {0}")]
    Decryption(String),

    #[error("Signature verification failed")]
    SignatureVerification,

    #[error("Key not found: {key_id}")]
    KeyNotFound { key_id: String },

    // ── Storage ────────────────────────────────────────────────────
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Database migration failed: {0}")]
    Migration(String),

    #[error("Record not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Sync conflict: {0}")]
    SyncConflict(String),

    // ── Networking ─────────────────────────────────────────────────
    #[error("Network error: {0}")]
    Network(String),

    #[error("Peer not found: {peer_id}")]
    PeerNotFound { peer_id: String },

    #[error("Connection failed to {address}: {reason}")]
    ConnectionFailed { address: String, reason: String },

    #[error("Message delivery failed: {0}")]
    MessageDelivery(String),

    #[error("Routing failed: no path to {destination}")]
    RoutingFailed { destination: String },

    // ── AI / Inference ─────────────────────────────────────────────
    #[error("AI inference error: {0}")]
    Inference(String),

    #[error("Model not found: {model_id}")]
    ModelNotFound { model_id: String },

    #[error("Model loading failed: {0}")]
    ModelLoad(String),

    // ── Compute ────────────────────────────────────────────────────
    #[error("Compute error: {0}")]
    Compute(String),

    #[error("Task scheduling failed: {0}")]
    TaskScheduling(String),

    #[error("Resource unavailable: {resource}")]
    ResourceUnavailable { resource: String },

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    // ── Authentication / Authorization ─────────────────────────────
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid permissions: {0}")]
    InvalidPermissions(String),

    // ── Plugin / SDK ───────────────────────────────────────────────
    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("Plugin not found: {plugin_id}")]
    PluginNotFound { plugin_id: String },

    #[error("API error: {status} — {message}")]
    Api { status: u16, message: String },

    // ── Device / Sensing ───────────────────────────────────────────
    #[error("Device capability unavailable: {capability}")]
    CapabilityUnavailable { capability: String },

    #[error("Sensor error: {0}")]
    Sensor(String),

    #[error("Positioning error: {0}")]
    Positioning(String),

    // ── Generic ────────────────────────────────────────────────────
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Convenience Result type for WIOS operations.
pub type WiosResult<T> = Result<T, WiosError>;

impl WiosError {
    /// Returns a machine-readable error code for API responses.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Config(_) | Self::ConfigNotFound { .. } | Self::ConfigInvalid { .. } => {
                "CONFIG_ERROR"
            }
            Self::Crypto(_)
            | Self::KeyGeneration(_)
            | Self::Encryption(_)
            | Self::Decryption(_)
            | Self::SignatureVerification
            | Self::KeyNotFound { .. } => "CRYPTO_ERROR",
            Self::Storage(_)
            | Self::Migration(_)
            | Self::NotFound { .. }
            | Self::SyncConflict(_) => "STORAGE_ERROR",
            Self::Network(_)
            | Self::PeerNotFound { .. }
            | Self::ConnectionFailed { .. }
            | Self::MessageDelivery(_)
            | Self::RoutingFailed { .. } => "NETWORK_ERROR",
            Self::Inference(_) | Self::ModelNotFound { .. } | Self::ModelLoad(_) => "AI_ERROR",
            Self::Compute(_)
            | Self::TaskScheduling(_)
            | Self::ResourceUnavailable { .. }
            | Self::TaskNotFound(_) => "COMPUTE_ERROR",
            Self::AuthFailed(_)
            | Self::AccessDenied(_)
            | Self::TokenExpired
            | Self::InvalidPermissions(_) => "AUTH_ERROR",
            Self::Plugin(_)
            | Self::PluginError(_)
            | Self::PluginNotFound { .. }
            | Self::Api { .. } => "PLUGIN_ERROR",
            Self::CapabilityUnavailable { .. } | Self::Sensor(_) | Self::Positioning(_) => {
                "DEVICE_ERROR"
            }
            Self::Internal(_) | Self::NotImplemented(_) => "INTERNAL_ERROR",
            Self::Serialization(_) | Self::Json(_) => "SERIALIZATION_ERROR",
            Self::Io(_) => "IO_ERROR",
        }
    }

    /// Returns whether this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Network(_)
                | Self::ConnectionFailed { .. }
                | Self::MessageDelivery(_)
                | Self::RoutingFailed { .. }
                | Self::ResourceUnavailable { .. }
                | Self::Io(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let err = WiosError::Config("test".into());
        assert_eq!(err.error_code(), "CONFIG_ERROR");

        let err = WiosError::PeerNotFound {
            peer_id: "abc".into(),
        };
        assert_eq!(err.error_code(), "NETWORK_ERROR");
    }

    #[test]
    fn test_retryable() {
        let err = WiosError::Network("timeout".into());
        assert!(err.is_retryable());

        let err = WiosError::Config("bad".into());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_error_display() {
        let err = WiosError::ConnectionFailed {
            address: "192.168.1.1:9000".into(),
            reason: "timeout".into(),
        };
        assert_eq!(
            err.to_string(),
            "Connection failed to 192.168.1.1:9000: timeout"
        );
    }
}
