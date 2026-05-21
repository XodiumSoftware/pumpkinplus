//! Head module — player head drops on death.
//!
//! ## Configuration
//!
//! | Field              | Default | Description                                         |
//! |--------------------|---------|-----------------------------------------------------|
//! | `enabled`          | `false` | Whether this module is active                       |
//! | `skull_drop_chance`| `0.01`  | Chance (0.0 - 1.0) for a player's head to drop    |
//!
//! ## Mechanics
//!
//! When a player dies, there is a chance their head (with their skin profile)
//! is dropped at their death location.
//!
//! ## Notes
//!
//! This module is currently a stub. The Pumpkin plugin API does not yet expose
//! a `PlayerDeathEvent` (or equivalent), so player head drop mechanics cannot be
//! hooked until upstream support is added. Additionally, creating player head
//! items with skin profiles requires item data component APIs that may not be
//! available in the WASM plugin environment.

use crate::config::ConfigManager;
use crate::module::Module;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles player head drops on death.
#[derive(Default)]
pub struct Head;

impl Module for Head {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<HeadConfig>().enabled)
            .unwrap_or(false)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when PlayerDeathEvent (or equivalent) is available
        // in the Pumpkin plugin API. The intended logic is:
        //
        // 1. Listen for player death events.
        // 2. Roll `skull_drop_chance`.
        // 3. If successful, create a player head ItemStack.
        // 4. Apply the player's skin profile to the head item.
        // 5. Drop the item naturally at the player's death location.
    }
}

/// Configuration for the player head drop module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Chance (0.0 - 1.0) for a player's head to drop on death.
    pub skull_drop_chance: f64,
}

impl Default for HeadConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            skull_drop_chance: 0.01,
        }
    }
}
