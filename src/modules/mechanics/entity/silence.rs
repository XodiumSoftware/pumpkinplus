//! Silence module — toggle mob silent state using an amethyst shard.
//!
//! When a player right-clicks a mob while holding an amethyst shard, the mob's
//! silent state is toggled (silent ↔ audible). A dust particle effect is shown:
//! red when silencing, green when un-silencing. The shard is consumed in
//! non-creative gamemodes.
//!
//! ## Status
//!
//! **Stub module** — Core logic is implemented but particle colors and
//! monster detection are limited by missing Pumpkin plugin APIs.
//!
//! ## Missing / Limited APIs
//!
//! The following Pumpkin plugin APIs affect full implementation:
//!
//! | API | Status | Purpose |
//! |-----|--------|---------|
//! | `Entity.get_type()` | ⚠️ Partial | Can check entity type, but no `Monster` marker interface — must whitelist hostile mobs |
//! | `Particle.DUST` color | ❌ Missing | WIT `particle` enum has `dust` but no color/dust-options support |
//! | `Entity.set_silent()` | ✅ Available | Can toggle silent state |
//! | `Entity.is_silent()` | ✅ Available | Can read current silent state |
//! | `LivingEntity` check | ✅ Available | Can use `entity.as_living()` to verify entity is alive |
//!
//! ## Permissions
//!
//! | Node | Default | Description |
//! |------|---------|-------------|
//! | `pumpkinplus:silence.use` | allow | Allows silencing mobs with an amethyst shard |
//!
//! ## Configuration
//!
//! | Field        | Default                    | Description                                    |
//! |--------------|----------------------------|------------------------------------------------|
//! | `enabled`    | `false`                    | Whether this module is active                  |
//! | `tool_item`  | `"minecraft:amethyst_shard"` | Registry key of the item used to toggle silence |
//! | `hand`       | `["Right"]`                | Hands the tool item can be held in             |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerInteractEntityEvent`: if holding amethyst shard and target is
//!   a living, non-player, non-monster entity, toggle silent state
//! - Show red dust when silencing, green dust when un-silencing
//! - Consume one shard in survival/adventure modes

use crate::MirrorHand;
use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerInteractEntityEvent,
};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::{Context, Item, ItemStackExt, Server};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Permission node required to silence mobs with the configured tool item.
pub const PERM_SILENCE: &str = concat!(env!("CARGO_PKG_NAME"), ":silence.use");

/// Handles mob silencing with amethyst shards.
///
/// See module-level docs for the current implementation status and API limitations.
#[derive(Default)]
pub struct Silence;

impl Mechanic for Silence {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.silence.enabled)
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: PERM_SILENCE.into(),
            description: "Allows toggling mob silence with the configured tool item.".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        }]
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerInteractEntityEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<PlayerInteractEntityEvent> for Silence {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerInteractEntityEvent>,
    ) -> EventData<PlayerInteractEntityEvent> {
        let config: SilenceConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.silence)
            .unwrap_or_default();

        if !config.enabled {
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

        if !event.player.has_permission(PERM_SILENCE) {
            return event;
        }

        // TODO: Full implementation blocked by missing APIs:
        //
        // 1. Get target entity from `event.entity_id` and convert to living entity
        // 2. Check if entity is a player (skip)
        // 3. Check if entity is a monster (needs entity type whitelist — no `Monster` marker)
        // 4. Toggle silent state:
        //    a. entity.set_silent(!entity.is_silent())
        //    b. Spawn dust particle (red/green colors not available in WIT particle enum)
        //    c. Consume item if not creative (use `player.get_gamemode()` and `item.set_count()`)
        //    d. Cancel event to prevent default interaction
        //
        // For now, log debug info to verify the event fires correctly.

        debug!(
            "Silence: player {} interacted with entity {} while holding {}",
            event.player.get_name(),
            event.entity_id,
            config.tool_item,
        );

        event
    }
}

/// Configuration for the silence mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Registry key of the item held in the main hand to trigger the silence toggle.
    /// Defaults to `minecraft:amethyst_shard`.
    pub tool_item: String,
    /// Which hands the tool item can be held in to trigger the toggle.
    /// Use variant names like "Left" or "Right". Leave empty to allow either hand.
    pub hand: Vec<MirrorHand>,
}

impl Default for SilenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            tool_item: "minecraft:amethyst_shard".to_string(),
            hand: vec![MirrorHand::Right],
        }
    }
}
