use pumpkin::plugin::player::player_join::PlayerJoinEvent;
use pumpkin::plugin::player::player_leave::PlayerLeaveEvent;
use pumpkin::plugin::{BoxFuture, EventHandler};
use pumpkin::server::Server;
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::TextComponent;
use pumpkin_util::text::color::NamedColor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Module for handling player-related events.
pub struct PlayerModule {
    pub config: Config,
}

#[with_runtime(global)]
impl EventHandler<PlayerJoinEvent> for PlayerModule {
    fn handle_blocking(
        &self,
        _server: &Arc<Server>,
        event: &mut PlayerJoinEvent,
    ) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            event.join_message =
                TextComponent::text(format!("Welcome, {}!", event.player.gameprofile.name))
                    .color_named(NamedColor::Green);
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
        Box::pin(async move {
            event.leave_message =
                TextComponent::text(format!("Goodbye, {}!", event.player.gameprofile.name))
                    .color_named(NamedColor::Red);
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub join_msg: String,
    pub leave_msg: String,
}
