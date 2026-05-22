//! Enderchest module - per-player enderchest sharing and management.
//!
//! ## Configuration
//!
//! | Field       | Default                | Description                               |
//! |-------------|------------------------|-------------------------------------------|
//! | `enabled`   | `false`                | Whether this module is active             |
//! | `gamemodes` | `["Survival", "Adventure"]` | Gamemodes allowed to use enderchests      |
//! | `actions`   | `["RightClickAir"]`        | Interaction actions that trigger the GUI  |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::{GameMode, InteractAction};
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles enderchest mechanics.
#[derive(Default)]
pub struct Enderchest;

impl Mechanic for Enderchest {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<EnderchestConfig>().enabled)
            .unwrap_or(true)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerInteractEvent, _>(
                Enderchest,
                EventPriority::Normal,
                true,
            )
            .expect("failed to register enderchest event handler");
    }
}

impl EventHandler<PlayerInteractEvent> for Enderchest {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerInteractEvent>,
    ) -> EventData<PlayerInteractEvent> {
        if !self.enabled() {
            return event;
        }

        let config: EnderchestConfig = ConfigManager::get()
            .map(|cm| cm.get_config())
            .unwrap_or_default();

        let action = InteractAction::from(event.action);
        if !action.matches_config(&config.actions) {
            return event;
        }

        if event.block != "minecraft:ender_chest" {
            return event;
        }

        let gamemode = GameMode::from(event.player.get_gamemode());
        if !gamemode.matches_config(&config.gamemodes) {
            return event;
        }

        //TODO: notify api-maintainers to add enderchest support.

        event
    }
}

/// Configuration for the enderchest mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnderchestConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// List of gamemodes allowed to use enderchests. Use variant names like "Survival", "Creative", etc. Leave empty to allow all.
    pub gamemodes: Vec<GameMode>,
    /// List of interaction actions that trigger opening the enderchest. Use variant names like "RightClickBlock", "RightClickAir", etc. Leave empty to allow all.
    pub actions: Vec<InteractAction>,
}

impl Default for EnderchestConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            actions: vec![InteractAction::RightClickAir],
        }
    }
}
