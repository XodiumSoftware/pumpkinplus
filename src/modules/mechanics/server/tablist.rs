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
use crate::mechanics::mechanic::Mechanic;
use crate::utils::placeholders::replace_all_placeholders;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerJoinEvent, PlayerLeaveEvent,
};
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Refresh interval for live tab-list placeholders such as `{tps}` and `{mspt}`.
/// 40 ticks = 2 seconds.
const REFRESH_TICKS: u64 = 40;

/// Handles tab-list mechanics, including custom messages.
#[derive(Default)]
pub struct Tablist;

impl Mechanic for Tablist {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.tablist.enabled)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerJoinEvent, _>(Tablist, EventPriority::Normal, true)
            .expect("failed to register tablist event handler");
        context
            .register_event_handler::<PlayerLeaveEvent, _>(Tablist, EventPriority::Normal, true)
            .expect("failed to register tablist leave event handler");

        // Keep live placeholders (TPS/MSPT) current for all online players.
        context.schedule_repeating_task(REFRESH_TICKS, REFRESH_TICKS, |server| {
            Self::update_tablist_for_all_players(&server);
        });
    }
}

impl Tablist {
    /// Applies the configured header and footer to a single player,
    /// resolving placeholders for that player.
    fn update_tablist_for_player(config: &TablistConfig, server: &Server, player: &Player) {
        let header = replace_all_placeholders(&config.header, server, player);
        let footer = replace_all_placeholders(&config.footer, server, player);
        player
            .set_tab_list_header_footer(TextComponent::text(&header), TextComponent::text(&footer));
    }

    /// Refreshes the tab list header and footer for every online player.
    fn update_tablist_for_all_players(server: &Server) {
        let config: TablistConfig = ConfigManager::get()
            .map(|cm| cm.tablist)
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
        let config: TablistConfig = ConfigManager::get()
            .map(|cm| cm.tablist)
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
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TablistConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Header text displayed at the top of the tab list. Supports Minecraft formatting codes. Leave empty to disable.
    pub header: String,
    /// Footer text displayed at the bottom of the tab list. Supports Minecraft formatting codes. Leave empty to disable.
    pub footer: String,
}
