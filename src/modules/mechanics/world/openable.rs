//! Openable module - synchronizes multi-block openable structures like double doors.
//!
//! When a player right-clicks a door that is part of a double door setup,
//! the adjacent door is toggled to match, so both open and close together.
//!
//! ## Configuration
//!
//! | Field       | Default                 | Description                   |
//! |-------------|-------------------------|-------------------------------|
//! | `enabled`   | `false`                 | Whether this module is active |
//! | `gamemodes` | `["Survival"]`          | Gamemodes that trigger sync   |
//! | `actions`   | `["RightClickAir"]`     | Actions that trigger the sync |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::{GameMode, InteractAction};
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::world::{BlockFlags, BlockPos, World};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Handles openable block synchronization (e.g. double doors).
#[derive(Default)]
pub struct Openable;

impl Mechanic for Openable {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_none_or(|cm| cm.get_config::<OpenableConfig>().enabled)
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
        info!("[Openable] handle triggered");

        if !self.enabled() {
            info!("[Openable] module disabled, returning early");
            return event;
        }

        let config: OpenableConfig = ConfigManager::get()
            .map(|cm| cm.get_config())
            .unwrap_or_default();

        let action = InteractAction::from(event.action);
        info!(
            "[Openable] raw action debug = {:?}, parsed action = {:?}, config.actions = {:?}",
            event.action, action, config.actions
        );
        if !action.matches_config(&config.actions) {
            info!("[Openable] action does not match config, returning early");
            return event;
        }

        let gamemode = GameMode::from(event.player.get_gamemode());
        info!(
            "[Openable] gamemode = {:?}, config.gamemodes = {:?}",
            gamemode, config.gamemodes
        );
        if !gamemode.matches_config(&config.gamemodes) {
            info!("[Openable] gamemode does not match config, returning early");
            return event;
        }

        info!("[Openable] block = {}", event.block);
        if !event.block.ends_with("_door") {
            info!("[Openable] block is not a door, returning early");
            return event;
        }

        let Some(clicked_pos) = event.clicked_pos else {
            info!("[Openable] no clicked_pos, returning early");
            return event;
        };
        info!("[Openable] clicked_pos = {:?}", clicked_pos);

        let world = event.player.get_world();

        let clicked_state_id = world.get_block_state_id(clicked_pos);
        info!("[Openable] clicked_state_id = {}", clicked_state_id);

        let adjacent_pos = find_adjacent_door(&world, clicked_pos);

        let Some(adjacent_pos) = adjacent_pos else {
            info!("[Openable] no adjacent door found, returning early");
            return event;
        };
        info!("[Openable] adjacent_pos = {:?}", adjacent_pos);

        let adjacent_state_id = world.get_block_state_id(adjacent_pos);
        info!("[Openable] adjacent_state_id = {}", adjacent_state_id);

        if clicked_state_id == adjacent_state_id {
            info!("[Openable] clicked_state_id == adjacent_state_id, returning early");
            return event;
        }

        let new_clicked = find_toggled_door_state(clicked_state_id);
        let new_adjacent = find_toggled_door_state(adjacent_state_id);
        info!(
            "[Openable] new_clicked = {}, new_adjacent = {}",
            new_clicked, new_adjacent
        );

        event.cancelled = true;
        info!("[Openable] event cancelled, syncing door states");

        let flags = BlockFlags::NOTIFY_NEIGHBORS | BlockFlags::NOTIFY_LISTENERS;

        world.set_block_state(clicked_pos, new_clicked, flags);
        world.set_block_state(adjacent_pos, new_adjacent, flags);

        debug!(
            "Synced double doors at {:?} and {:?} (states {} -> {}, {} -> {})",
            clicked_pos,
            adjacent_pos,
            clicked_state_id,
            new_clicked,
            adjacent_state_id,
            new_adjacent
        );

        event
    }
}

/// Searches the four horizontal neighbors for a door block.
///
/// Iterates over the four cardinal directions (±x, ±z) and returns the first
/// neighbor that is not air or liquid. In a valid double-door setup this
/// will be the paired door, since the two door halves are the only blocks
/// occupying those adjacent positions at the same Y level.
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
        let state = world.get_block_state(*neighbor);
        if !state.is_air && !state.is_liquid {
            return Some(*neighbor);
        }
    }

    None
}

/// Attempts to find the toggled (open <-> closed) state ID for a door.
///
/// In Minecraft's block state encoding, the `open` property for doors is
/// typically encoded as one of the low bits. For a given combination of
/// `facing`, `half`, `hinge`, and `powered`, the `open=false` and `open=true`
/// variants usually differ by a small offset.
///
/// Since we don't have direct property access in the plugin API, we try a
/// small set of nearby state IDs and pick the one that is most likely the
/// toggled counterpart. The most common offset in vanilla is ±1 or ±2.
fn find_toggled_door_state(state_id: u16) -> u16 {
    let candidates = [
        state_id.wrapping_add(1),
        state_id.wrapping_sub(1),
        state_id.wrapping_add(2),
        state_id.wrapping_sub(2),
        state_id.wrapping_add(4),
        state_id.wrapping_sub(4),
    ];
    candidates[0]
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
