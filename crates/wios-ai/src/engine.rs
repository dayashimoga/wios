//! Inference engine abstraction.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::WiosResult;
use wios_core::traits::{ModelInfo, ModelType};

/// Supported inference backends.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InferenceBackend {
    OnnxRuntime,
    TfLite,
    LlamaCpp,
}

/// Inference engine that manages models and runs inference.
pub struct InferenceEngine {
    loaded_models: Arc<RwLock<HashMap<String, LoadedModel>>>,
    models_dir: std::path::PathBuf,
    max_memory_mb: u64,
}

/// A model loaded into memory for inference.
#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub info: ModelInfo,
    pub backend: InferenceBackend,
    pub loaded_at: DateTime<Utc>,
    pub inference_count: u64,
    pub total_inference_time_ms: u64,
}

impl InferenceEngine {
    /// Create a new inference engine.
    pub fn new(models_dir: impl Into<std::path::PathBuf>, max_memory_mb: u64) -> Self {
        let dir = models_dir.into();
        if let Err(e) = std::fs::create_dir_all(&dir) {
            tracing::warn!("Could not create models directory: {}", e);
        }
        Self {
            loaded_models: Arc::new(RwLock::new(HashMap::new())),
            models_dir: dir,
            max_memory_mb,
        }
    }

    /// List all available models (on disk).
    pub async fn list_available_models(&self) -> WiosResult<Vec<ModelInfo>> {
        let mut models = Vec::new();
        let loaded = self.loaded_models.read().await;

        if self.models_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&self.models_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let model_type = match ext {
                            "onnx" => Some(ModelType::Onnx),
                            "tflite" => Some(ModelType::TfLite),
                            "gguf" => Some(ModelType::Gguf),
                            _ => None,
                        };

                        if let Some(mt) = model_type {
                            let name = path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("unknown")
                                .to_string();
                            let id = name.clone();
                            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                            let is_loaded = loaded.contains_key(&id);

                            models.push(ModelInfo {
                                id,
                                name,
                                model_type: mt,
                                size_bytes: size,
                                is_loaded,
                            });
                        }
                    }
                }
            }
        }
        Ok(models)
    }

    /// Get list of loaded models.
    pub async fn loaded_models(&self) -> Vec<LoadedModel> {
        self.loaded_models.read().await.values().cloned().collect()
    }

    /// Get model info if loaded.
    pub async fn get_loaded_model(&self, model_id: &str) -> Option<LoadedModel> {
        self.loaded_models.read().await.get(model_id).cloned()
    }

    /// Get engine statistics.
    pub async fn stats(&self) -> EngineStats {
        let models = self.loaded_models.read().await;
        EngineStats {
            loaded_models: models.len(),
            total_inferences: models.values().map(|m| m.inference_count).sum(),
            max_memory_mb: self.max_memory_mb,
        }
    }
}

/// Engine statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStats {
    pub loaded_models: usize,
    pub total_inferences: u64,
    pub max_memory_mb: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let dir = tempfile::tempdir().unwrap();
        let engine = InferenceEngine::new(dir.path(), 2048);
        let stats = engine.stats().await;
        assert_eq!(stats.loaded_models, 0);
        assert_eq!(stats.max_memory_mb, 2048);
    }

    #[tokio::test]
    async fn test_list_empty_models() {
        let dir = tempfile::tempdir().unwrap();
        let engine = InferenceEngine::new(dir.path(), 2048);
        let models = engine.list_available_models().await.unwrap();
        assert!(models.is_empty());
    }
}
