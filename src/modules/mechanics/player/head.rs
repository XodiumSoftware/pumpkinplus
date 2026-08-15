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
//! This module is currently a stub. `PlayerDeathEvent` is now available in the
//! Pumpkin plugin API, but two capabilities are still missing:
//!
//! 1. **Dropping an item at a specific location** — there is no API to spawn or
//!    drop an `ItemStack` at the player's death position. `World::spawn-entity`
//!    can create an `item` entity, but there is no way to set its carried stack.
//! 2. **Applying a skin profile to a player head** — the `profile` data component
//!    is exposed, but `ItemStack::set-component` requires raw serialized component
//!    bytes and no helper is available to build a player profile component.
//!
//! Once upstream adds helpers for dropping an `ItemStack` and setting a player
//! profile component, the TODO below can be implemented.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles player head drops on death.
#[derive(Default)]
pub struct Head;

impl Mechanic for Head {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.head.enabled)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when the Pumpkin plugin API exposes:
        //
        // 1. A way to drop or spawn an ItemStack at a specific world position.
        //    `World::spawn-entity` can spawn an `item` entity, but there is no
        //    method to set the item it carries.
        // 2. A helper to set the `minecraft:profile` data component on a player
        //    head ItemStack from a player/skin handle.
        //
        // Intended logic once both are available:
        // - Register a `PlayerDeathEvent` handler.
        // - Roll `skull_drop_chance`; abort if it fails.
        // - Create a player head ItemStack ("minecraft:player_head").
        // - Apply the dying player's skin profile to the head item.
        // - Drop the head naturally at the player's death location.
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
