//! Openable module — synchronizes multi-block openable structures like double doors
//! and adds a door-knock mechanic.
//!
//! ## Double door sync
//!
//! When a player right-clicks a door that is part of a double-door setup,
//! the adjacent door is toggled to match, so both open and close together.
//!
//! ## Door knock
//!
//! When a player left-clicks a door while sneaking with an empty main hand,
//! a knocking sound is played at the door. The event is cancelled so the door
//! is not damaged.
//!
//! > **Note:** Pumpkin's current WASM host has a bug in `world.play-sound`
//! > sound-name conversion (it lowercases the enum variant and replaces
//! > underscores with dots, but the variants are PascalCase without underscores,
//! > so no sound is actually emitted). The knock code is in place and will work
//! > once upstream fixes the conversion.
//!
//! ## Configuration
//!
//! | Field                  | Default                       | Description                                              |
//! |------------------------|-------------------------------|----------------------------------------------------------|
//! | `enabled`              | `false`                       | Whether this module is active                          |
//! | `gamemodes`            | `["Survival", "Adventure"]`   | Gamemodes that trigger door sync                       |
//! | `actions`              | `["RightClickBlock"]`         | Actions that trigger door sync                         |
//! | `knock_enabled`        | `true`                        | Whether sneaking left-click door knocking is enabled   |
//! | `knock_gamemodes`      | `["Survival", "Adventure"]`   | Gamemodes allowed to knock                             |
//! | `knock_sneaking_required` | `true`                     | Whether the player must be sneaking to knock           |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::utils::block::toggle_open_property;
use crate::{GameMode, InteractAction};
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::world::{BlockFlags, BlockPos, World, block_state_to_info};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Handles openable block synchronization and door knocking.
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
        info!("[Openable] event action={:?}", action);

        if action == InteractAction::LeftClickBlock && config.knock_enabled {
            return handle_knock(event, &config);
        }

        if !action.matches_config(&config.actions) {
            info!("[Openable] sync action not allowed, returning");
            return event;
        }

        let gamemode = GameMode::from(event.player.get_gamemode());
        info!(
            "[Openable] sync gamemode={:?}, allowed={:?}",
            gamemode, config.gamemodes
        );
        if !gamemode.matches_config(&config.gamemodes) {
            info!("[Openable] sync gamemode not allowed, returning");
            return event;
        }

        let Some(clicked_pos) = event.clicked_pos else {
            info!("[Openable] sync no clicked_pos, returning");
            return event;
        };

        let world = event.player.get_world();

        let clicked_state_id = world.get_block_state_id(clicked_pos);
        info!(
            "[Openable] sync clicked_pos={:?}, state_id={}",
            clicked_pos, clicked_state_id
        );
        let Some(clicked_info) = block_state_to_info(clicked_state_id) else {
            info!("[Openable] sync could not resolve block info, returning");
            return event;
        };

        info!("[Openable] sync block name={}", clicked_info.name);
        if !clicked_info.name.ends_with("_door") {
            info!("[Openable] sync block is not a door, returning");
            return event;
        }

        let Some(adjacent_pos) = find_adjacent_door(&world, clicked_pos) else {
            info!("[Openable] sync no adjacent door, returning");
            return event;
        };

        let adjacent_state_id = world.get_block_state_id(adjacent_pos);
        info!(
            "[Openable] sync adjacent_pos={:?}, state_id={}",
            adjacent_pos, adjacent_state_id
        );
        let Some(adjacent_info) = block_state_to_info(adjacent_state_id) else {
            info!("[Openable] sync could not resolve adjacent block info, returning");
            return event;
        };

        if !adjacent_info.name.ends_with("_door") {
            info!("[Openable] sync adjacent block is not a door, returning");
            return event;
        }

        let Some(new_clicked_id) = toggle_open_property(&clicked_info) else {
            info!("[Openable] sync could not toggle clicked door, returning");
            return event;
        };
        let Some(new_adjacent_id) = toggle_open_property(&adjacent_info) else {
            info!("[Openable] sync could not toggle adjacent door, returning");
            return event;
        };

        info!(
            "[Openable] sync toggling doors: {} -> {}, {} -> {}",
            clicked_state_id, new_clicked_id, adjacent_state_id, new_adjacent_id
        );
        event.cancelled = true;

        let flags = BlockFlags::NOTIFY_NEIGHBORS | BlockFlags::NOTIFY_LISTENERS;

        world.set_block_state(clicked_pos, new_clicked_id, flags);
        world.set_block_state(adjacent_pos, new_adjacent_id, flags);

        info!("[Openable] sync complete");
        event
    }
}

