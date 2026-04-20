//! Player module - handles join/leave messages and chat customization.
//!
//! ## Configuration
//!
//! | Field         | Default | Description                                                        |
//! |---------------|---------|--------------------------------------------------------------------|
//! | `enabled`     | `false` | Whether this module is active                                      |
//! | `join_msg`    | `""`    | Message broadcast when a player joins                              |
//! | `leave_msg`   | `""`    | Message broadcast when a player leaves                             |
//! | `kick_msg`    | `""`    | Message shown when a player is kicked during login               |
//! | `chat_format` | `""`    | Custom chat format. Use `{player}` and `{message}` placeholders    |
//! | `chat_filter` | `[]`    | List of blocked words/phrases (case-insensitive)                   |
//!
//! ## Placeholders
//!
//! | Placeholder | Available in                                    |
//! |-------------|-------------------------------------------------|
//! | `{player}`  | `join_msg`, `leave_msg`, `kick_msg`, `chat_format` |
//! | `{message}` | `chat_format`                                   |

use crate::config::ConfigManager;
use crate::module::Module;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerChatEvent, PlayerJoinEvent, PlayerLeaveEvent,
    PlayerLoginEvent,
};
use pumpkin_plugin_api::{Context, Server, text::TextComponent};
use serde::{Deserialize, Serialize};

/// Handles player join and leave mechanics, including custom messages.
#[derive(Default)]
pub struct Player;

impl Module for Player {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<Config>().enabled)
            .unwrap_or(true)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerJoinEvent, _>(Player, EventPriority::Highest, true)
            .expect("failed to register player join event handler");
        context
            .register_event_handler::<PlayerLeaveEvent, _>(Player, EventPriority::Highest, true)
            .expect("failed to register player leave event handler");
        context
            .register_event_handler::<PlayerLoginEvent, _>(Player, EventPriority::Highest, true)
            .expect("failed to register player login event handler");
        context
            .register_event_handler::<PlayerChatEvent, _>(Player, EventPriority::Highest, true)
            .expect("failed to register player chat event handler");
    }
}

impl EventHandler<PlayerJoinEvent> for Player {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let config: Config = ConfigManager::get()
            .map(|cm| cm.get_config())
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

impl EventHandler<PlayerLeaveEvent> for Player {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerLeaveEvent>,
    ) -> EventData<PlayerLeaveEvent> {
        let config: Config = ConfigManager::get()
            .map(|cm| cm.get_config())
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

impl EventHandler<PlayerLoginEvent> for Player {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerLoginEvent>,
    ) -> EventData<PlayerLoginEvent> {
        let config: Config = ConfigManager::get()
            .map(|cm| cm.get_config())
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

impl EventHandler<PlayerChatEvent> for Player {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerChatEvent>,
    ) -> EventData<PlayerChatEvent> {
        let config: Config = ConfigManager::get()
            .map(|cm| cm.get_config())
            .unwrap_or_default();
        if !config.chat_filter.is_empty() {
            let lower = event.message.to_lowercase();
            if config
                .chat_filter
                .iter()
                .any(|word| lower.contains(word.as_str()))
            {
                event.cancelled = true;
                return event;
            }
        }
        if !config.chat_format.is_empty() {
            let name = event.player.get_display_name().get_text();
            let original = event.message.clone();
            event.message = config
                .chat_format
                .replace("{player}", &name)
                .replace("{message}", &original);
        }
        event
    }
}

/// Configuration for the player mechanics module.
pub type PlayerConfig = Config;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
    /// Message broadcast when a player joins. Use `{player}` as a placeholder for the player's name. Leave empty to disable.
    pub join_msg: String,
    /// Message broadcast when a player leaves. Use `{player}` as a placeholder for the player's name. Leave empty to disable.
    pub leave_msg: String,
    /// Message shown to the player when they are kicked during login. Use `{player}` as a placeholder for the player's name. Leave empty to disable.
    pub kick_msg: String,
    /// Custom chat format. Use `{player}` and `{message}` as placeholders. Leave empty to disable.
    pub chat_format: String,
    /// List of blocked words/phrases. Messages containing any entry (case-insensitive) are cancelled. Leave empty to disable.
    pub chat_filter: Vec<String>,
}
