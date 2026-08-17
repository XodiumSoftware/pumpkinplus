//! Placeholder replacement helpers for player- and server-facing strings.
//!
//! The placeholder vocabulary mirrors the tables documented in the message, chat,
//! and tablist modules.

use pumpkin_plugin_api::{Server, player::Player};

/// Replaces a set of `key` → `value` placeholders in `text`.
#[must_use]
pub fn replace_placeholders(text: &str, replacements: &[(&str, &str)]) -> String {
    replacements
        .iter()
        .fold(text.to_string(), |acc, (key, value)| {
            acc.replace(key, value)
        })
}

/// Replaces `{player}` with the player's current display name.
#[must_use]
pub fn replace_player_placeholders(text: &str, player: &Player) -> String {
    text.replace("{player}", &player.get_display_name().get_text())
}

/// Replaces `{online}`, `{tps}`, and `{mspt}` with live server data.
#[must_use]
pub fn replace_server_placeholders(text: &str, server: &Server) -> String {
    text.replace("{online}", &server.get_player_count().to_string())
        .replace("{tps}", &format!("{:.1}", server.get_tps()))
        .replace("{mspt}", &format!("{:.1}", server.get_mspt()))
}

/// Replaces all known player and server placeholders.
#[must_use]
pub fn replace_all_placeholders(text: &str, server: &Server, player: &Player) -> String {
    replace_server_placeholders(&replace_player_placeholders(text, player), server)
}
