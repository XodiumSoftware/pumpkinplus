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
//! Pumpkin plugin API and is registered below, but two capabilities are still
//! missing before a physical head item can actually be dropped:
//!
//! 1. **Spawning an item entity with a specific `ItemStack`** — `World::spawn-entity`
//!    can create an `item` entity, but the returned `entity` handle has no method
//!    to set the stack it carries (e.g. `set_item_stack` or similar).
//! 2. **Applying a player skin profile to a player head** — the `profile` data
//!    component is exposed in `pumpkin:plugin/data-components`, and
//!    `ItemStack::set-component` accepts raw serialized bytes, but there is no
//!    helper to serialize a `minecraft:profile` component from a `PlayerSkin` or
//!    username.
//!
//! Once upstream adds those two helpers, replace the debug logging below with:
//!
//! - Roll `skull_drop_chance`; abort if it fails.
//! - Create `ItemStack("minecraft:player_head", 1)`.
//! - Set the `minecraft:profile` component using the dying player's skin.
//! - Spawn or drop the item at the player's death location.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerDeathEvent};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Handles player head drops on death.
#[derive(Default)]
pub struct Head;

impl Mechanic for Head {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.head.enabled)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerDeathEvent, _>(Head, EventPriority::Normal, true)
            .expect("failed to register player death event handler");
    }
}

impl EventHandler<PlayerDeathEvent> for Head {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerDeathEvent>,
    ) -> EventData<PlayerDeathEvent> {
        let config = ConfigManager::get().map(|cm| cm.head).unwrap_or_default();
        let name = event.player.get_name();

        info!(
            "PlayerDeathEvent fired for {name}; skull_drop_chance is {}. \
             Head drop is not yet implemented because the API lacks \
             (1) a way to set an item entity's carried ItemStack and \
             (2) a helper to build the minecraft:profile data component.",
            config.skull_drop_chance
        );

        // TODO: Implement item drop once the Pumpkin plugin API exposes:
        //
        // 1. A method on spawned `item` entities to set their carried `ItemStack`.
        // 2. A helper to serialize a `minecraft:profile` component from a
        //    `PlayerSkin` (event.player.get_skin()) or username.
        //
        // Intended logic:
        // - Use a random roll against `config.skull_drop_chance`; return early if it fails.
        // - Create `ItemStack::new("minecraft:player_head", 1)`.
        // - Apply the dying player's profile/skin via `ItemStack::set_component`.
        // - Spawn or drop the item at `event.player.get_position()` in `event.player.get_world()`.

        event
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
