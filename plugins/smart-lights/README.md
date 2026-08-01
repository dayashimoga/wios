# Smart Lights Plugin — Example WIOS Plugin

A sample plugin demonstrating the WIOS plugin system. Automates lighting
based on presence detection from the sensing layer.

## Plugin Manifest

```toml
[plugin]
id = "smart-lights"
name = "Smart Lights"
version = "1.0.0"
author = "WIOS Team"
description = "Automate lighting based on presence detection"
min_wios_version = "0.3.0"

[permissions]
sensing = ["read"]
automation = ["read", "write"]
network = ["publish"]

[triggers]
events = ["presence.detected", "presence.lost", "zone.enter", "zone.exit"]
```

## Implementation

```rust
use wios_api::plugin::{Plugin, PluginContext};

pub struct SmartLightsPlugin {
    zones: HashMap<String, LightConfig>,
}

#[derive(Clone)]
struct LightConfig {
    brightness: u8,
    color_temp: u16,
    auto_off_secs: u64,
}

impl Plugin for SmartLightsPlugin {
    fn name(&self) -> &str { "smart-lights" }
    fn version(&self) -> &str { "1.0.0" }

    fn on_start(&mut self, ctx: &PluginContext) -> Result<()> {
        // Subscribe to presence events
        ctx.subscribe("presence.detected")?;
        ctx.subscribe("presence.lost")?;
        Ok(())
    }

    fn on_event(&mut self, event: &str, data: &[u8]) -> Result<()> {
        match event {
            "presence.detected" => {
                let zone = String::from_utf8_lossy(data);
                if let Some(config) = self.zones.get(zone.as_ref()) {
                    // Turn on lights in zone
                    println!("Lights ON in {} (brightness: {}%)", zone, config.brightness);
                }
            }
            "presence.lost" => {
                let zone = String::from_utf8_lossy(data);
                println!("Lights OFF in {} (auto-off)", zone);
            }
            _ => {}
        }
        Ok(())
    }

    fn on_stop(&mut self) -> Result<()> {
        println!("Smart Lights plugin stopped");
        Ok(())
    }
}
```

## Configuration

Place in `~/.wios/plugins/smart-lights/config.toml`:

```toml
[zones.office]
brightness = 80
color_temp = 4000
auto_off_secs = 300

[zones.hallway]
brightness = 50
color_temp = 3000
auto_off_secs = 60
```
