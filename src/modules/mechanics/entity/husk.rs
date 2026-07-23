//! Husk module — custom husk sand drops with camel rider bonus.
//!
//! ## Configuration
//!
//! | Field                        | Default | Description                                         |
//! |------------------------------|---------|-----------------------------------------------------|
//! | `enabled`                    | `false` | Whether this module is active                       |
//! | `sand_drop_chance`           | `1.0`   | Chance (0.0 - 1.0) for a husk to drop sand          |
//! | `sand_base_min`              | `0`     | Minimum base amount of sand dropped                 |
//! | `sand_base_max`              | `2`     | Maximum base amount of sand dropped                 |
//! | `sand_looting_bonus`         | `1`     | Extra sand granted per level of Looting             |
//! | `camel_rider_sand_base_max`  | `3`     | Maximum base sand for husks riding a camel          |
//! | `camel_rider_looting_bonus`  | `2`     | Extra sand per Looting level for camel riders       |
//!
//! ## Notes
//!
//! This module is currently a stub. The Pumpkin plugin API does not yet expose
//! an `EntityDeathEvent` (or equivalent), so husk drop mechanics cannot be hooked
//! until upstream support is added.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles husk death drop mechanics.
#[derive(Default)]
pub struct Husk;

impl Mechanic for Husk {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.get_config::<HuskConfig>().enabled)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when EntityDeathEvent (or equivalent) is available
        // in the Pumpkin plugin API. The intended logic is:
        //
        // 1. Listen for entity death events.
        // 2. If the entity is not a Husk, return.
        // 3. If the drop chance roll fails, return.
        // 4. Determine if the husk is riding a Camel (`isCamelHusk`).
        // 5. Determine the killer and their Looting enchantment level.
        // 6. Calculate min and max sand amount based on config values and whether it's a camel rider.
        // 7. Roll the amount: `Random.nextInt(minAmount, maxAmount + 1)`.
        // 8. If amount > 0, add `Material::Sand` to the event drops.
    }
}

/// Configuration for the husk mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuskConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Chance (0.0 - 1.0) for a husk to drop sand.
    pub sand_drop_chance: f64,
    /// Minimum base amount of sand dropped.
    pub sand_base_min: u32,
    /// Maximum base amount of sand dropped (non-camel).
    pub sand_base_max: u32,
    /// Extra sand granted per level of Looting (non-camel).
    pub sand_looting_bonus: u32,
    /// Maximum base amount of sand dropped when the husk is riding a camel.
    pub camel_rider_sand_base_max: u32,
    /// Extra sand per level of Looting when the husk is riding a camel.
    pub camel_rider_looting_bonus: u32,
}

impl Default for HuskConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sand_drop_chance: 1.0,
            sand_base_min: 0,
            sand_base_max: 2,
            sand_looting_bonus: 1,
            camel_rider_sand_base_max: 3,
            camel_rider_looting_bonus: 2,
        }
    }
}
