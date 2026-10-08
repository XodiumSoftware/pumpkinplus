//! XP module — convert experience to experience bottles.
//!
//! When a player right-clicks an enchanting table while holding a glass bottle,
//! 11 XP points are consumed and converted into an experience bottle.
//!
//! ## Status
//!
//! **Stub module** — Event handler and permission gate are wired; the actual
//! XP-to-bottle conversion is pending upstream Pumpkin plugin APIs.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to complete this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `Player.get_experience()` | Read total XP points (levels alone don't suffice for partial-level drains) |
//! | `Player.give_exp(amount)` | Deduct 11 XP points |
//! | `World.drop_item_at(pos, stack)` | Drop the bottle at the player's location when the inventory is full |
//! | `Inventory.add_or_drop(stack)` | Give the bottle, with full-inventory fallback |
//! | `BlockStateInfo.name == "enchanting_table"` | Detect the clicked block (available via `block_state_to_info`, but the event must expose `clicked_pos` reliably for air clicks) |
//!
//! ## Permissions
//!
//! | Node | Default | Description |
//! |------|---------|-------------|
//! | `pumpkinplus:xp.bottle` | allow | Allows converting XP into experience bottles |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerInteractEvent`: if clicked enchanting table with glass bottle,
//!   consume 11 XP and give experience bottle
//! - Costs 11 XP points (not levels)
//! - Bottle drops at player location if inventory is full

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Permission node required to convert XP into experience bottles.
pub const PERM_XP_BOTTLE: &str = concat!(env!("CARGO_PKG_NAME"), ":xp.bottle");

/// Cost in raw XP points (not levels) to produce one experience bottle.
pub const XP_PER_BOTTLE: u32 = 11;

/// Handles XP to bottle conversion.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Xp;

impl Mechanic for Xp {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.xp.enabled)
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: PERM_XP_BOTTLE.into(),
            description: "Allows converting XP into experience bottles.".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        }]
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerInteractEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<PlayerInteractEvent> for Xp {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerInteractEvent>,
    ) -> EventData<PlayerInteractEvent> {
        if !self.enabled() {
            return event;
        }

        if !event.player.has_permission(PERM_XP_BOTTLE) {
            return event;
        }

        // TODO: Implement the XP-to-bottle conversion once the following plugin
        // APIs exist:
        //   1. Check the clicked block is an enchanting table. `block_state_to_info`
        //      gives us the block name for comparison, but we also need the
        //      interact event to include `clicked_pos` for right-click-block.
        //   2. Confirm the player is holding a `glass_bottle` in the used hand.
        //      `Player.get_item_in_hand(Hand)` exists, so this part is ready.
        //   3. `Player.get_experience()` returning total XP points (not the
        //      level integer). Required because partial progress into the next
        //      level must count toward the 11-XP cost.
        //   4. `Player.give_exp(-11)` (negative to deduct) or a dedicated
        //      `take_exp(amount)`.
        //   5. Add an `experience_bottle` `ItemStack` to the player's inventory,
        //      dropping it at their feet via `World.drop_item_at()` if full.
        //
        // Reference (IllyriaPlus): player takes 11 XP -> receives bottle.

        event
    }
}

/// Configuration for the XP mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct XpConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
