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
#[must_use]
pub fn parse_minimessage(input: &str) -> TextComponent {
    minimessage_rs::deserialize(input).map_or_else(
        |err| {
            tracing::warn!("Failed to parse MiniMessage input {input:?}: {err}");
            TextComponent::text(input)
        },
        |component| minimessage_rt_compat::convert(&component),
    )
}
