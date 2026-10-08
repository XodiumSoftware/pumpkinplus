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
//! ## Permissions
//!
//! | Node | Default | Description |
//! |------|---------|-------------|
//! | `pumpkinplus:tameable.transfer` | allow | Allows transferring pet ownership with a lead |
//!
//! ## Configuration
//!
//! | Field        | Default    | Description                                          |
//! |--------------|------------|------------------------------------------------------|
//! | `enabled`    | `false`    | Whether this module is active                        |
//! | `tool_item`  | `"minecraft:lead"` | Registry key of the item used to transfer ownership |
//! | `hand`       | `["Right"]` | Hands the tool item can be held in                   |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerInteractEntityEvent`: if source holds lead and target is a player,
//!   find source's leashed tamed pet, transfer ownership to target, re-leash to target.

use crate::MirrorHand;
use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerInteractEntityEvent,
};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::wit::pumpkin::plugin::event::EntityInteractionAction;
use pumpkin_plugin_api::{Context, Item, ItemStackExt, Server};
use serde::{Deserialize, Serialize};

/// Permission node required to transfer pet ownership with a lead.
pub const PERM_TAMEABLE_TRANSFER: &str = concat!(env!("CARGO_PKG_NAME"), ":tameable.transfer");

/// Handles pet ownership transfer between players.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Tameable;

impl Mechanic for Tameable {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.tameable.enabled)
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: PERM_TAMEABLE_TRANSFER.into(),
            description: "Allows transferring pet ownership with a lead.".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        }]
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

        if event.action != EntityInteractionAction::Interact {
            return event;
        }

        let Some(tool) = Item::from_registry_key(&config.tool_item) else {
            return event;
        };

        let holding_tool = [MirrorHand::Right, MirrorHand::Left]
            .into_iter()
            .filter(|h| h.matches_config(&config.hand))
            .any(|h| {
                event
                    .player
                    .get_item_in_hand(h.into())
                    .is_some_and(|stack| stack.is_item(tool))
            });

        if !holding_tool {
            return event;
        }

        if !event.player.has_permission(PERM_TAMEABLE_TRANSFER) {
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

        event
    }
}

/// Configuration for the tameable mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TameableConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Registry key of the item held to trigger the ownership transfer.
    /// Defaults to `minecraft:lead`.
    pub tool_item: String,
    /// Which hands the tool item can be held in to trigger the transfer.
    /// Use variant names like "Left" or "Right". Leave empty to allow either hand.
    pub hand: Vec<MirrorHand>,
}

impl Default for TameableConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            tool_item: "minecraft:lead".to_string(),
            hand: vec![MirrorHand::Right],
        }
    }
}
