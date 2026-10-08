//! Head module — rare player head drops on death.
//!
//! When a player dies, there is a small chance (1%) for their head to drop
//! as a player skull item with their profile texture.
//!
//! ## Status
//!
//! **Stub module** — Event handler and permission gate are wired; the actual
//! head-drop is pending upstream Pumpkin plugin APIs.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to complete this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `World.drop_item_at(pos, stack)` | Spawn the head item at the death location |
//! | `ItemStack.with_profile(profile)` | Set the player profile on the skull item so the head renders the dead player's skin |
//! | `Player.profile()` / `GameProfile` | Read the player's uuid + skin texture properties to build the profile |
//! | Random source or `rand` host function | Roll the 1% drop chance |
//!
//! ## Permissions
//!
//! | Node | Default | Description |
//! |------|---------|-------------|
//! | `pumpkinplus:head.drop` | allow | Allows this player's head to drop on death |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerDeathEvent`: 1% chance to drop player head with skin texture
//! - Uses `ItemStack.of(Material.PLAYER_HEAD)` with `ResolvableProfile` data component

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerDeathEvent};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Permission node required for a player's head to drop on death.
pub const PERM_HEAD_DROP: &str = concat!(env!("CARGO_PKG_NAME"), ":head.drop");

/// Handles player head drops on death.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Head;

impl Mechanic for Head {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.head.enabled)
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: PERM_HEAD_DROP.into(),
            description: "Allows this player's head to drop on death.".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        }]
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerDeathEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<PlayerDeathEvent> for Head {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerDeathEvent>,
    ) -> EventData<PlayerDeathEvent> {
        if !self.enabled() {
            return event;
        }

        if !event.player.has_permission(PERM_HEAD_DROP) {
            return event;
        }

        // TODO: Implement the actual head drop once the following plugin APIs
        // exist:
        //   1. Random source (for the 1% roll). `rand` does not run under WASI
        //      without a host-provided entropy source; a `Context.random_f32()`
        //      host call would suffice.
        //   2. `Player.game_profile()` (or equivalent) returning the player's
        //      UUID + skin properties so we can build a `Profile` component.
        //   3. `ItemStack.with_profile(profile)` (or a profile component setter)
        //      so the skull renders the dead player's skin.
        //   4. `World.drop_item_at(pos, stack)` to spawn the head at the death
        //      location (or an `add_drops` field on `PlayerDeathEvent`).
        //
        // Reference (IllyriaPlus):
        //   if rand.nextDouble() < 0.01 {
        //       let head = ItemStack(Material.PLAYER_HEAD);
        //       head.setData(ResolvableProfile(player.profile()));
        //       world.dropItem(player.location(), head);
        //   }

        event
    }
}

/// Configuration for the head mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeadConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
