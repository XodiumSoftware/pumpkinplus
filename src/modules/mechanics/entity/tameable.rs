//! Tameable module — transfer tameable entity ownership via lead right-click.
//!
//! ## Configuration
//!
//! | Field       | Default | Description                                         |
//! |-------------|---------|-----------------------------------------------------|
//! | `enabled`   | `false` | Whether this module is active                       |
//!
//! ## Mechanics
//!
//! When a player right-clicks another player while holding a lead:
//! - If the source player has a leashed, tamed entity
//! - And the source player is the owner of that entity
//! - Ownership of the entity is transferred to the target player
//! - The entity is leashed to the new owner
//!
//! ## Notes
//!
//! This module is currently a stub. The following events are now available in
//! the Pumpkin plugin API:
//!
//! - `PlayerInteractEntityEvent`
//! - `PlayerInteractAtEntityEvent`
//! - `PlayerLeashEntityEvent` / `PlayerUnleashEntityEvent`
//! - `EntityTameEvent`
//!
//! However, the entity manipulation APIs required for ownership transfer are
//! still missing:
//!
//! 1. **Querying a player's leashed entity** — there is no way to get the entity
//!    currently leashed by a player.
//! 2. **Checking or modifying tameable state / ownership** — there is no API to
//!    read an entity's owner UUID or tame state, nor to change its owner.
//! 3. **Setting an entity's leash holder** — there is no API to leash an entity
//!    to a specific player or entity.
//!
//! Until these APIs are available, ownership transfer cannot be fully implemented.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles tameable entity ownership transfer.
#[derive(Default)]
pub struct Tameable;

impl Mechanic for Tameable {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.tameable.enabled)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when the Pumpkin plugin API exposes:
        //
        // 1. A way to retrieve the entity currently leashed by a player.
        // 2. A way to read and write an entity's owner UUID / tameable state.
        // 3. A way to set or transfer an entity's leash holder to another player.
        //
        // The relevant events (`PlayerInteractEntityEvent`, `PlayerLeashEntityEvent`,
        // `EntityTameEvent`) are already available, but the above entity state APIs
        // are still missing.
        //
        // Intended logic once all pieces are available:
        // - Listen for `PlayerInteractEntityEvent`.
        // - If the interaction is not a right-click on another player, return.
        // - If the source player is not holding a lead in their main hand, return.
        // - Query the source player's leashed entity.
        // - If the entity is not tamed or the source is not the owner, return.
        // - Transfer ownership to the target player.
        // - Set the target player as the new leash holder.
        // - Cancel the event to prevent default interaction.
    }
}

/// Configuration for the tameable ownership transfer module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TameableConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
