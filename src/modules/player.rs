use crate::modules::module::Module;
use pumpkin::plugin::player::player_join::PlayerJoinEvent;
use pumpkin::plugin::player::player_leave::PlayerLeaveEvent;
use pumpkin::plugin::{BoxFuture, EventHandler};
use pumpkin::server::Server;
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::TextComponent;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a module handling player mechanics within the system.
pub struct PlayerModule {
    config: Config,
}

impl Module for PlayerModule {
    fn enabled(&self) -> bool {
        self.config.enabled
    }
}

#[with_runtime(global)]
impl EventHandler<PlayerJoinEvent> for PlayerModule {
    fn handle_blocking(
        &self,
        _server: &Arc<Server>,
        event: &mut PlayerJoinEvent,
    ) -> BoxFuture<'_, ()> {
        if !self.enabled() {
            return;
        }

        Box::pin(async move {
            event.join_message = TextComponent::text(
                self.config
                    .join_msg
                    .replace("{player}", &event.player.gameprofile.name),
            );
        })
    }
}

#[with_runtime(global)]
impl EventHandler<PlayerLeaveEvent> for PlayerModule {
    fn handle_blocking(
        &self,
        _server: &Arc<Server>,
        event: &mut PlayerLeaveEvent,
    ) -> BoxFuture<'_, ()> {
        if !self.enabled() {
            return;
        }

        Box::pin(async move {
            event.leave_message = TextComponent::text(
                self.config
                    .leave_msg
                    .replace("{player}", &event.player.gameprofile.name),
            );
        })
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
            enabled: true,
            join_msg: "<green>➕<reset> <gradient:#FFE259:#FFA751>›</gradient> {player}!"
                .to_string(),
            leave_msg: "<red>➖<reset> <gradient:#FFE259:#FFA751>›</gradient> {player}!"
                .to_string(),
        }
    }
}
