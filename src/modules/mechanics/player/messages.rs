//! Messages module - custom join, leave, and kick messages.
//!
//! ## Configuration
//!
//! | Field       | Default | Description                                                        |
//! |-------------|---------|--------------------------------------------------------------------|
//! | `enabled`   | `false` | Whether this module is active                                      |
//! | `join_msg`  | `""`    | Message broadcast when a player joins                              |
//! | `leave_msg` | `""`    | Message broadcast when a player leaves                             |
//! | `kick_msg`  | `""`    | Message shown when a player is kicked during login                 |
//!
//! ## Placeholders
//!
//! | Placeholder | Available in                                    |
//! |-------------|-------------------------------------------------|
//! | `{player}`  | `join_msg`, `leave_msg`, `kick_msg`             |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerJoinEvent, PlayerLeaveEvent, PlayerLoginEvent,
};
use pumpkin_plugin_api::{Context, Server, text::TextComponent};
use serde::{Deserialize, Serialize};

/// Handles player join, leave, and kick messages.
#[derive(Default)]
pub struct Messages;

impl Mechanic for Messages {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_none_or(|cm| cm.messages.enabled)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerJoinEvent, _>(Messages, EventPriority::Highest, true)
            .expect("failed to register join event handler");
        context
            .register_event_handler::<PlayerLeaveEvent, _>(Messages, EventPriority::Highest, true)
            .expect("failed to register leave event handler");
        context
            .register_event_handler::<PlayerLoginEvent, _>(Messages, EventPriority::Highest, true)
            .expect("failed to register login event handler");
    }
}

impl EventHandler<PlayerJoinEvent> for Messages {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let config: MessagesConfig = ConfigManager::get()
            .map(|cm| cm.messages)
            .unwrap_or_default();
        if config.join_msg.is_empty() {
            return event;
        }
        let name = event.player.get_display_name().get_text();
        event.join_message =
            TextComponent::text(config.join_msg.replace("{player}", &name).as_str());
        event
    }
}

impl EventHandler<PlayerLeaveEvent> for Messages {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerLeaveEvent>,
    ) -> EventData<PlayerLeaveEvent> {
        let config: MessagesConfig = ConfigManager::get()
            .map(|cm| cm.messages)
            .unwrap_or_default();
        if config.leave_msg.is_empty() {
            return event;
        }
        let name = event.player.get_display_name().get_text();
        event.leave_message =
            TextComponent::text(config.leave_msg.replace("{player}", &name).as_str());
        event
    }
}

impl EventHandler<PlayerLoginEvent> for Messages {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerLoginEvent>,
    ) -> EventData<PlayerLoginEvent> {
        let config: MessagesConfig = ConfigManager::get()
            .map(|cm| cm.messages)
            .unwrap_or_default();
        if config.kick_msg.is_empty() {
            return event;
        }
        let name = event.player.get_display_name().get_text();
        event.kick_message =
            TextComponent::text(config.kick_msg.replace("{player}", &name).as_str());
        event
    }
}

/// Configuration for the messages module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagesConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Message broadcast when a player joins. Use `{player}` as a placeholder for the player's name. Leave empty to disable.
    pub join_msg: String,
    /// Message broadcast when a player leaves. Use `{player}` as a placeholder for the player's name. Leave empty to disable.
    pub leave_msg: String,
    /// Message shown to the player when they are kicked during login. Use `{player}` as a placeholder for the player's name. Leave empty to disable.
    pub kick_msg: String,
}
