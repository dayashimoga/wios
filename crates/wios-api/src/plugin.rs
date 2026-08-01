//! Plugin system — dynamic extension points for WIOS.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};

/// Plugin metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub entry_point: String,
}

/// Plugin lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginState {
    Registered,
    Loaded,
    Active,
    Disabled,
    Error,
}

/// Plugin runtime info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub manifest: PluginManifest,
    pub state: PluginState,
    pub load_time_ms: u64,
}

/// Plugin hook — functions plugins can implement.
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Plugin identifier.
    fn id(&self) -> &str;

    /// Called when the plugin is loaded.
    async fn on_load(&self) -> WiosResult<()>;

    /// Called when the plugin is unloaded.
    async fn on_unload(&self) -> WiosResult<()>;

    /// Handle a plugin-specific command.
    async fn handle_command(&self, command: &str, args: &[u8]) -> WiosResult<Vec<u8>>;
}

/// Plugin registry and lifecycle manager.
pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, PluginInfo>>>,
    handlers: Arc<RwLock<HashMap<String, Arc<dyn Plugin>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a plugin manifest.
    pub async fn register(&self, manifest: PluginManifest) -> WiosResult<()> {
        let id = manifest.id.clone();
        let info = PluginInfo {
            manifest,
            state: PluginState::Registered,
            load_time_ms: 0,
        };
        self.plugins.write().await.insert(id, info);
        Ok(())
    }

    /// Load and activate a plugin.
    pub async fn load(&self, id: &str, handler: Arc<dyn Plugin>) -> WiosResult<()> {
        let start = std::time::Instant::now();
        handler.on_load().await?;

        let mut plugins = self.plugins.write().await;
        if let Some(info) = plugins.get_mut(id) {
            info.state = PluginState::Active;
            info.load_time_ms = start.elapsed().as_millis() as u64;
        } else {
            return Err(WiosError::PluginError(format!(
                "Plugin {} not registered",
                id
            )));
        }

        self.handlers.write().await.insert(id.to_string(), handler);
        Ok(())
    }

    /// Unload a plugin.
    pub async fn unload(&self, id: &str) -> WiosResult<()> {
        if let Some(handler) = self.handlers.write().await.remove(id) {
            handler.on_unload().await?;
        }
        if let Some(info) = self.plugins.write().await.get_mut(id) {
            info.state = PluginState::Disabled;
        }
        Ok(())
    }

    /// Send a command to a plugin.
    pub async fn send_command(
        &self,
        plugin_id: &str,
        command: &str,
        args: &[u8],
    ) -> WiosResult<Vec<u8>> {
        let handlers = self.handlers.read().await;
        let handler = handlers
            .get(plugin_id)
            .ok_or(WiosError::PluginError(format!(
                "Plugin {} not loaded",
                plugin_id
            )))?;
        handler.handle_command(command, args).await
    }

    /// List all registered plugins.
    pub async fn list(&self) -> Vec<PluginInfo> {
        self.plugins.read().await.values().cloned().collect()
    }

    /// Get active plugin count.
    pub async fn active_count(&self) -> usize {
        self.plugins
            .read()
            .await
            .values()
            .filter(|p| p.state == PluginState::Active)
            .count()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;

    #[async_trait]
    impl Plugin for TestPlugin {
        fn id(&self) -> &str {
            "test-plugin"
        }
        async fn on_load(&self) -> WiosResult<()> {
            Ok(())
        }
        async fn on_unload(&self) -> WiosResult<()> {
            Ok(())
        }
        async fn handle_command(&self, cmd: &str, _args: &[u8]) -> WiosResult<Vec<u8>> {
            Ok(format!("handled: {}", cmd).into_bytes())
        }
    }

    #[tokio::test]
    async fn test_plugin_lifecycle() {
        let registry = PluginRegistry::new();
        let manifest = PluginManifest {
            id: "test-plugin".into(),
            name: "Test".into(),
            version: "1.0.0".into(),
            author: "WIOS".into(),
            description: "A test plugin".into(),
            permissions: vec!["network.read".into()],
            entry_point: "test.wasm".into(),
        };

        registry.register(manifest).await.unwrap();
        registry
            .load("test-plugin", Arc::new(TestPlugin))
            .await
            .unwrap();
        assert_eq!(registry.active_count().await, 1);

        let result = registry
            .send_command("test-plugin", "ping", b"")
            .await
            .unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "handled: ping");

        registry.unload("test-plugin").await.unwrap();
        assert_eq!(registry.active_count().await, 0);
    }
}
