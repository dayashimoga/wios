//! Automation rule engine with sensor triggers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};

/// A condition that triggers a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// Sensor value exceeds threshold
    ThresholdAbove {
        sensor_id: String,
        value: f64,
    },
    ThresholdBelow {
        sensor_id: String,
        value: f64,
    },
    /// Asset enters/exits a zone
    ZoneEnter {
        zone_id: String,
    },
    ZoneExit {
        zone_id: String,
    },
    /// Time-based
    Schedule {
        cron: String,
    },
    /// Peer connects/disconnects
    PeerConnected {
        node_pattern: String,
    },
    PeerDisconnected {
        node_pattern: String,
    },
    /// Compound
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

/// Action to execute when conditions are met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    SendAlert { message: String, severity: String },
    PublishMessage { topic: String, payload: String },
    RunInference { model_id: String, input: String },
    SetConfig { key: String, value: String },
    ExecutePlugin { plugin_id: String, command: String },
    Log { message: String },
}

/// An automation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub condition: Condition,
    pub actions: Vec<Action>,
    pub cooldown_secs: u64,
    pub last_triggered: Option<u64>,
    pub trigger_count: u64,
}

/// Sensor event data for rule evaluation.
#[derive(Debug, Clone)]
pub struct SensorEvent {
    pub sensor_id: String,
    pub value: f64,
    pub timestamp: u64,
}

/// Rule engine managing automation rules.
pub struct RuleEngine {
    rules: HashMap<String, Rule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn remove_rule(&mut self, rule_id: &str) -> Option<Rule> {
        self.rules.remove(rule_id)
    }

    pub fn enable_rule(&mut self, rule_id: &str) -> WiosResult<()> {
        let rule = self.rules.get_mut(rule_id).ok_or(WiosError::NotFound {
            entity: "rule".into(),
            id: rule_id.into(),
        })?;
        rule.enabled = true;
        Ok(())
    }

    pub fn disable_rule(&mut self, rule_id: &str) -> WiosResult<()> {
        let rule = self.rules.get_mut(rule_id).ok_or(WiosError::NotFound {
            entity: "rule".into(),
            id: rule_id.into(),
        })?;
        rule.enabled = false;
        Ok(())
    }

    /// Evaluate a sensor event against all rules. Returns triggered actions.
    pub fn evaluate(&mut self, event: &SensorEvent) -> Vec<(String, Vec<Action>)> {
        let now = event.timestamp;
        let mut triggered = Vec::new();

        for rule in self.rules.values_mut() {
            if !rule.enabled {
                continue;
            }

            // Check cooldown
            if let Some(last) = rule.last_triggered {
                if now - last < rule.cooldown_secs {
                    continue;
                }
            }

            if Self::check_condition(&rule.condition, event) {
                rule.last_triggered = Some(now);
                rule.trigger_count += 1;
                triggered.push((rule.id.clone(), rule.actions.clone()));
            }
        }
        triggered
    }

    fn check_condition(condition: &Condition, event: &SensorEvent) -> bool {
        match condition {
            Condition::ThresholdAbove { sensor_id, value } => {
                event.sensor_id == *sensor_id && event.value > *value
            }
            Condition::ThresholdBelow { sensor_id, value } => {
                event.sensor_id == *sensor_id && event.value < *value
            }
            Condition::And(conditions) => {
                conditions.iter().all(|c| Self::check_condition(c, event))
            }
            Condition::Or(conditions) => conditions.iter().any(|c| Self::check_condition(c, event)),
            _ => false, // Zone/schedule/peer conditions need different event types
        }
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn active_rules(&self) -> Vec<&Rule> {
        self.rules.values().filter(|r| r.enabled).collect()
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rule(id: &str, sensor: &str, threshold: f64) -> Rule {
        Rule {
            id: id.into(),
            name: format!("Test rule {}", id),
            enabled: true,
            condition: Condition::ThresholdAbove {
                sensor_id: sensor.into(),
                value: threshold,
            },
            actions: vec![Action::Log {
                message: "triggered".into(),
            }],
            cooldown_secs: 0,
            last_triggered: None,
            trigger_count: 0,
        }
    }

    #[test]
    fn test_rule_trigger() {
        let mut engine = RuleEngine::new();
        engine.add_rule(make_rule("r1", "temp", 30.0));

        let event = SensorEvent {
            sensor_id: "temp".into(),
            value: 35.0,
            timestamp: 100,
        };
        let triggered = engine.evaluate(&event);
        assert_eq!(triggered.len(), 1);
        assert_eq!(triggered[0].0, "r1");
    }

    #[test]
    fn test_cooldown() {
        let mut engine = RuleEngine::new();
        let mut rule = make_rule("r1", "temp", 30.0);
        rule.cooldown_secs = 60;
        engine.add_rule(rule);

        let event1 = SensorEvent {
            sensor_id: "temp".into(),
            value: 35.0,
            timestamp: 100,
        };
        assert_eq!(engine.evaluate(&event1).len(), 1);

        let event2 = SensorEvent {
            sensor_id: "temp".into(),
            value: 35.0,
            timestamp: 110,
        };
        assert_eq!(engine.evaluate(&event2).len(), 0); // Still in cooldown

        let event3 = SensorEvent {
            sensor_id: "temp".into(),
            value: 35.0,
            timestamp: 200,
        };
        assert_eq!(engine.evaluate(&event3).len(), 1); // Cooldown expired
    }

    #[test]
    fn test_disabled_rule() {
        let mut engine = RuleEngine::new();
        engine.add_rule(make_rule("r1", "temp", 30.0));
        engine.disable_rule("r1").unwrap();

        let event = SensorEvent {
            sensor_id: "temp".into(),
            value: 35.0,
            timestamp: 100,
        };
        assert_eq!(engine.evaluate(&event).len(), 0);
    }
}
