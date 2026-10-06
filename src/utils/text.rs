//! Text formatting helpers.
//!
//! Provides utilities for converting [MiniMessage](https://docs.advntr.dev/minimessage/format.html)
//! formatted strings into Pumpkin `TextComponent` trees.

use pumpkin_plugin_api::text::TextComponent;

/// Parses a string containing `MiniMessage` tags (e.g. `<red>`, `<bold>`,
/// `<click:open_url:...>`) and returns a styled `TextComponent`.
///
/// Placeholder replacement must happen before parsing; this function only
/// interprets `MiniMessage` markup. If the input fails to parse, the error is
/// logged and the raw input is returned as a plain text component so a typo
/// in the config never breaks message delivery.
///
/// `minimessage-runtime` currently depends on `pumpkin-plugin-api` 0.1 while
/// pumpkinplus targets 0.2.  The WIT resource types are structurally identical
/// (same `pumpkin:plugin/text@0.1.0` interface), so we bridge them via the
/// server's JSON representation.
#[must_use]
pub fn parse_minimessage(input: &str) -> TextComponent {
    match minimessage_runtime::deserialize(input) {
        Ok(comp) => {
            let json = comp.to_json();
            TextComponent::from_json(&json).unwrap_or_else(|err| {
                tracing::warn!("Failed to bridge MiniMessage component from JSON: {err}");
                TextComponent::text(input)
            })
        }
        Err(err) => {
            tracing::warn!("Failed to parse MiniMessage input {input:?}: {err}");
            TextComponent::text(input)
        }
    }
}
