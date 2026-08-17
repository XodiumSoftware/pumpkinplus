//! Openable module - synchronizes multi-block openable structures like double doors.
//!
//! When a player right-clicks a door that is part of a double-door setup,
//! the adjacent door is toggled to match, so both open and close together.
//!
//! ## Configuration
//!
//! | Field       | Default                 | Description                   |
//! |-------------|-------------------------|-------------------------------|
//! | `enabled`   | `false`                 | Whether this module is active |
//! | `gamemodes` | `["Survival", "Adventure"]` | Gamemodes that trigger sync   |
//! | `actions`   | `["RightClickBlock"]`   | Actions that trigger the sync |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::utils::block::toggle_open_property;
use crate::{GameMode, InteractAction};
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::world::{BlockFlags, BlockPos, World, block_state_to_info};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles openable block synchronization (e.g. double doors).
#[derive(Default)]
pub struct Openable;

impl Mechanic for Openable {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.openable.enabled)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerInteractEvent, _>(Openable, EventPriority::Normal, true)
            .expect("failed to register openable event handler");
    }
}

impl EventHandler<PlayerInteractEvent> for Openable {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerInteractEvent>,
    ) -> EventData<PlayerInteractEvent> {
        if !self.enabled() {
            return event;
        }

        let config: OpenableConfig = ConfigManager::get()
            .map(|cm| cm.openable)
            .unwrap_or_default();

        let action = InteractAction::from(event.action);
        if !action.matches_config(&config.actions) {
            return event;
        }

        let gamemode = GameMode::from(event.player.get_gamemode());
        if !gamemode.matches_config(&config.gamemodes) {
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

        if !clicked_info.name.ends_with("_door") {
            return event;
        }

        let Some(adjacent_pos) = find_adjacent_door(&world, clicked_pos) else {
            return event;
        };

        let adjacent_state_id = world.get_block_state_id(adjacent_pos);
        let Some(adjacent_info) = block_state_to_info(adjacent_state_id) else {
            return event;
        };

        if !adjacent_info.name.ends_with("_door") {
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
        if block_state_to_info(state_id).is_some_and(|info| info.name.ends_with("_door")) {
            return Some(*neighbor);
        }
    }

    None
}

/// Configuration for the openable mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenableConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// List of gamemodes allowed to trigger the mechanic. Use variant names like "Survival", "Creative", etc. Leave empty to allow all.
    pub gamemodes: Vec<GameMode>,
    /// List of interaction actions that trigger the mechanic. Use variant names like `RightClickBlock`, `RightClickAir`, etc. Leave empty to allow all.
    pub actions: Vec<InteractAction>,
}

impl Default for OpenableConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            actions: vec![InteractAction::RightClickBlock],
        }
    }
}
