//! Placeholder replacement helpers for player- and server-facing strings.
//!
//! The placeholder vocabulary mirrors the tables documented in the message and
//! tablist modules.

use pumpkin_plugin_api::{Server, player::Player, text::TextComponent};
use tracing::warn;

use crate::utils::text::parse_minimessage;

/// Replaces `{player}` with the player's current display name.
#[must_use]
pub(crate) fn replace_player_placeholders(text: &str, player: &Player) -> String {
    text.replace("{player}", &player.get_display_name().get_text())
}

/// Replaces `{online}`, `{tps}`, and `{mspt}` with live server data.
#[must_use]
pub(crate) fn replace_server_placeholders(text: &str, server: &Server) -> String {
    text.replace("{online}", &server.get_player_count().to_string())
        .replace("{tps}", &format!("{:.1}", server.get_tps()))
        .replace("{mspt}", &format!("{:.1}", server.get_mspt()))
}

/// Replaces all known player and server placeholders.
#[must_use]
pub(crate) fn replace_all_placeholders(text: &str, server: &Server, player: &Player) -> String {
    replace_server_placeholders(&replace_player_placeholders(text, player), server)
}

/// Parses `MiniMessage` `format` and substitutes `{player}` with the player's display
/// name component, preserving the nickname's own explicit styling.
///
/// Semantics match Paper/Adventure: the surrounding `MiniMessage` styles (color,
/// decorations) apply to `{player}` *unless* the player's display name already
/// carries its own explicit style for a given property, in which case the
/// nickname's style wins for that property.
///
/// Implementation: `format` is parsed with the placeholder as literal text, the
/// resulting component tree is serialized to Minecraft's JSON text format, every
/// occurrence of the placeholder inside a `"text":"..."` leaf is rewritten as
/// an inline array of `[text_before, player_component, text_after]`, and the
/// result is deserialized back into a `TextComponent`. The rewrite happens
/// inside the JSON's `extra` array of the surrounding styled node, so styles
/// still inherit.
///
/// Falls back to parsing `format` with the placeholder replaced by the plain-text
/// display name if any JSON step fails.
#[must_use]
pub(crate) fn parse_with_player_component(format: &str, player: &Player) -> TextComponent {
    if !format.contains("{player}") {
        return parse_minimessage(format);
    }

    let parsed = parse_minimessage(format);
    let json = parsed.to_json();
    let player_json = player.get_display_name().to_json();

    let Some(substituted) = substitute_text_placeholder(&json, "{player}", &player_json) else {
        return parsed;
    };

    match TextComponent::from_json(&substituted) {
        Ok(component) => component,
        Err(err) => {
            warn!("Failed to deserialize substituted text component: {err}");
            parse_minimessage(&format.replace("{player}", &player.get_display_name().get_text()))
        }
    }
}

/// Rewrites every `"text":"...PLACEHOLDER..."` occurrence in `json` so the
/// placeholder text is replaced by `replacement_json` inserted as a sibling in the
/// parent's `extra` array.
///
/// Returns `None` if no substitution was made.
fn substitute_text_placeholder(
    json: &str,
    placeholder: &str,
    replacement_json: &str,
) -> Option<String> {
    let mut out = String::with_capacity(json.len() + replacement_json.len());
    let mut rest = json;
    let mut any_replaced = false;

    while let Some(idx) = rest.find(placeholder) {
        let Some((str_start, str_end)) = find_enclosing_json_string(rest, idx) else {
            out.push_str(&rest[..idx]);
            out.push_str(placeholder);
            rest = &rest[idx + placeholder.len()..];
            continue;
        };

        if !is_text_value_at(rest, str_start) {
            out.push_str(&rest[..=str_end]);
            rest = &rest[str_end + 1..];
            continue;
        }

        let content = &rest[str_start + 1..str_end];
        let pieces: Vec<&str> = content.split(placeholder).collect();

        if pieces.len() == 1 {
            out.push_str(&rest[..=str_end]);
            rest = &rest[str_end + 1..];
            continue;
        }

        out.push_str(&rest[..str_start]);

        out.push('"');
        out.push_str(pieces[0]);
        out.push_str("\",\"extra\":[");

        for (i, piece) in pieces.iter().skip(1).enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(replacement_json);
            if !piece.is_empty() {
                out.push_str(",{\"text\":\"");
                out.push_str(piece);
                out.push_str("\"}");
            }
        }
        out.push(']');

        any_replaced = true;
        rest = &rest[str_end + 1..];
    }

    if !any_replaced {
        return None;
    }

    out.push_str(rest);
    Some(out)
}

/// Returns `true` if the JSON at string-start index `str_start` is the value of a
/// `"text":` key, i.e. immediately preceded by `"text"` and `:` (allowing
/// whitespace).
fn is_text_value_at(json: &str, str_start: usize) -> bool {
    let bytes = json.as_bytes();
    if str_start < 7 {
        return false;
    }

    let mut i = str_start;
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    if i == 0 || bytes[i - 1] != b':' {
        return false;
    }
    i -= 1;
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    if i == 0 || bytes[i - 1] != b'"' {
        return false;
    }
    i -= 1;
    if i < 4 {
        return false;
    }
    &json[i - 4..i] == "text"
}

/// Finds the start and end byte indices of the JSON string that contains byte
/// `pos`. Returns `(start, end)` where `start` is the index of the opening quote
/// and `end` is the index of the closing quote.
///
/// Returns `None` if `pos` is not inside a JSON string.
fn find_enclosing_json_string(json: &str, pos: usize) -> Option<(usize, usize)> {
    let bytes = json.as_bytes();

    let mut start = pos;
    loop {
        if start == 0 {
            return None;
        }
        start -= 1;
        if bytes[start] == b'"' {
            let mut backslashes = 0;
            let mut i = start;
            while i > 0 && bytes[i - 1] == b'\\' {
                backslashes += 1;
                i -= 1;
            }
            if backslashes % 2 == 0 {
                break;
            }
        }
    }

    let mut end = pos;
    loop {
        if end >= bytes.len() {
            return None;
        }
        if bytes[end] == b'"' {
            let mut backslashes = 0;
            let mut i = end;
            while i > 0 && bytes[i - 1] == b'\\' {
                backslashes += 1;
                i -= 1;
            }
            if backslashes % 2 == 0 {
                break;
            }
        }
        end += 1;
    }

    Some((start, end))
}
