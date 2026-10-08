//! Openable module — synchronizes double doors and adds a knock mechanic to
//! doors, trapdoors, and fence gates.
//!
//! ## Double door sync
//!
//! When a player right-clicks a door that is part of a double door setup, the
//! adjacent door is toggled to match, so both open and close together. This
//! sync only applies to doors; trapdoors and fence gates are not synced.
//!
//! ## Knock
//!
//! When a player left-clicks an openable block (door, trapdoor, or fence gate)
//! while sneaking with an empty main hand, the interaction is cancelled so the
//! block is not damaged.
//!
//! > **Note:** The knock sound is temporarily disabled due to a WASM ABI
//! > compatibility issue with the `Sound` enum. It will be re-enabled when the
//! > upstream Pumpkin plugin API fixes the issue.
//!
//! ## Configuration
//!
//! | Field                  | Default                       | Description                                              |
//! |------------------------|-------------------------------|----------------------------------------------------------|
//! | `enabled`              | `false`                       | Whether this module is active                                    |
//! | `sync_enabled`         | `false`                       | Whether double door sync is enabled                              |
//! | `sync_gamemodes`       | `["Survival", "Adventure"]`   | Gamemodes that trigger sync                                      |
//! | `sync_actions`         | `["RightClickBlock"]`         | Actions that trigger sync                                        |
//! | `sync_sneaking_required` | `false`                     | Whether the player must be sneaking to trigger sync              |
//! | `knock_enabled`        | `false`                       | Whether sneaking left-click knock is enabled                    |
//! | `knock_gamemodes`      | `["Survival", "Adventure"]`   | Gamemodes allowed to knock                                       |
//! | `knock_actions`        | `["LeftClickBlock"]`          | Actions that trigger knock                                       |
//! | `knock_sneaking_required` | `true`                     | Whether the player must be sneaking to knock                     |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::utils::block::{is_door, is_openable, toggle_open_property};
use crate::{GameMode, InteractAction};
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::world::{BlockFlags, BlockPos, World, block_state_to_info};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles openable block synchronization and door knocking.
#[derive(Default)]
pub struct Openable;

impl Mechanic for Openable {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.openable.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerInteractEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<PlayerInteractEvent> for Openable {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerInteractEvent>,
    ) -> EventData<PlayerInteractEvent> {
        if !self.enabled() {
            return event;
        }

        let config: OpenableConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.openable)
            .unwrap_or_default();

        let action = InteractAction::from(event.action);

        if config.knock_enabled && action.matches_config(&config.knock_actions) {
            return handle_knock(event, &config);
        }

        if config.sync_enabled && action.matches_config(&config.sync_actions) {
            return handle_sync(event, &config);
        }

        event
    }
}

/// Handles the double-door sync interaction.
fn handle_sync(
    mut event: EventData<PlayerInteractEvent>,
    config: &OpenableConfig,
) -> EventData<PlayerInteractEvent> {
    let gamemode = GameMode::from(event.player.get_gamemode());
    if !gamemode.matches_config(&config.sync_gamemodes) {
        return event;
    }

    if config.sync_sneaking_required && !event.player.as_entity().is_sneaking() {
        return event;
    }

    let Some(clicked_pos) = event.clicked_pos else {
        return event;
    };

    let world = event.player.get_world();

    let clicked_state_id = world.get_block_state_id(clicked_pos);
    let Some(clicked_info) = block_state_to_info(clicked_state_id) else {
        return event;
    };

    if !is_door(&clicked_info) {
        return event;
    }

    let Some(adjacent_pos) = find_adjacent_door(&world, clicked_pos) else {
        return event;
    };

    let adjacent_state_id = world.get_block_state_id(adjacent_pos);
    let Some(adjacent_info) = block_state_to_info(adjacent_state_id) else {
        return event;
    };

    if !is_door(&adjacent_info) {
        return event;
    }

    let Some(new_clicked_id) = toggle_open_property(&clicked_info) else {
        return event;
    };
    let Some(new_adjacent_id) = toggle_open_property(&adjacent_info) else {
        return event;
    };

    event.cancelled = true;

    let flags = BlockFlags::NOTIFY_NEIGHBORS | BlockFlags::NOTIFY_LISTENERS;

    world.set_block_state(clicked_pos, new_clicked_id, flags);
    world.set_block_state(adjacent_pos, new_adjacent_id, flags);

    event
}

