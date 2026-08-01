//! # WIOS AI
//!
//! Local AI inference engine supporting ONNX Runtime, TFLite, and llama.cpp.
//!
//! Provides:
//! - Model management (download, cache, version, quantize)
//! - Inference pipeline with task queuing
//! - Anomaly detection (z-score statistical)
//! - NLP intent parsing
//! - DAG workflow orchestration

pub mod engine;
pub mod model_manager;
pub mod pipeline;
pub mod backends;
pub mod intelligence;

pub use engine::InferenceEngine;
pub use model_manager::ModelManager;
pub use pipeline::InferencePipeline;
pub use backends::{AiBackend, OnnxBackend, TfLiteBackend, LlamaBackend};
pub use intelligence::{AnomalyDetector, NlpProcessor, WorkflowEngine};
