//! Chat module - chat formatting and word filtering.
//!
//! ## Configuration
//!
//! | Field         | Default | Description                                                        |
//! |---------------|---------|--------------------------------------------------------------------|
//! | `enabled`     | `false` | Whether this module is active                                      |
//! | `chat_format` | `""`    | Custom chat format. Use `{player}` and `{message}` placeholders    |
//! | `chat_filter` | `[]`    | List of blocked words/phrases (case-insensitive)                   |
//!
//! ## Placeholders
//!
//! | Placeholder | Available in                                    |
//! |-------------|-------------------------------------------------|
//! | `{player}`  | `chat_format`                                   |
//! | `{message}` | `chat_format`                                   |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerChatEvent};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles chat formatting and word filtering.
#[derive(Default)]
pub struct Chat;

impl Mechanic for Chat {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<ChatConfig>().enabled)
            .unwrap_or(true)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerChatEvent, _>(Chat, EventPriority::Highest, true)
            .expect("failed to register chat event handler");
    }
}

impl EventHandler<PlayerChatEvent> for Chat {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerChatEvent>,
    ) -> EventData<PlayerChatEvent> {
        let config: ChatConfig = ConfigManager::get()
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

/// Configuration for the chat module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Custom chat format. Use `{player}` and `{message}` as placeholders. Leave empty to disable.
    pub chat_format: String,
    /// List of blocked words/phrases. Messages containing any entry (case-insensitive) are cancelled. Leave empty to disable.
    pub chat_filter: Vec<String>,
}
