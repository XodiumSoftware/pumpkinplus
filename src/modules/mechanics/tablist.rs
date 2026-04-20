//! Tablist module - custom header/footer with dynamic placeholders.
//!
//! ## Configuration
//!
//! | Field     | Default | Description                                                          |
//! |-----------|---------|----------------------------------------------------------------------|
//! | `enabled` | `false` | Whether this module is active                                        |
//! | `header`  | `""`    | Header text. Supports placeholders and Minecraft formatting codes  |
//! | `footer`  | `""`    | Footer text. Supports placeholders and Minecraft formatting codes  |
//!
//! ## Placeholders
//!
//! | Placeholder | Description                    | Example Output |
//! |-------------|--------------------------------|----------------|
//! | `{player}`  | Current player's name            | `Notch`        |
//! | `{online}`  | Number of online players         | `42`           |
//! | `{tps}`     | Server TPS (ticks per second)    | `20.0`         |
//! | `{mspt}`    | Milliseconds per tick            | `5.2`          |

use crate::config::ConfigManager;
use crate::module::Module;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerJoinEvent, PlayerLeaveEvent,
};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles tab-list mechanics, including custom messages.
#[derive(Default)]
pub struct Tablist;

impl Module for Tablist {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<Config>().enabled)
            .unwrap_or(true)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerJoinEvent, _>(Tablist, EventPriority::Normal, true)
            .expect("failed to register tablist event handler");
        context
            .register_event_handler::<PlayerLeaveEvent, _>(Tablist, EventPriority::Normal, true)
            .expect("failed to register tablist leave event handler");
    }
}

impl Tablist {
    fn replace_placeholders(
        text: &str,
        server: &Server,
        player: &pumpkin_plugin_api::player::Player,
    ) -> String {
        let player_name = player.get_display_name().get_text();
        let online = server.get_player_count();
        let tps = server.get_tps();
        let mspt = server.get_mspt();

        text.replace("{player}", &player_name)
            .replace("{online}", &online.to_string())
            .replace("{tps}", &format!("{:.1}", tps))
            .replace("{mspt}", &format!("{:.1}", mspt))
    }

    fn update_tablist_for_player(
        config: &Config,
        server: &Server,
        player: &pumpkin_plugin_api::player::Player,
    ) {
        let header = Self::replace_placeholders(&config.header, server, player);
        let footer = Self::replace_placeholders(&config.footer, server, player);
        player
            .set_tab_list_header_footer(TextComponent::text(&header), TextComponent::text(&footer));
    }

    fn update_tablist_for_all_players(server: &Server) {
        let config: Config = ConfigManager::get()
            .map(|cm| cm.get_config())
            .unwrap_or_default();

        if !config.enabled {
            return;
        }

        for player in server.get_all_players() {
            Self::update_tablist_for_player(&config, server, &player);
        }
    }
}

impl EventHandler<PlayerJoinEvent> for Tablist {
    fn handle(
        &self,
        server: Server,
        event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let config: Config = ConfigManager::get()
            .map(|cm| cm.get_config())
            .unwrap_or_default();

        if !self.enabled() {
            return event;
        }

        Self::update_tablist_for_player(&config, &server, &event.player);

        for player in server.get_all_players() {
            if player.get_display_name().get_text() != event.player.get_display_name().get_text() {
                Self::update_tablist_for_player(&config, &server, &player);
            }
        }

        event
    }
}

impl EventHandler<PlayerLeaveEvent> for Tablist {
    fn handle(
        &self,
        server: Server,
        event: EventData<PlayerLeaveEvent>,
    ) -> EventData<PlayerLeaveEvent> {
        if !self.enabled() {
            return event;
        }

        Self::update_tablist_for_all_players(&server);

        event
    }
}

/// Configuration for the tablist mechanics module.
pub type TablistConfig = Config;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
    /// Header text displayed at the top of the tab list. Supports Minecraft formatting codes. Leave empty to disable.
    pub header: String,
    /// Footer text displayed at the bottom of the tab list. Supports Minecraft formatting codes. Leave empty to disable.
    pub footer: String,
}
