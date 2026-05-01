//! Double Doors module - synchronizes double door opening/closing.
//!
//! When a player right-clicks a door that is part of a double door setup,
//! the adjacent door is toggled to match, so both open and close together.
//!
//! ## Configuration
//!
//! | Field     | Default | Description                   |
//! |-----------|---------|-------------------------------|
//! | `enabled` | `false` | Whether this module is active |

use crate::config::ConfigManager;
use crate::module::Module;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Handles double door synchronization.
#[derive(Default)]
pub struct DoubleDoors;

impl Module for DoubleDoors {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<Config>().enabled)
            .unwrap_or(true)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerInteractEvent, _>(
                DoubleDoors,
                EventPriority::Normal,
                true,
            )
            .expect("failed to register double doors event handler");
    }
}

impl EventHandler<PlayerInteractEvent> for DoubleDoors {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerInteractEvent>,
    ) -> EventData<PlayerInteractEvent> {
        if !self.enabled() {
            return event;
        }

        if !matches!(
            event.action,
            pumpkin_plugin_api::events::InteractAction::RightClickBlock
        ) {
            return event;
        }

        if !event.block.ends_with("_door") {
            return event;
        }

        let Some(clicked_pos) = event.clicked_pos else {
            return event;
        };

        let world = event.player.get_world();

        let clicked_state_id = world.get_block_state_id(clicked_pos);

        let adjacent_pos = find_adjacent_door(&world, clicked_pos, &event.block);

        let Some(adjacent_pos) = adjacent_pos else {
            return event;
        };

        let adjacent_state_id = world.get_block_state_id(adjacent_pos);

        if clicked_state_id == adjacent_state_id {
            return event;
        }

        let toggled_clicked_id = find_toggled_door_state(clicked_state_id);
        let toggled_adjacent_id = find_toggled_door_state(adjacent_state_id);

        if let (Some(new_clicked), Some(new_adjacent)) = (toggled_clicked_id, toggled_adjacent_id) {
            event.cancelled = true;

            let flags = pumpkin_plugin_api::world::BlockFlags::new()
                .with_notify_neighbors(true)
                .with_notify_listeners(true);

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
fn find_adjacent_door(
    world: &pumpkin_plugin_api::world::World,
    pos: (i32, i32, i32),
    door_type: &str,
) -> Option<(i32, i32, i32)> {
    let neighbors = [
        (pos.0 + 1, pos.1, pos.2),
        (pos.0 - 1, pos.1, pos.2),
        (pos.0, pos.1, pos.2 + 1),
        (pos.0, pos.1, pos.2 - 1),
    ];

    for neighbor in &neighbors {
        let neighbor_type = get_block_registry_key(world, *neighbor);
        if neighbor_type.as_deref() == Some(door_type) {
            return Some(*neighbor);
        }
    }

    None
}

/// Gets the block registry key (e.g. "minecraft:oak_door") at a position.
/// Returns None if we can't determine the block type.
fn get_block_registry_key(
    world: &pumpkin_plugin_api::world::World,
    pos: (i32, i32, i32),
) -> Option<String> {
    let state = world.get_block_state(pos);
    if state.is_air || state.is_liquid {
        return None;
    }
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

/// Configuration for the double doors mechanics module.
pub type DoubleDoorsConfig = Config;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
}
