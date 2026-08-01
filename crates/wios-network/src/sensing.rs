//! WiFi/BLE RSSI-based positioning and RF heatmap generation.

use serde::{Deserialize, Serialize};

/// An RSSI measurement from a beacon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssiReading {
    /// Beacon/AP identifier (MAC or UUID).
    pub beacon_id: String,
    /// Signal type.
    pub signal_type: SignalType,
    /// RSSI value in dBm (typically -30 to -100).
    pub rssi_dbm: f64,
    /// Timestamp of reading.
    pub timestamp_ms: u64,
    /// Known beacon position (if available).
    pub beacon_position: Option<Position>,
}

/// Signal type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    Wifi24Ghz,
    Wifi5Ghz,
    Wifi6Ghz,
    BleLowEnergy,
    BleClassic,
}

/// 2D position.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// RSSI-based position estimator using weighted trilateration.
pub struct PositionEstimator {
    /// Path loss exponent (2.0 = free space, 2.5-4.0 = indoor).
    path_loss_exponent: f64,
    /// Reference RSSI at 1 meter.
    reference_rssi: f64,
}

impl PositionEstimator {
    pub fn new(path_loss_exponent: f64, reference_rssi: f64) -> Self {
        Self {
            path_loss_exponent,
            reference_rssi,
        }
    }

    /// Estimate distance from RSSI using log-distance path loss model.
    pub fn rssi_to_distance(&self, rssi_dbm: f64) -> f64 {
        10f64.powf((self.reference_rssi - rssi_dbm) / (10.0 * self.path_loss_exponent))
    }

    /// Estimate position from multiple beacon readings via weighted centroid.
    pub fn estimate_position(&self, readings: &[RssiReading]) -> Option<Position> {
        let positioned: Vec<_> = readings
            .iter()
            .filter_map(|r| {
                r.beacon_position.map(|pos| {
                    let dist = self.rssi_to_distance(r.rssi_dbm);
                    let weight = 1.0 / (dist * dist).max(0.01);
                    (pos, weight)
                })
            })
            .collect();

        if positioned.len() < 3 {
            return None;
        }

        let total_weight: f64 = positioned.iter().map(|(_, w)| w).sum();
        let x = positioned.iter().map(|(p, w)| p.x * w).sum::<f64>() / total_weight;
        let y = positioned.iter().map(|(p, w)| p.y * w).sum::<f64>() / total_weight;

        Some(Position::new(x, y))
    }
}

impl Default for PositionEstimator {
    fn default() -> Self {
        Self::new(3.0, -40.0) // Typical indoor values
    }
}

/// RF heatmap generator.
pub struct RfHeatmap {
    /// Grid resolution in units.
    pub resolution: f64,
    pub width: usize,
    pub height: usize,
    /// Grid cells: [y][x] = signal strength.
    pub grid: Vec<Vec<f64>>,
}

impl RfHeatmap {
    /// Create a new heatmap grid.
    pub fn new(width: usize, height: usize, resolution: f64) -> Self {
        Self {
            resolution,
            width,
            height,
            grid: vec![vec![-100.0; width]; height],
        }
    }

    /// Update the heatmap from beacon readings using path-loss model.
    /// Each beacon's signal attenuates by ~20*log10(distance) with distance.
    pub fn update(&mut self, beacons: &[(Position, f64)]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let px = x as f64 * self.resolution;
                let py = y as f64 * self.resolution;
                let point = Position::new(px, py);

                let mut best_rssi = -100.0;
                for (beacon_pos, rssi) in beacons {
                    let dist = point.distance_to(beacon_pos).max(0.1);
                    // Path-loss model: signal drops with 20*log10(dist)
                    let attenuated = rssi - 20.0 * dist.log10();
                    if attenuated > best_rssi {
                        best_rssi = attenuated;
                    }
                }
                self.grid[y][x] = best_rssi;
            }
        }
    }

    /// Get the signal strength at a grid position.
    pub fn signal_at(&self, x: usize, y: usize) -> f64 {
        if x < self.width && y < self.height {
            self.grid[y][x]
        } else {
            -100.0
        }
    }

    /// Find the position with strongest signal.
    pub fn strongest_signal(&self) -> (usize, usize, f64) {
        let mut best = (0, 0, f64::NEG_INFINITY);
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] > best.2 {
                    best = (x, y, self.grid[y][x]);
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rssi_to_distance() {
        let est = PositionEstimator::default();
        let d1 = est.rssi_to_distance(-40.0); // at reference = ~1m
        let d2 = est.rssi_to_distance(-70.0); // farther
        assert!(d1 < d2);
        assert!((d1 - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_position_estimation() {
        let est = PositionEstimator::default();
        let readings = vec![
            RssiReading {
                beacon_id: "a".into(),
                signal_type: SignalType::Wifi24Ghz,
                rssi_dbm: -40.0,
                timestamp_ms: 0,
                beacon_position: Some(Position::new(0.0, 0.0)),
            },
            RssiReading {
                beacon_id: "b".into(),
                signal_type: SignalType::Wifi24Ghz,
                rssi_dbm: -40.0,
                timestamp_ms: 0,
                beacon_position: Some(Position::new(10.0, 0.0)),
            },
            RssiReading {
                beacon_id: "c".into(),
                signal_type: SignalType::Wifi24Ghz,
                rssi_dbm: -40.0,
                timestamp_ms: 0,
                beacon_position: Some(Position::new(5.0, 10.0)),
            },
        ];
        let pos = est.estimate_position(&readings).unwrap();
        assert!((pos.x - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_heatmap() {
        let mut heatmap = RfHeatmap::new(10, 10, 1.0);
        let beacons = vec![
            (Position::new(5.0, 5.0), -30.0), // Strong beacon at center
        ];
        heatmap.update(&beacons);

        // Center should be strongest
        let (x, y, _) = heatmap.strongest_signal();
        assert_eq!(x, 5);
        assert_eq!(y, 5);
        // Corner should be weaker
        assert!(heatmap.signal_at(0, 0) < heatmap.signal_at(5, 5));
    }
}
