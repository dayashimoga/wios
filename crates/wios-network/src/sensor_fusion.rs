//! Sensor fusion — combine multiple positioning sources for accuracy.

use serde::{Deserialize, Serialize};

/// A position estimate from any source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionEstimate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub accuracy_m: f64,
    pub source: PositionSource,
    pub timestamp_ms: u64,
    pub confidence: f64, // 0.0–1.0
}

/// Position data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionSource {
    WifiRssi,
    BleRssi,
    Uwb,
    DeadReckoning,
    Gps,
    Manual,
}

/// Fused position result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusedPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub accuracy_m: f64,
    pub source_count: usize,
}

/// Fuse multiple position estimates using weighted average based on confidence and accuracy.
pub fn fuse_positions(estimates: &[PositionEstimate]) -> Option<FusedPosition> {
    if estimates.is_empty() { return None; }
    if estimates.len() == 1 {
        let e = &estimates[0];
        return Some(FusedPosition { x: e.x, y: e.y, z: e.z, accuracy_m: e.accuracy_m, source_count: 1 });
    }

    let mut total_weight = 0.0;
    let mut wx = 0.0;
    let mut wy = 0.0;
    let mut wz = 0.0;

    for est in estimates {
        // Weight = confidence / accuracy² (more accurate + more confident = higher weight)
        let weight = est.confidence / (est.accuracy_m * est.accuracy_m).max(0.01);
        wx += est.x * weight;
        wy += est.y * weight;
        wz += est.z * weight;
        total_weight += weight;
    }

    if total_weight < 1e-9 { return None; }

    let fused_accuracy = estimates.iter()
        .map(|e| e.accuracy_m * (1.0 - e.confidence * 0.5))
        .fold(f64::MAX, f64::min);

    Some(FusedPosition {
        x: wx / total_weight,
        y: wy / total_weight,
        z: wz / total_weight,
        accuracy_m: fused_accuracy * 0.8, // Fusion improves accuracy
        source_count: estimates.len(),
    })
}

/// Kalman-like simple smoothing for position updates.
pub struct PositionSmoother {
    pub position: Option<FusedPosition>,
    alpha: f64, // smoothing factor (0.0–1.0, higher = more responsive)
}

impl PositionSmoother {
    pub fn new(alpha: f64) -> Self {
        Self { position: None, alpha: alpha.clamp(0.01, 1.0) }
    }

    pub fn update(&mut self, new: FusedPosition) {
        match &self.position {
            None => self.position = Some(new),
            Some(prev) => {
                self.position = Some(FusedPosition {
                    x: prev.x + self.alpha * (new.x - prev.x),
                    y: prev.y + self.alpha * (new.y - prev.y),
                    z: prev.z + self.alpha * (new.z - prev.z),
                    accuracy_m: new.accuracy_m,
                    source_count: new.source_count,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn est(x: f64, y: f64, acc: f64, conf: f64, src: PositionSource) -> PositionEstimate {
        PositionEstimate { x, y, z: 0.0, accuracy_m: acc, source: src, timestamp_ms: 0, confidence: conf }
    }

    #[test]
    fn test_single_source() {
        let result = fuse_positions(&[est(5.0, 10.0, 2.0, 0.9, PositionSource::WifiRssi)]).unwrap();
        assert_eq!(result.x, 5.0);
        assert_eq!(result.source_count, 1);
    }

    #[test]
    fn test_fusion_prefers_accurate() {
        let estimates = vec![
            est(10.0, 10.0, 1.0, 0.9, PositionSource::Uwb),     // High accuracy
            est(20.0, 20.0, 10.0, 0.5, PositionSource::WifiRssi), // Low accuracy
        ];
        let result = fuse_positions(&estimates).unwrap();
        // Should be much closer to UWB (10,10) than WiFi (20,20)
        assert!(result.x < 15.0);
        assert_eq!(result.source_count, 2);
    }

    #[test]
    fn test_smoother() {
        let mut smoother = PositionSmoother::new(0.5);
        smoother.update(FusedPosition { x: 10.0, y: 10.0, z: 0.0, accuracy_m: 1.0, source_count: 1 });
        assert_eq!(smoother.position.as_ref().unwrap().x, 10.0);

        smoother.update(FusedPosition { x: 20.0, y: 20.0, z: 0.0, accuracy_m: 1.0, source_count: 1 });
        assert!((smoother.position.as_ref().unwrap().x - 15.0).abs() < 0.01);
    }
}
