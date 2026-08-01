//! Configuration management for WIOS.
//!
//! Supports hierarchical configuration from multiple sources:
//! 1. Built-in defaults
//! 2. Configuration file (TOML)
//! 3. Environment variables (WIOS_ prefix)
//! 4. Runtime overrides

use crate::error::{WiosError, WiosResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Top-level WIOS configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WiosConfig {
    /// Node identity configuration.
    pub node: NodeConfig,
    /// Network/mesh configuration.
    pub network: NetworkConfig,
    /// Storage configuration.
    pub storage: StorageConfig,
    /// Security configuration.
    pub security: SecurityConfig,
    /// AI/inference configuration.
    pub ai: AiConfig,
    /// Compute configuration.
    pub compute: ComputeConfig,
    /// Logging configuration.
    pub logging: LoggingConfig,
    /// API server configuration.
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Human-readable node name.
    pub name: String,
    /// Data directory path.
    pub data_dir: PathBuf,
    /// Maximum number of concurrent connections.
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listen addresses for the mesh network.
    pub listen_addresses: Vec<String>,
    /// Bootstrap peers for initial discovery.
    pub bootstrap_peers: Vec<String>,
    /// Enable mDNS peer discovery.
    pub mdns_enabled: bool,
    /// Enable Kademlia DHT.
    pub dht_enabled: bool,
    /// Enable Gossipsub.
    pub gossipsub_enabled: bool,
    /// Maximum number of mesh peers.
    pub max_peers: usize,
    /// Connection timeout in seconds.
    pub connection_timeout_secs: u64,
    /// Enable relay (for NAT traversal).
    pub relay_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// SQLite database path.
    pub sqlite_path: PathBuf,
    /// RocksDB data directory.
    pub rocksdb_path: PathBuf,
    /// Enable encryption at rest.
    pub encrypt_at_rest: bool,
    /// Maximum storage quota in bytes (0 = unlimited).
    pub max_quota_bytes: u64,
    /// Sync interval in seconds.
    pub sync_interval_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Key storage directory.
    pub keystore_path: PathBuf,
    /// Enable zero-trust mode.
    pub zero_trust: bool,
    /// Session timeout in seconds.
    pub session_timeout_secs: u64,
    /// Maximum failed login attempts before lockout.
    pub max_login_attempts: u32,
    /// Lockout duration in seconds.
    pub lockout_duration_secs: u64,
    /// Enable audit logging.
    pub audit_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Directory for AI model storage.
    pub models_dir: PathBuf,
    /// Maximum memory for inference in MB.
    pub max_memory_mb: u64,
    /// Number of inference threads.
    pub inference_threads: usize,
    /// Enable GPU acceleration.
    pub gpu_enabled: bool,
    /// Default model for NLP tasks.
    pub default_nlp_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeConfig {
    /// Maximum CPU cores to share.
    pub max_shared_cores: usize,
    /// Maximum RAM to share in MB.
    pub max_shared_ram_mb: u64,
    /// Maximum storage to share in MB.
    pub max_shared_storage_mb: u64,
    /// Enable GPU sharing.
    pub share_gpu: bool,
    /// Task timeout in seconds.
    pub task_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error).
    pub level: String,
    /// Log output format (json, pretty).
    pub format: String,
    /// Log file path (None = stdout only).
    pub file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// REST API listen address.
    pub rest_address: String,
    /// gRPC API listen address.
    pub grpc_address: String,
    /// Enable CORS.
    pub cors_enabled: bool,
    /// API rate limit (requests per second, 0 = unlimited).
    pub rate_limit_rps: u32,
}

impl Default for WiosConfig {
    fn default() -> Self {
        let data_dir = dirs_default_data_dir();
        Self {
            node: NodeConfig {
                name: "wios-node".into(),
                data_dir: data_dir.clone(),
                max_connections: 256,
            },
            network: NetworkConfig {
                listen_addresses: vec![
                    "/ip4/0.0.0.0/tcp/0".into(),
                    "/ip4/0.0.0.0/udp/0/quic-v1".into(),
                ],
                bootstrap_peers: vec![],
                mdns_enabled: true,
                dht_enabled: true,
                gossipsub_enabled: true,
                max_peers: 64,
                connection_timeout_secs: 30,
                relay_enabled: true,
            },
            storage: StorageConfig {
                sqlite_path: data_dir.join("wios.db"),
                rocksdb_path: data_dir.join("rocksdb"),
                encrypt_at_rest: true,
                max_quota_bytes: 0,
                sync_interval_secs: 30,
            },
            security: SecurityConfig {
                keystore_path: data_dir.join("keystore"),
                zero_trust: true,
                session_timeout_secs: 3600,
                max_login_attempts: 5,
                lockout_duration_secs: 300,
                audit_logging: true,
            },
            ai: AiConfig {
                models_dir: data_dir.join("models"),
                max_memory_mb: 2048,
                inference_threads: 4,
                gpu_enabled: false,
                default_nlp_model: None,
            },
            compute: ComputeConfig {
                max_shared_cores: 2,
                max_shared_ram_mb: 1024,
                max_shared_storage_mb: 4096,
                share_gpu: false,
                task_timeout_secs: 300,
            },
            logging: LoggingConfig {
                level: "info".into(),
                format: "pretty".into(),
                file: None,
            },
            api: ApiConfig {
                rest_address: "127.0.0.1:8080".into(),
                grpc_address: "127.0.0.1:50051".into(),
                cors_enabled: true,
                rate_limit_rps: 100,
            },
        }
    }
}

impl WiosConfig {
    /// Load configuration from file, falling back to defaults.
    pub fn load(path: Option<&Path>) -> WiosResult<Self> {
        match path {
            Some(p) => Self::load_from_file(p),
            None => {
                let default_path = dirs_default_data_dir().join("config.toml");
                if default_path.exists() {
                    Self::load_from_file(&default_path)
                } else {
                    Ok(Self::default())
                }
            }
        }
    }

    /// Load configuration from a specific TOML file.
    pub fn load_from_file(path: &Path) -> WiosResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| WiosError::Config(e.to_string()))?;
        toml::from_str(&content).map_err(|e| WiosError::Config(e.to_string()))
    }

    /// Save configuration to a TOML file.
    pub fn save(&self, path: &Path) -> WiosResult<()> {
        let content =
            toml::to_string_pretty(self).map_err(|e| WiosError::Config(e.to_string()))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate the configuration values.
    pub fn validate(&self) -> WiosResult<()> {
        if self.node.name.is_empty() {
            return Err(WiosError::ConfigInvalid {
                key: "node.name".into(),
                reason: "Node name cannot be empty".into(),
            });
        }
        if self.network.max_peers == 0 {
            return Err(WiosError::ConfigInvalid {
                key: "network.max_peers".into(),
                reason: "Must allow at least 1 peer".into(),
            });
        }
        if self.security.max_login_attempts == 0 {
            return Err(WiosError::ConfigInvalid {
                key: "security.max_login_attempts".into(),
                reason: "Must allow at least 1 login attempt".into(),
            });
        }
        Ok(())
    }

    /// Initialize the logging subsystem based on configuration.
    pub fn init_logging(&self) -> WiosResult<()> {
        let filter = tracing_subscriber::EnvFilter::try_new(&self.logging.level)
            .map_err(|e| WiosError::Config(format!("Invalid log level: {}", e)))?;

        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);

        match self.logging.format.as_str() {
            "json" => subscriber
                .json()
                .try_init()
                .map_err(|e| WiosError::Config(e.to_string())),
            _ => subscriber
                .try_init()
                .map_err(|e| WiosError::Config(e.to_string())),
        }
    }
}

/// Get the default data directory for the platform.
fn dirs_default_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"))
            .join("WIOS")
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"))
            .join("Library/Application Support/WIOS")
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join(".local/share"))
                    .unwrap_or_else(|_| PathBuf::from("/tmp"))
            })
            .join("wios")
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from(".wios")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WiosConfig::default();
        assert_eq!(config.node.name, "wios-node");
        assert!(config.network.mdns_enabled);
        assert!(config.security.zero_trust);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = WiosConfig::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: WiosConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.node.name, config.node.name);
    }

    #[test]
    fn test_config_validation_empty_name() {
        let mut config = WiosConfig::default();
        config.node.name = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_save_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_config.toml");
        let config = WiosConfig::default();
        config.save(&path).unwrap();
        let loaded = WiosConfig::load_from_file(&path).unwrap();
        assert_eq!(loaded.node.name, config.node.name);
    }
}
