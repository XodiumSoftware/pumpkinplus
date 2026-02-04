use pumpkin::plugin::player::player_join::PlayerJoinEvent;
use pumpkin::plugin::player::player_leave::PlayerLeaveEvent;
use pumpkin::plugin::{BoxFuture, EventHandler};
use pumpkin::server::Server;
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::TextComponent;
use pumpkin_util::text::color::NamedColor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a module handling player mechanics within the system.
pub struct PlayerModule {
    pub config: PlayerModuleConfig,
}

#[with_runtime(global)]
impl EventHandler<PlayerJoinEvent> for PlayerModule {
    fn handle_blocking(
        &self,
        _server: &Arc<Server>,
        event: &mut PlayerJoinEvent,
    ) -> BoxFuture<'_, ()> {
        let msg = self
            .config
            .join_msg
            .replace("{player}", &event.player.gameprofile.name);

        Box::pin(async move {
            event.join_message = TextComponent::text(msg).color_named(NamedColor::Green);
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
        let msg = self
            .config
            .leave_msg
            .replace("{player}", &event.player.gameprofile.name);

        Box::pin(async move {
            event.leave_message = TextComponent::text(msg).color_named(NamedColor::Red);
        })
    }
}

/// Represents the config of the module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerModuleConfig {
    pub join_msg: String,
    pub leave_msg: String,
}

impl Default for PlayerModuleConfig {
    fn default() -> Self {
        Self {
            join_msg: "Welcome, {player}!".to_string(),
            leave_msg: "Goodbye, {player}!".to_string(),
        }
    }
}