/// Handles the door-knock interaction.
fn handle_knock(
    mut event: EventData<PlayerInteractEvent>,
    config: &OpenableConfig,
) -> EventData<PlayerInteractEvent> {
    let gamemode = GameMode::from(event.player.get_gamemode());
    if !gamemode.matches_config(&config.knock_gamemodes) {
        return event;
    }

    let is_sneaking = event.player.as_entity().is_sneaking();
    if config.knock_sneaking_required && !is_sneaking {
        return event;
    }

    let hand_item = event.player.get_item_in_hand(Hand::Right);
    if hand_item.is_some() {
        return event;
    }

    let Some(clicked_pos) = event.clicked_pos else {
        return event;
    };

    let world = event.player.get_world();

    let clicked_state_id = world.get_block_state_id(clicked_pos);
    let Some(clicked_info) = block_state_to_info(clicked_state_id) else {
        return event;
    };

    if !is_openable(&clicked_info) {
        return event;
    }

    event.cancelled = true;

    // TODO: play_sound is disabled due to a WASM ABI compatibility issue
    // with the Sound enum (LegacySyncReentry error). Re-enable when fixed.
    // world.play_sound(
    //     KNOCK_SOUND,
    //     wit::pumpkin::plugin::sounds::SoundCategory::Blocks,
    //     block_center(clicked_pos),
    //     1.0,
    //     1.0,
    // );

    event
}

/// Searches the four horizontal neighbors for another door block.
fn find_adjacent_door(world: &World, pos: BlockPos) -> Option<BlockPos> {
    let neighbors = [
        BlockPos {
            x: pos.x + 1,
            y: pos.y,
            z: pos.z,
        },
        BlockPos {
            x: pos.x - 1,
            y: pos.y,
            z: pos.z,
        },
        BlockPos {
            x: pos.x,
            y: pos.y,
            z: pos.z + 1,
        },
        BlockPos {
            x: pos.x,
            y: pos.y,
            z: pos.z - 1,
        },
    ];

    for neighbor in &neighbors {
        let state_id = world.get_block_state_id(*neighbor);
        if block_state_to_info(state_id).is_some_and(|info| is_door(&info)) {
            return Some(*neighbor);
        }
    }

    None
}

/// Configuration for the openable mechanics module.
// Config structs intentionally use multiple feature toggles; a state machine
// does not apply here since fields are deserialized independently.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenableConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Whether double door sync is enabled.
    pub sync_enabled: bool,
    /// List of gamemodes allowed to trigger door sync. Use variant names like "Survival", "Creative", etc. Leave empty to allow all.
    pub sync_gamemodes: Vec<GameMode>,
    /// List of interaction actions that trigger door sync. Use variant names like `RightClickBlock`, `RightClickAir`, etc. Leave empty to allow all.
    pub sync_actions: Vec<InteractAction>,
    /// Whether the player must be sneaking to trigger door sync.
    pub sync_sneaking_required: bool,
    /// Whether sneaking left-click knock is enabled.
    pub knock_enabled: bool,
    /// List of gamemodes allowed to knock on doors. Leave empty to allow all.
    pub knock_gamemodes: Vec<GameMode>,
    /// List of interaction actions that trigger knocking. Use variant names like `LeftClickBlock`, `RightClickBlock`, etc. Leave empty to allow all.
    pub knock_actions: Vec<InteractAction>,
    /// Whether the player must be sneaking to knock.
    pub knock_sneaking_required: bool,
}

impl Default for OpenableConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sync_enabled: false,
            sync_gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            sync_actions: vec![InteractAction::RightClickBlock],
            sync_sneaking_required: false,
            knock_enabled: false,
            knock_gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            knock_actions: vec![InteractAction::LeftClickBlock],
            knock_sneaking_required: true,
        }
    }
}
