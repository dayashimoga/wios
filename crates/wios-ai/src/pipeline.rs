//! Inference pipeline with task queuing.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use wios_core::error::WiosResult;

/// A queued inference task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceTask {
    pub id: String,
    pub model_id: String,
    pub input: Vec<u8>,
    pub priority: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Result of an inference task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    pub task_id: String,
    pub output: Vec<u8>,
    pub duration_ms: u64,
}

/// Inference pipeline that queues and processes inference tasks.
pub struct InferencePipeline {
    queue: Arc<Mutex<VecDeque<InferenceTask>>>,
    max_queue_size: usize,
}

impl InferencePipeline {
    /// Create a new inference pipeline.
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_queue_size,
        }
    }

    /// Submit a task to the pipeline.
    pub async fn submit(
        &self,
        model_id: &str,
        input: Vec<u8>,
        priority: u32,
    ) -> WiosResult<String> {
        let task = InferenceTask {
            id: Uuid::new_v4().to_string(),
            model_id: model_id.into(),
            input,
            priority,
            created_at: chrono::Utc::now(),
        };
        let id = task.id.clone();
        let mut queue = self.queue.lock().await;
        if queue.len() >= self.max_queue_size {
            return Err(wios_core::error::WiosError::Inference("Queue full".into()));
        }
        queue.push_back(task);
        Ok(id)
    }

    /// Get queue depth.
    pub async fn queue_depth(&self) -> usize {
        self.queue.lock().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pipeline_submit() {
        let pipeline = InferencePipeline::new(100);
        let id = pipeline
            .submit("model1", b"input".to_vec(), 1)
            .await
            .unwrap();
        assert!(!id.is_empty());
        assert_eq!(pipeline.queue_depth().await, 1);
    }

    #[tokio::test]
    async fn test_pipeline_full() {
        let pipeline = InferencePipeline::new(1);
        pipeline.submit("m", b"a".to_vec(), 1).await.unwrap();
        assert!(pipeline.submit("m", b"b".to_vec(), 1).await.is_err());
    }
}
