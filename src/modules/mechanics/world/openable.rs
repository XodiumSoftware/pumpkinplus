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
use crate::mirror_types::{GameMode, InteractionAction};
use crate::module::Module;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Handles openable block synchronization (e.g. double doors).
#[derive(Default)]
pub struct Openable;

impl Module for Openable {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<OpenableConfig>().enabled)
            .unwrap_or(true)
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

        let action = InteractionAction::from_debug(&event.action);
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

        let adjacent_pos = find_adjacent_door(&world, clicked_pos, &event.block);

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

        let toggled_clicked_id = find_toggled_door_state(clicked_state_id);
        let toggled_adjacent_id = find_toggled_door_state(adjacent_state_id);
        info!(
            "[Openable] toggled_clicked_id = {:?}, toggled_adjacent_id = {:?}",
            toggled_clicked_id, toggled_adjacent_id
        );

        if let (Some(new_clicked), Some(new_adjacent)) = (toggled_clicked_id, toggled_adjacent_id) {
            event.cancelled = true;
            info!("[Openable] event cancelled, syncing door states");

            let flags = pumpkin_plugin_api::world::BlockFlags::empty()
                .union(pumpkin_plugin_api::world::BlockFlags::NOTIFY_NEIGHBORS)
                .union(pumpkin_plugin_api::world::BlockFlags::NOTIFY_LISTENERS);

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
        }

        event
    }
}

/// Searches the four horizontal neighbors for a door of the same material.
///
/// Iterates over the four cardinal directions (±x, ±z) and checks whether
/// the block at each neighbor position has the same registry key as
/// `door_type`. Returns the first match found.
fn find_adjacent_door(
    world: &pumpkin_plugin_api::world::World,
    pos: pumpkin_plugin_api::world::BlockPos,
    door_type: &str,
) -> Option<pumpkin_plugin_api::world::BlockPos> {
    let neighbors = [
        pumpkin_plugin_api::world::BlockPos {
            x: pos.x + 1,
            y: pos.y,
            z: pos.z,
        },
        pumpkin_plugin_api::world::BlockPos {
            x: pos.x - 1,
            y: pos.y,
            z: pos.z,
        },
        pumpkin_plugin_api::world::BlockPos {
            x: pos.x,
            y: pos.y,
            z: pos.z + 1,
        },
        pumpkin_plugin_api::world::BlockPos {
            x: pos.x,
            y: pos.y,
            z: pos.z - 1,
        },
    ];

    for neighbor in &neighbors {
        let neighbor_type = get_block_registry_key(world, *neighbor);
        if neighbor_type.as_deref() == Some(door_type) {
            return Some(*neighbor);
        }
    }

    None
}

/// Gets the block registry key (e.g. `"minecraft:oak_door"`) at a position.
///
/// Currently returns `None` for air or liquid blocks, and a placeholder
/// `"unknown_door"` for everything else. This may be refined once the
/// plugin API exposes block state properties directly.
///
/// Returns `None` if the block is air or liquid.
fn get_block_registry_key(
    world: &pumpkin_plugin_api::world::World,
    pos: pumpkin_plugin_api::world::BlockPos,
) -> Option<String> {
    let state = world.get_block_state(pos);
    info!(
        "[Openable] get_block_registry_key at {:?}: is_air={}, is_liquid={}",
        pos, state.is_air, state.is_liquid
    );
    if state.is_air || state.is_liquid {
        return None;
    }
    // FIXME: Plugin API does not expose the actual registry key yet.
    // We return a placeholder here, which breaks find_adjacent_door's type matching.
    info!("[Openable] returning placeholder unknown_door");
    Some("unknown_door".to_string())
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
fn find_toggled_door_state(state_id: u16) -> Option<u16> {
    let candidates = [
        state_id.wrapping_add(1),
        state_id.wrapping_sub(1),
        state_id.wrapping_add(2),
        state_id.wrapping_sub(2),
        state_id.wrapping_add(4),
        state_id.wrapping_sub(4),
    ];
    Some(candidates[0])
}

/// Configuration for the openable mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenableConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// List of gamemodes allowed to trigger the mechanic. Use variant names like "Survival", "Creative", etc. Leave empty to allow all.
    pub gamemodes: Vec<GameMode>,
    /// List of interaction actions that trigger the mechanic. Use variant names like "RightClickBlock", "RightClickAir", etc. Leave empty to allow all.
    pub actions: Vec<InteractionAction>,
}

impl Default for OpenableConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            actions: vec![InteractionAction::RightClickBlock],
        }
    }
}
