//! Asset tracker — BLE/UWB tag management and tracking.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};

/// A trackable asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub tag_id: String,
    pub tag_type: TagType,
    pub last_position: Option<(f64, f64, f64)>,
    pub last_rssi: Option<i8>,
    pub last_seen: u64,
    pub zone: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Tag technology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagType {
    Ble,
    Uwb,
    WifiRtt,
    Passive,
}

/// Geofence zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoZone {
    pub id: String,
    pub name: String,
    pub center: (f64, f64),
    pub radius_m: f64,
}

/// Asset tracking manager.
pub struct AssetTracker {
    assets: HashMap<String, Asset>,
    zones: Vec<GeoZone>,
}

impl AssetTracker {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            zones: Vec::new(),
        }
    }

    /// Register an asset for tracking.
    pub fn register(&mut self, asset: Asset) {
        self.assets.insert(asset.id.clone(), asset);
    }

    /// Update asset position from a beacon reading.
    pub fn update_position(
        &mut self,
        asset_id: &str,
        x: f64,
        y: f64,
        z: f64,
        rssi: i8,
    ) -> WiosResult<()> {
        let asset = self.assets.get_mut(asset_id).ok_or(WiosError::NotFound {
            entity: "asset".into(),
            id: asset_id.into(),
        })?;
        asset.last_position = Some((x, y, z));
        asset.last_rssi = Some(rssi);
        asset.last_seen = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Check zone membership
        asset.zone = self
            .zones
            .iter()
            .find(|z| {
                let dx = z.center.0 - x;
                let dy = z.center.1 - y;
                (dx * dx + dy * dy).sqrt() <= z.radius_m
            })
            .map(|z| z.id.clone());
        Ok(())
    }

    /// Add a geofence zone.
    pub fn add_zone(&mut self, zone: GeoZone) {
        self.zones.push(zone);
    }

    /// Get all assets in a specific zone.
    pub fn assets_in_zone(&self, zone_id: &str) -> Vec<&Asset> {
        self.assets
            .values()
            .filter(|a| a.zone.as_deref() == Some(zone_id))
            .collect()
    }

    /// Get assets not seen for longer than `timeout_secs`.
    pub fn stale_assets(&self, timeout_secs: u64) -> Vec<&Asset> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.assets
            .values()
            .filter(|a| now - a.last_seen > timeout_secs)
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<&Asset> {
        self.assets.get(id)
    }
    pub fn count(&self) -> usize {
        self.assets.len()
    }
}

impl Default for AssetTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_tracking() {
        let mut tracker = AssetTracker::new();
        tracker.add_zone(GeoZone {
            id: "office".into(),
            name: "Office".into(),
            center: (10.0, 10.0),
            radius_m: 5.0,
        });

        tracker.register(Asset {
            id: "laptop1".into(),
            name: "Dev Laptop".into(),
            tag_id: "ble-001".into(),
            tag_type: TagType::Ble,
            last_position: None,
            last_rssi: None,
            last_seen: 0,
            zone: None,
            metadata: HashMap::new(),
        });

        tracker
            .update_position("laptop1", 10.0, 10.0, 0.0, -45)
            .unwrap();
        let asset = tracker.get("laptop1").unwrap();
        assert_eq!(asset.zone.as_deref(), Some("office"));
        assert_eq!(asset.last_rssi, Some(-45));
    }

    #[test]
    fn test_zone_query() {
        let mut tracker = AssetTracker::new();
        tracker.add_zone(GeoZone {
            id: "z1".into(),
            name: "Zone 1".into(),
            center: (0.0, 0.0),
            radius_m: 5.0,
        });

        tracker.register(Asset {
            id: "a1".into(),
            name: "A1".into(),
            tag_id: "t1".into(),
            tag_type: TagType::Ble,
            last_position: None,
            last_rssi: None,
            last_seen: 0,
            zone: None,
            metadata: HashMap::new(),
        });
        tracker.update_position("a1", 1.0, 1.0, 0.0, -50).unwrap();
        assert_eq!(tracker.assets_in_zone("z1").len(), 1);
        assert_eq!(tracker.assets_in_zone("z2").len(), 0);
    }
}