/// Handles the door-knock interaction.
fn handle_knock(
    mut event: EventData<PlayerInteractEvent>,
    config: &OpenableConfig,
) -> EventData<PlayerInteractEvent> {
    info!("[Openable] Knock attempt detected");

    let gamemode = GameMode::from(event.player.get_gamemode());
    info!(
        "[Openable] knock gamemode={:?}, allowed={:?}",
        gamemode, config.knock_gamemodes
    );
    if !gamemode.matches_config(&config.knock_gamemodes) {
        info!("[Openable] knock gamemode not allowed, returning");
        return event;
    }

    let is_sneaking = event.player.as_entity().is_sneaking();
    info!(
        "[Openable] knock sneaking_required={}, is_sneaking={}",
        config.knock_sneaking_required, is_sneaking
    );
    if config.knock_sneaking_required && !is_sneaking {
        info!("[Openable] player not sneaking, returning");
        return event;
    }

    let hand_item = event.player.get_item_in_hand(Hand::Right);
    info!("[Openable] knock hand_item_present={}", hand_item.is_some());
    if hand_item.is_some() {
        info!("[Openable] hand not empty, returning");
        return event;
    }

    let Some(clicked_pos) = event.clicked_pos else {
        info!("[Openable] no clicked_pos for knock, returning");
        return event;
    };

    let world = event.player.get_world();

    let clicked_state_id = world.get_block_state_id(clicked_pos);
    info!(
        "[Openable] knock clicked_pos={:?}, state_id={}",
        clicked_pos, clicked_state_id
    );
    let Some(clicked_info) = block_state_to_info(clicked_state_id) else {
        info!("[Openable] could not resolve knock block info, returning");
        return event;
    };

    info!("[Openable] knock block name={}", clicked_info.name);
    if !clicked_info.name.ends_with("_door") {
        info!("[Openable] knock block is not a door, returning");
        return event;
    }

    info!("[Openable] playing knock sound at {:?}", clicked_pos);
    world.play_sound(
        pumpkin_plugin_api::world::Sound::EntityZombieAttackWoodenDoor,
        pumpkin_plugin_api::world::SoundCategory::Master,
        block_center(clicked_pos),
        1.0,
        1.0,
    );

    event.cancelled = true;
    info!("[Openable] knock handled and event cancelled");
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
        if block_state_to_info(state_id).is_some_and(|info| info.name.ends_with("_door")) {
            return Some(*neighbor);
        }
    }

    None
}

/// Returns the center of a block position as a world position.
fn block_center(pos: BlockPos) -> (f64, f64, f64) {
    (
        f64::from(pos.x) + 0.5,
        f64::from(pos.y) + 0.5,
        f64::from(pos.z) + 0.5,
    )
}

/// Configuration for the openable mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenableConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// List of gamemodes allowed to trigger door sync. Use variant names like "Survival", "Creative", etc. Leave empty to allow all.
    pub gamemodes: Vec<GameMode>,
    /// List of interaction actions that trigger door sync. Use variant names like `RightClickBlock`, `RightClickAir`, etc. Leave empty to allow all.
    pub actions: Vec<InteractAction>,
    /// Whether sneaking left-click door knocking is enabled.
    ///
    /// Note: sound playback is currently broken in Pumpkin's WASM host
    /// (see module-level docs). Disable or leave disabled until fixed.
    pub knock_enabled: bool,
    /// List of gamemodes allowed to knock on doors. Leave empty to allow all.
    pub knock_gamemodes: Vec<GameMode>,
    /// Whether the player must be sneaking to knock.
    pub knock_sneaking_required: bool,
}

impl Default for OpenableConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            actions: vec![InteractAction::RightClickBlock],
            knock_enabled: false,
            knock_gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            knock_sneaking_required: true,
        }
    }
}
