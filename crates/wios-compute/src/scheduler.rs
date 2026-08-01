//! Task scheduler for distributed compute.

use chrono::Utc;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use wios_core::error::{WiosError, WiosResult};
use wios_core::traits::{TaskState, TaskStatus};

/// Manages distributed compute task lifecycle.
pub struct TaskScheduler {
    tasks: Arc<RwLock<HashMap<String, TaskStatus>>>,
    max_concurrent: usize,
}

impl TaskScheduler {
    /// Create a new task scheduler.
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            max_concurrent,
        }
    }

    /// Submit a new task.
    pub async fn submit(&self, name: impl Into<String>, _payload: Vec<u8>) -> WiosResult<String> {
        let task_id = Uuid::new_v4().to_string();
        let status = TaskStatus {
            task_id: task_id.clone(),
            name: name.into(),
            state: TaskState::Pending,
            progress: 0.0,
            assigned_node: None,
            result: None,
            error: None,
            submitted_at: Utc::now(),
            completed_at: None,
        };

        let mut tasks = self.tasks.write().await;
        let running_count = tasks
            .values()
            .filter(|t| t.state == TaskState::Running)
            .count();
        if running_count >= self.max_concurrent {
            return Err(WiosError::TaskScheduling(
                "Max concurrent tasks reached".into(),
            ));
        }
        tasks.insert(task_id.clone(), status);
        Ok(task_id)
    }

    /// Get task status.
    pub async fn status(&self, task_id: &str) -> WiosResult<TaskStatus> {
        self.tasks
            .read()
            .await
            .get(task_id)
            .cloned()
            .ok_or_else(|| WiosError::Compute(format!("Task not found: {}", task_id)))
    }

    /// Update task state.
    pub async fn update_state(&self, task_id: &str, state: TaskState) -> WiosResult<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.state = state.clone();
            if state == TaskState::Completed
                || state == TaskState::Failed
                || state == TaskState::Cancelled
            {
                task.completed_at = Some(Utc::now());
            }
            Ok(())
        } else {
            Err(WiosError::Compute(format!("Task not found: {}", task_id)))
        }
    }

    /// List all tasks.
    pub async fn list_tasks(&self) -> Vec<TaskStatus> {
        self.tasks.read().await.values().cloned().collect()
    }

    /// Cancel a task.
    pub async fn cancel(&self, task_id: &str) -> WiosResult<()> {
        self.update_state(task_id, TaskState::Cancelled).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_lifecycle() {
        let scheduler = TaskScheduler::new(10);
        let id = scheduler
            .submit("test-task", b"payload".to_vec())
            .await
            .unwrap();

        let status = scheduler.status(&id).await.unwrap();
        assert_eq!(status.state, TaskState::Pending);

        scheduler
            .update_state(&id, TaskState::Running)
            .await
            .unwrap();
        let status = scheduler.status(&id).await.unwrap();
        assert_eq!(status.state, TaskState::Running);

        scheduler
            .update_state(&id, TaskState::Completed)
            .await
            .unwrap();
        let status = scheduler.status(&id).await.unwrap();
        assert_eq!(status.state, TaskState::Completed);
        assert!(status.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_cancel_task() {
        let scheduler = TaskScheduler::new(10);
        let id = scheduler.submit("test", b"data".to_vec()).await.unwrap();
        scheduler.cancel(&id).await.unwrap();
        let status = scheduler.status(&id).await.unwrap();
        assert_eq!(status.state, TaskState::Cancelled);
    }
}
