//! Tameable module — transfers pet ownership between players using a lead.
//!
//! When a player holding a lead right-clicks another player, and the source
//! player has a tamed, leashed pet that they own, ownership of the pet is
//! transferred to the target player and the pet is re-leashed to them.
//!
//! ## Status
//!
//! **Stub module** — The core logic is implemented but blocked by missing
//! Pumpkin plugin APIs. The event handler is registered and will log debug
//! information when triggered, but ownership transfer is not yet possible.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to complete this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `Entity.get_leashed_entities()` | Find entities currently leashed to a player |
//! | `Mob.set_owner(uuid)` | Transfer pet ownership |
//! | `Mob.set_leash_holder(entity)` | Re-leash the pet to the new owner |
//! | Generic tameable check | Currently only `wolf-data`/`cat-data` have `is-tamed`; need generic `Tameable` trait |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerInteractEntityEvent`: if source holds lead and target is a player,
//!   find source's leashed tamed pet, transfer ownership to target, re-leash to target.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerInteractEntityEvent,
};
use pumpkin_plugin_api::{Context, Item, ItemStackExt, Server};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Handles pet ownership transfer between players.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Tameable;

impl Mechanic for Tameable {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.tameable.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerInteractEntityEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<PlayerInteractEntityEvent> for Tameable {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerInteractEntityEvent>,
    ) -> EventData<PlayerInteractEntityEvent> {
        let config: TameableConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.tameable)
            .unwrap_or_default();

        if !config.enabled {
            return event;
        }

        // Only trigger on right-click (interact), not attack or interact-at
        // Note: `EntityInteractionAction` is not re-exported by `pumpkin-plugin-api`,
        // so we compare against the raw WIT discriminant values or rely on context.
        // For now, assume the event only fires for `Interact` (right-click) actions.

        // Check if source is holding a lead
        let Some(item) = event.player.get_item_in_hand(Hand::Right) else {
            return event;
        };

        if !item.is_item(Item::Lead) {
            return event;
        }

        // TODO: Implement once missing APIs are available:
        //
        // 1. Get the target entity from `event.entity_id`
        // 2. Check if target is a player (entity type check)
        // 3. Find source's leashed entities (needs `get_leashed_entities` API)
        // 4. For each leashed entity:
        //    a. Check if tamed and owned by source (needs generic tameable check or wolf/cat data)
        //    b. Transfer ownership to target (needs `set_owner` API)
        //    c. Re-leash to target (needs `set_leash_holder` API)
        //    d. Cancel event to prevent default interaction
        //
        // For now, log debug info to verify the event fires correctly.

        debug!(
            "Tameable: player {} interacted with entity {} while holding lead",
            event.player.get_name(),
            event.entity_id
        );

        event
    }
}

/// Configuration for the tameable mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TameableConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
