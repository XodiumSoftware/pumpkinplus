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
//! This module is currently a stub. The relevant interaction and leash events
//! are now available in the Pumpkin plugin API:
//!
//! - `PlayerInteractEntityEvent`
//! - `PlayerInteractAtEntityEvent`
//! - `PlayerLeashEntityEvent` / `PlayerUnleashEntityEvent`
//! - `EntityTameEvent`
//!
//! `Player::get_item_in_hand` is also available, so we can detect when the
//! source player is holding a lead. However, the entity manipulation APIs
//! required for ownership transfer are **all** still missing:
//!
//! 1. **Querying a player's leashed entity** — there is no way to get the entity
//!    currently leashed by a player (`Player`/`World` has no `get_leashed_entity`
//!    or equivalent).
//! 2. **Reading or writing tameable state / ownership** — there is no API to
//!    read an entity's owner UUID or tame state, nor to change its owner.
//! 3. **Setting an entity's leash holder** — there is no API to leash an entity
//!    to a specific player or entity.
//!
//! `EntityTameEvent` only reports a taming event as it happens; it does not
//! provide a way to inspect or modify an entity's owner after the fact.
//!
//! Until those three APIs are available, ownership transfer cannot be
//! implemented. The `PlayerInteractEntityEvent` handler is registered below so
//! the module can log when the intended interaction occurs.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerInteractEntityEvent,
};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Handles tameable entity ownership transfer.
#[derive(Default)]
pub struct Tameable;

impl Mechanic for Tameable {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.tameable.enabled)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerInteractEntityEvent, _>(
                Tameable,
                EventPriority::Normal,
                true,
            )
            .expect("failed to register player interact entity event handler");
    }
}

impl EventHandler<PlayerInteractEntityEvent> for Tameable {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerInteractEntityEvent>,
    ) -> EventData<PlayerInteractEntityEvent> {
        let source_name = event.player.get_name();
        let held = event.player.get_item_in_hand(Hand::Right);
        let holding_lead = held
            .as_ref()
            .is_some_and(|stack| stack.get_registry_key() == "minecraft:lead");

        info!(
            "PlayerInteractEntityEvent fired: source={}, target_entity_id={}, action={:?}, \
             holding_lead={}. Ownership transfer is not implemented because the API lacks \
             (1) a way to get a player's leashed entity, \
             (2) a way to read/write an entity's owner/tameable state, and \
             (3) a way to set an entity's leash holder.",
            source_name, event.entity_id, event.action, holding_lead
        );

        // TODO: Implement ownership transfer once the Pumpkin plugin API exposes
        // **all three** of the following (none are currently available):
        //
        // 1. A method to retrieve the entity currently leashed by a player.
        // 2. A method to read and write an entity's owner UUID / tameable state.
        // 3. A method to set or transfer an entity's leash holder to another player.
        //
        // Intended logic once all pieces are available:
        // - Listen for `PlayerInteractEntityEvent`.
        // - If the action is not a right-click (`interact`), return.
        // - If the target entity is not another player, return.
        // - If the source player is not holding a lead (`minecraft:lead`) in their main hand, return.
        // - Query the source player's leashed entity.
        // - If the entity is not tamed or the source is not the owner, return.
        // - Transfer ownership to the target player.
        // - Set the target player as the new leash holder.
        // - Cancel the event to prevent default interaction.

        event
    }
}

/// Configuration for the tameable ownership transfer module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TameableConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
