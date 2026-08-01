//! Spawn egg module — rare spawn egg drops on entity death.
//!
//! ## Configuration
//!
//! | Field                  | Default | Description                                         |
//! |------------------------|---------|-----------------------------------------------------|
//! | `enabled`              | `false` | Whether this module is active                       |
//! | `spawn_egg_drop_chance`| `0.001` | Chance (0.0 - 1.0) for an entity to drop its spawn egg |
//!
//! ## Notes
//!
//! This module is currently a stub. The Pumpkin plugin API does not yet expose
//! an `EntityDeathEvent` (or equivalent), so spawn egg drop mechanics cannot be
//! hooked until upstream support is added.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles rare spawn egg drops on entity death.
#[derive(Default)]
pub struct SpawnEgg;

impl Mechanic for SpawnEgg {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.spawn_egg.enabled)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when EntityDeathEvent (or equivalent) is available
        // in the Pumpkin plugin API. The intended logic is:
        //
        // 1. Listen for entity death events.
        // 2. Roll `spawn_egg_drop_chance`.
        // 3. If successful, map the entity type name to a spawn egg material.
        // 4. Add the spawn egg item to the event drops.
    }
}

/// Configuration for the spawn egg mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnEggConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Chance (0.0 - 1.0) for an entity to drop its spawn egg.
    pub spawn_egg_drop_chance: f64,
}

impl Default for SpawnEggConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            spawn_egg_drop_chance: 0.001,
        }
    }
}
