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
//! This module is currently a stub. The Pumpkin plugin API does not yet expose:
//! - `PlayerInteractEntityEvent` (or equivalent) for right-clicking entities
//! - Methods to query a player's leashed entity
//! - Methods to check or modify entity tameable state / ownership
//! - Methods to set an entity's leash holder
//!
//! Until these APIs are available, ownership transfer cannot be implemented.

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
        // TODO: Implement when PlayerInteractEntityEvent (or equivalent) and
        // entity/leash/tameable APIs are available in the Pumpkin plugin API.
        //
        // The intended logic is:
        //
        // 1. Listen for player interact entity events.
        // 2. If the interaction is not a right-click on another player, return.
        // 3. If the source player is not holding a lead in their main hand, return.
        // 4. Query the source player's leashed entity.
        // 5. If the entity is not tamed or the source is not the owner, return.
        // 6. Transfer ownership to the target player.
        // 7. Set the target player as the new leash holder.
        // 8. Cancel the event to prevent default interaction.
    }
}

/// Configuration for the tameable ownership transfer module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TameableConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
