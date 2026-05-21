//! Bat module - custom bat drops (phantom membrane).
//!
//! ## Configuration
//!
//! | Field                    | Default | Description                                         |
//! |--------------------------|---------|-----------------------------------------------------|
//! | `enabled`                | `false` | Whether this module is active                       |
//! | `drop_chance`            | `1.0`   | Chance (0.0 - 1.0) for a bat to drop membrane     |
//! | `base_min`               | `0`     | Minimum base amount of membrane dropped             |
//! | `base_max`               | `1`     | Maximum base amount of membrane dropped             |
//! | `looting_bonus_per_level`| `1`     | Extra membrane granted per level of Looting         |
//!
//! ## Notes
//!
//! This module is currently a stub. The Pumpkin plugin API does not yet expose
//! an `EntityDeathEvent` (or equivalent), so bat drop mechanics cannot be hooked
//! until upstream support is added.

use crate::{config::ConfigManager, mechanics::mechanic::Mechanic};
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles bat drop mechanics.
#[derive(Default)]
pub struct Bat;

impl Mechanic for Bat {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<BatConfig>().enabled)
            .unwrap_or(false)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when EntityDeathEvent (or equivalent) is available
        // in the Pumpkin plugin API. The intended logic is:
        //
        // 1. Listen for entity death events.
        // 2. Check if the gamerule `spawn_phantoms` is true; if false, return.
        // 3. If the entity is a bat and the drop chance succeeds:
        //    - Determine the killer.
        //    - Check the killer's main-hand weapon for a Looting enchantment.
        //    - Calculate drops: base + (looting_level * config.looting_bonus_per_level).
        //    - Add `Material::PhantomMembrane` to the event drops.
    }
}

/// Configuration for the bat mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Chance (0.0 - 1.0) for a bat to drop phantom membrane.
    pub drop_chance: f64,
    /// Minimum base amount of phantom membrane dropped.
    pub base_min: u32,
    /// Maximum base amount of phantom membrane dropped.
    pub base_max: u32,
    /// Extra membrane granted per level of Looting.
    pub looting_bonus_per_level: u32,
}

impl Default for BatConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            drop_chance: 1.0,
            base_min: 0,
            base_max: 1,
            looting_bonus_per_level: 1,
        }
    }
}
