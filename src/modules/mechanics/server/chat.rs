//! Chat module - chat formatting and word filtering.
//!
//! ## Configuration
//!
//! | Field         | Default | Description                                                        |
//! |---------------|---------|--------------------------------------------------------------------|
//! | `enabled`     | `false` | Whether this module is active                                      |
//! | `chat_format` | `""`    | Chat format with `MiniMessage` tags and `{player}`/`{message}` placeholders |
//! | `chat_filter` | `[]`    | List of blocked words/phrases (case-insensitive)                   |
//!
//! The `chat_format` field supports [MiniMessage](https://docs.advntr.dev/minimessage/format.html)
//! formatting tags (resolved after placeholders).
//!
//! ## Placeholders
//!
//! | Placeholder | Available in                                    |
//! |-------------|-------------------------------------------------|
//! | `{player}`  | `chat_format`                                   |
//! | `{message}` | `chat_format`                                   |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::utils::placeholders::replace_placeholders;
use crate::utils::text::parse_minimessage;
use pumpkin_plugin_api::events::{AsyncPlayerChatEvent, EventData, EventHandler, EventPriority};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles chat formatting and word filtering.
#[derive(Default)]
pub struct Chat;

impl Mechanic for Chat {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.chat.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<AsyncPlayerChatEvent>(context, EventPriority::Highest, true);
    }
}

impl EventHandler<AsyncPlayerChatEvent> for Chat {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<AsyncPlayerChatEvent>,
    ) -> EventData<AsyncPlayerChatEvent> {
        let config: ChatConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.chat)
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
            let formatted = replace_placeholders(
                &config.chat_format,
                &[("{player}", name.as_str()), ("{message}", &original)],
            );
            event.format = parse_minimessage(&formatted);
        }

        event
    }
}

/// Configuration for the chat module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Custom chat format. Use `{player}` and `{message}` as placeholders. Supports `MiniMessage` tags. Leave empty to disable.
    pub chat_format: String,
    /// List of blocked words/phrases. Messages containing any entry (case-insensitive) are cancelled. Leave empty to disable.
    pub chat_filter: Vec<String>,
}
