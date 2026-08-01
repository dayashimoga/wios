//! Pluggable AI backend trait — ONNX, TFLite, llama.cpp implementations slot in here.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wios_core::error::WiosResult;

/// AI model format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelFormat {
    Onnx,
    TfLite,
    Gguf,
}

/// Model metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub format: ModelFormat,
    pub size_bytes: u64,
    pub quantized: bool,
    pub task: String,
}

/// Inference input tensor.
#[derive(Debug, Clone)]
pub struct Tensor {
    pub name: String,
    pub data: Vec<f32>,
    pub shape: Vec<usize>,
}

/// Inference output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceOutput {
    pub model_id: String,
    pub values: Vec<f32>,
    pub shape: Vec<usize>,
    pub duration_ms: u64,
}

/// Trait that ONNX, TFLite, and llama.cpp backends implement.
#[async_trait]
pub trait AiBackend: Send + Sync {
    /// Load a model from disk.
    async fn load_model(&self, path: &str) -> WiosResult<ModelInfo>;

    /// Unload a model from memory.
    async fn unload_model(&self, model_id: &str) -> WiosResult<()>;

    /// Run inference.
    async fn infer(&self, model_id: &str, inputs: Vec<Tensor>) -> WiosResult<InferenceOutput>;

    /// List loaded models.
    async fn loaded_models(&self) -> Vec<ModelInfo>;

    /// Get the supported format.
    fn supported_format(&self) -> ModelFormat;
}

/// Stub ONNX backend (real impl uses `ort` crate).
pub struct OnnxBackend;

#[async_trait]
impl AiBackend for OnnxBackend {
    async fn load_model(&self, path: &str) -> WiosResult<ModelInfo> {
        Ok(ModelInfo {
            id: uuid::Uuid::new_v4().to_string(),
            name: path.to_string(),
            format: ModelFormat::Onnx,
            size_bytes: 0,
            quantized: false,
            task: "classification".into(),
        })
    }

    async fn unload_model(&self, _model_id: &str) -> WiosResult<()> {
        Ok(())
    }

    async fn infer(&self, model_id: &str, _inputs: Vec<Tensor>) -> WiosResult<InferenceOutput> {
        Ok(InferenceOutput {
            model_id: model_id.into(),
            values: vec![0.0],
            shape: vec![1],
            duration_ms: 0,
        })
    }

    async fn loaded_models(&self) -> Vec<ModelInfo> {
        vec![]
    }
    fn supported_format(&self) -> ModelFormat {
        ModelFormat::Onnx
    }
}

/// Stub TFLite backend.
pub struct TfLiteBackend;

#[async_trait]
impl AiBackend for TfLiteBackend {
    async fn load_model(&self, path: &str) -> WiosResult<ModelInfo> {
        Ok(ModelInfo {
            id: uuid::Uuid::new_v4().to_string(),
            name: path.to_string(),
            format: ModelFormat::TfLite,
            size_bytes: 0,
            quantized: false,
            task: "classification".into(),
        })
    }
    async fn unload_model(&self, _model_id: &str) -> WiosResult<()> {
        Ok(())
    }
    async fn infer(&self, model_id: &str, _inputs: Vec<Tensor>) -> WiosResult<InferenceOutput> {
        Ok(InferenceOutput {
            model_id: model_id.into(),
            values: vec![0.0],
            shape: vec![1],
            duration_ms: 0,
        })
    }
    async fn loaded_models(&self) -> Vec<ModelInfo> {
        vec![]
    }
    fn supported_format(&self) -> ModelFormat {
        ModelFormat::TfLite
    }
}

/// Stub llama.cpp backend for LLM inference.
pub struct LlamaBackend;

#[async_trait]
impl AiBackend for LlamaBackend {
    async fn load_model(&self, path: &str) -> WiosResult<ModelInfo> {
        Ok(ModelInfo {
            id: uuid::Uuid::new_v4().to_string(),
            name: path.to_string(),
            format: ModelFormat::Gguf,
            size_bytes: 0,
            quantized: true,
            task: "text-generation".into(),
        })
    }
    async fn unload_model(&self, _model_id: &str) -> WiosResult<()> {
        Ok(())
    }
    async fn infer(&self, model_id: &str, _inputs: Vec<Tensor>) -> WiosResult<InferenceOutput> {
        Ok(InferenceOutput {
            model_id: model_id.into(),
            values: vec![],
            shape: vec![0],
            duration_ms: 0,
        })
    }
    async fn loaded_models(&self) -> Vec<ModelInfo> {
        vec![]
    }
    fn supported_format(&self) -> ModelFormat {
        ModelFormat::Gguf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_onnx_backend() {
        let backend = OnnxBackend;
        assert_eq!(backend.supported_format(), ModelFormat::Onnx);
        let info = backend.load_model("test.onnx").await.unwrap();
        assert_eq!(info.format, ModelFormat::Onnx);
    }

    #[tokio::test]
    async fn test_tflite_backend() {
        let backend = TfLiteBackend;
        assert_eq!(backend.supported_format(), ModelFormat::TfLite);
    }

    #[tokio::test]
    async fn test_llama_backend() {
        let backend = LlamaBackend;
        let info = backend.load_model("model.gguf").await.unwrap();
        assert!(info.quantized);
        assert_eq!(info.task, "text-generation");
    }
}
