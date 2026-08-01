//! Model management — download, cache, version, delete.

use std::path::{Path, PathBuf};
use tracing::info;
use wios_core::error::WiosResult;

/// Manages model lifecycle on disk.
pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    /// Create a new model manager.
    pub fn new(models_dir: impl AsRef<Path>) -> WiosResult<Self> {
        let dir = models_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir)?;
        Ok(Self { models_dir: dir })
    }

    /// Get the path for a model.
    pub fn model_path(&self, model_id: &str, extension: &str) -> PathBuf {
        self.models_dir.join(format!("{}.{}", model_id, extension))
    }

    /// Check if a model exists on disk.
    pub fn model_exists(&self, model_id: &str) -> bool {
        ["onnx", "tflite", "gguf"]
            .iter()
            .any(|ext| self.model_path(model_id, ext).exists())
    }

    /// Delete a model from disk.
    pub fn delete_model(&self, model_id: &str) -> WiosResult<()> {
        for ext in &["onnx", "tflite", "gguf"] {
            let path = self.model_path(model_id, ext);
            if path.exists() {
                std::fs::remove_file(&path)?;
                info!("Deleted model: {}", path.display());
            }
        }
        Ok(())
    }

    /// Get total disk usage of all models.
    pub fn total_disk_usage(&self) -> WiosResult<u64> {
        let mut total = 0u64;
        if let Ok(entries) = std::fs::read_dir(&self.models_dir) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    total += meta.len();
                }
            }
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_manager() {
        let dir = tempfile::tempdir().unwrap();
        let manager = ModelManager::new(dir.path()).unwrap();
        assert!(!manager.model_exists("test-model"));
        assert_eq!(manager.total_disk_usage().unwrap(), 0);
    }
}
