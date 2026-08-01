//! AI anomaly detection and NLP interface.

use serde::{Deserialize, Serialize};

/// An anomaly detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub id: String,
    pub source: String,
    pub metric: String,
    pub expected: f64,
    pub actual: f64,
    pub deviation: f64,
    pub severity: AnomalySeverity,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Simple statistical anomaly detector using z-score.
pub struct AnomalyDetector {
    baselines: std::collections::HashMap<String, Vec<f64>>,
    threshold: f64,
    window_size: usize,
}

impl AnomalyDetector {
    pub fn new(threshold: f64, window_size: usize) -> Self {
        Self {
            baselines: std::collections::HashMap::new(),
            threshold,
            window_size,
        }
    }

    /// Feed a data point and check for anomaly.
    pub fn check(&mut self, metric: &str, value: f64) -> Option<Anomaly> {
        let history = self.baselines.entry(metric.to_string()).or_default();
        history.push(value);
        if history.len() > self.window_size {
            history.remove(0);
        }
        if history.len() < 5 {
            return None;
        } // Need min samples

        let mean: f64 = history.iter().sum::<f64>() / history.len() as f64;
        let variance: f64 =
            history.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / history.len() as f64;
        let std_dev = variance.sqrt();
        if std_dev < 1e-9 {
            return None;
        }

        let z_score = (value - mean).abs() / std_dev;
        if z_score > self.threshold {
            let severity = if z_score > 4.0 {
                AnomalySeverity::Critical
            } else if z_score > 3.0 {
                AnomalySeverity::High
            } else if z_score > 2.5 {
                AnomalySeverity::Medium
            } else {
                AnomalySeverity::Low
            };
            Some(Anomaly {
                id: uuid::Uuid::new_v4().to_string(),
                source: "anomaly_detector".into(),
                metric: metric.into(),
                expected: mean,
                actual: value,
                deviation: z_score,
                severity,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            })
        } else {
            None
        }
    }
}

/// NLP intent from user input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NlpIntent {
    pub intent: String,
    pub entities: std::collections::HashMap<String, String>,
    pub confidence: f64,
    pub raw_text: String,
}

/// Simple pattern-based NLP processor (production would use llama.cpp).
pub struct NlpProcessor;

impl Default for NlpProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl NlpProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Parse user input into an intent.
    pub fn parse(&self, text: &str) -> NlpIntent {
        let lower = text.to_lowercase();
        let (intent, entities) = if lower.contains("scan") || lower.contains("discover") {
            ("scan_network", vec![("action", "scan")])
        } else if lower.contains("send") && lower.contains("message") {
            ("send_message", vec![("action", "send")])
        } else if lower.contains("status") || lower.contains("health") {
            ("check_status", vec![("action", "status")])
        } else if lower.contains("connect") {
            ("connect_peer", vec![("action", "connect")])
        } else if lower.contains("encrypt") {
            ("encrypt_data", vec![("action", "encrypt")])
        } else if lower.contains("model") || lower.contains("inference") {
            ("run_inference", vec![("action", "infer")])
        } else {
            ("unknown", vec![])
        };

        NlpIntent {
            intent: intent.into(),
            entities: entities
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            confidence: if intent == "unknown" { 0.0 } else { 0.85 },
            raw_text: text.into(),
        }
    }
}

/// DAG-based workflow orchestrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub action: String,
    pub depends_on: Vec<String>,
    pub status: StepStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub struct WorkflowEngine {
    steps: Vec<WorkflowStep>,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(&mut self, id: String, action: String, depends_on: Vec<String>) {
        self.steps.push(WorkflowStep {
            id,
            action,
            depends_on,
            status: StepStatus::Pending,
        });
    }

    /// Get steps ready to execute (all dependencies completed).
    pub fn ready_steps(&self) -> Vec<&WorkflowStep> {
        self.steps
            .iter()
            .filter(|s| {
                s.status == StepStatus::Pending
                    && s.depends_on.iter().all(|dep| {
                        self.steps
                            .iter()
                            .any(|d| d.id == *dep && d.status == StepStatus::Completed)
                    })
            })
            .collect()
    }

    pub fn complete_step(&mut self, id: &str) {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == id) {
            step.status = StepStatus::Completed;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.steps.iter().all(|s| s.status == StepStatus::Completed)
    }
}

impl Default for WorkflowEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_detection() {
        let mut detector = AnomalyDetector::new(2.0, 20);
        // Feed normal data
        for i in 0..15 {
            assert!(detector.check("cpu", 50.0 + (i as f64 % 3.0)).is_none());
        }
        // Feed anomalous value
        let anomaly = detector.check("cpu", 200.0);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().deviation > 2.0);
    }

    #[test]
    fn test_nlp_parser() {
        let nlp = NlpProcessor::new();
        let intent = nlp.parse("Scan the network for devices");
        assert_eq!(intent.intent, "scan_network");
        assert!(intent.confidence > 0.5);

        let unknown = nlp.parse("What is the meaning of life?");
        assert_eq!(unknown.intent, "unknown");
    }

    #[test]
    fn test_workflow_dag() {
        let mut engine = WorkflowEngine::new();
        engine.add_step("fetch".into(), "fetch_data".into(), vec![]);
        engine.add_step("process".into(), "run_model".into(), vec!["fetch".into()]);
        engine.add_step("notify".into(), "send_alert".into(), vec!["process".into()]);

        assert_eq!(engine.ready_steps().len(), 1); // Only "fetch" has no deps
        assert_eq!(engine.ready_steps()[0].id, "fetch");

        engine.complete_step("fetch");
        assert_eq!(engine.ready_steps()[0].id, "process");

        engine.complete_step("process");
        engine.complete_step("notify");
        assert!(engine.is_complete());
    }
}
