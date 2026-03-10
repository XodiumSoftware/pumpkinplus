use crate::modules::module::Module;

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::{EventHandler, PlayerJoinEventData, PlayerLeaveEventData};
use pumpkin_plugin_api::text::TextComponent;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Player {
    config: Config,
}

impl Module for Player {
    fn enabled(&self) -> bool {
        self.config.enabled
    }
}

impl EventHandler<PlayerJoinEventData> for Player {
    fn handle(&self, _server: Server, mut event: PlayerJoinEventData) -> PlayerJoinEventData {
        if self.enabled() {
            event.join_message = TextComponent::text(
                self.config
                    .join_msg
                    .replace("{player}", &event.player.name)
                    .as_str(),
            )
        }

        event
    }
}

impl EventHandler<PlayerLeaveEventData> for Player {
    fn handle(&self, _server: Server, mut event: PlayerLeaveEventData) -> PlayerLeaveEventData {
        if self.enabled() {
            event.leave_message = TextComponent::text(
                self.config
                    .leave_msg
                    .replace("{player}", &event.player.name)
                    .as_str(),
            );
        }

        event
    }
}

/// Represents the config of the module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub join_msg: String,
    pub leave_msg: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            join_msg: "<green>➕<reset> <gradient:#FFE259:#FFA751>›</gradient> {player}!".into(),
            leave_msg: "<red>➖<reset> <gradient:#FFE259:#FFA751>›</gradient> {player}!".into(),
        }
    }
}
