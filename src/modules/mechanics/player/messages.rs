//! Messages module - custom join, leave, kick, and death messages.
//!
//! ## Configuration
//!
//! | Field         | Default | Description                                                        |
//! |---------------|---------|--------------------------------------------------------------------|
//! | `enabled`     | `false` | Whether this module is active                                      |
//! | `join_msg`    | `""`    | Message broadcast when a player joins                              |
//! | `leave_msg`   | `""`    | Message broadcast when a player leaves                             |
//! | `kick_msg`    | `""`    | Message shown when a player is kicked during login                 |
//! | `death_msg`   | `""`    | Message broadcast when a player dies                               |
//!
//! All message fields support [MiniMessage](https://docs.advntr.dev/minimessage/format.html)
//! formatting tags (resolved after placeholders).
//!
//! ## Placeholders
//!
//! | Placeholder | Available in                                     |
//! |-------------|--------------------------------------------------|
//! | `{player}`  | `join_msg`, `leave_msg`, `kick_msg`, `death_msg` |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::utils::placeholders::parse_with_player_component;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerDeathEvent, PlayerJoinEvent, PlayerLeaveEvent,
    PlayerLoginEvent,
};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles player join, leave, and kick messages.
#[derive(Default)]
pub struct Messages;

impl Mechanic for Messages {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.messages.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerJoinEvent>(context, EventPriority::Highest, true);
        self.register_event::<PlayerLeaveEvent>(context, EventPriority::Highest, true);
        self.register_event::<PlayerLoginEvent>(context, EventPriority::Highest, true);
        self.register_event::<PlayerDeathEvent>(context, EventPriority::Highest, true);
    }
}

impl EventHandler<PlayerJoinEvent> for Messages {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let config: MessagesConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.messages)
            .unwrap_or_default();
        if config.join_msg.is_empty() {
            return event;
        }
        event.join_message = parse_with_player_component(&config.join_msg, &event.player);
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
            .map(|cm| cm.mechanics.messages)
            .unwrap_or_default();
        if config.leave_msg.is_empty() {
            return event;
        }
        event.leave_message = parse_with_player_component(&config.leave_msg, &event.player);
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
            .map(|cm| cm.mechanics.messages)
            .unwrap_or_default();
        if config.kick_msg.is_empty() {
            return event;
        }
        event.kick_message = parse_with_player_component(&config.kick_msg, &event.player);
        event
    }
}

impl EventHandler<PlayerDeathEvent> for Messages {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerDeathEvent>,
    ) -> EventData<PlayerDeathEvent> {
        let config: MessagesConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.messages)
            .unwrap_or_default();
        if config.death_msg.is_empty() {
            return event;
        }
        event.death_message = parse_with_player_component(&config.death_msg, &event.player);
        event
    }
}

/// Configuration for the messages module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagesConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Message broadcast when a player joins. Use `{player}` as a placeholder for the player's name. Supports `MiniMessage` tags. Leave empty to disable.
    pub join_msg: String,
    /// Message broadcast when a player leaves. Use `{player}` as a placeholder for the player's name. Supports `MiniMessage` tags. Leave empty to disable.
    pub leave_msg: String,
    /// Message shown to the player when they are kicked during login. Use `{player}` as a placeholder for the player's name. Supports `MiniMessage` tags. Leave empty to disable.
    pub kick_msg: String,
    /// Message broadcast when a player dies. Use `{player}` as a placeholder for the player's name. Supports `MiniMessage` tags. Leave empty to disable.
    pub death_msg: String,
}
