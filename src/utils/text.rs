//! Text formatting helpers.
//!
//! Provides utilities for converting [MiniMessage](https://docs.advntr.dev/minimessage/format.html)
//! formatted strings into Pumpkin `TextComponent` trees.

use std::borrow::Cow;
use std::str::FromStr;

use minimessage_impl::parser::{Expression, Node, Parser};
use minimessage_impl::style::{ClickEvent, Decoration, HoverEvent, NamedColor, Special};
use minimessage_impl::tokenizer::Tokenizer;
use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::text::TextComponent;

/// Parses a string containing `MiniMessage` tags (e.g. `<red>`, `<bold>`,
/// `<click:open_url:...>`) and returns a styled `TextComponent`.
///
/// Text placeholders written as `{name}` (Adventure's interpolation syntax) are
/// rendered as their literal source text so typos stay visible instead of being
/// silently dropped. Substituting a styled player name is handled separately by
/// [`parse_with_player_component`](crate::utils::placeholders::parse_with_player_component).
///
/// If the input fails to parse, the error is logged and the raw input is
/// returned as a plain text component so a typo in the config never breaks
/// message delivery.
#[must_use]
pub(crate) fn parse_minimessage(input: &str) -> TextComponent {
    let nodes: Vec<Node<'_>> = Parser::new(Tokenizer::new(input))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| {
            tracing::warn!("Failed to parse MiniMessage input {input:?}: {err}");
            vec![Node::Text(Cow::Borrowed(input))]
        });

    let mut root = TextComponent::text("");
    for node in &nodes {
        root = root.add_child(node_to_component(node));
    }
    root
}

/// Recursively converts a [`Node`] from `minimessage-impl` into a [`TextComponent`].
fn node_to_component(node: &Node<'_>) -> TextComponent {
    match node {
        Node::Text(text) => TextComponent::text(text.as_ref()),

        Node::Expression(Expression::Unnamed) => TextComponent::text(""),

        Node::Expression(Expression::Named(name)) => TextComponent::text(&format!("{{{name}}}")),

        Node::Element {
            tag,
            tag_descriptors,
            children,
        } => {
            let mut comp = TextComponent::text("");
            let mut matched = false;

            if let Ok(decoration) = Decoration::from_str(tag) {
                matched = true;
                comp = match decoration {
                    Decoration::Bold => comp.bold(true),
                    Decoration::Italic => comp.italic(true),
                    Decoration::Underlined => comp.underlined(true),
                    Decoration::Strikethrough => comp.strikethrough(true),
                    Decoration::Obfuscated => comp.obfuscated(true),
                };
            }

            if !matched {
                match Special::from_descriptor(tag, tag_descriptors.clone()) {
                    Ok(special) => {
                        matched = true;
                        comp = apply_special(comp, special);
                    }
                    Err(minimessage_impl::style::SpecialError::NotFound(_)) => {}
                    Err(err) => {
                        tracing::warn!("Unknown MiniMessage tag {tag:?}: {err}");
                        return TextComponent::text("");
                    }
                }
            }

            if !matched && let Ok(color) = NamedColor::from_str(tag) {
                matched = true;
                comp = comp.color_named(map_named_color(color));
            }

            if !matched {
                tracing::warn!("Unknown MiniMessage tag: {tag:?}");
                return TextComponent::text("");
            }

            for child in children {
                comp = comp.add_child(node_to_component(child));
            }

            comp
        }
    }
}

fn apply_special(comp: TextComponent, special: Special) -> TextComponent {
    match special {
        Special::Click(click) => match click {
            ClickEvent::OpenUrl(url) => comp.click_open_url(&url),
            ClickEvent::RunCommand(cmd) => comp.click_run_command(&cmd),
            ClickEvent::SuggestCommand(cmd) => comp.click_suggest_command(&cmd),
            ClickEvent::CopyToClipboard(text) => comp.click_copy_to_clipboard(&text),
            ClickEvent::__Empty => comp,
        },
        Special::Hover(hover) => match hover {
            HoverEvent::ShowText(text) => comp.hover_show_text(TextComponent::text(&text)),
            HoverEvent::ShowItem(item) => comp.hover_show_item(&item),
            HoverEvent::ShowEntity {
                entity_type,
                id,
                name,
            } => comp.hover_show_entity(&entity_type, &id, name.map(|n| TextComponent::text(&n))),
            HoverEvent::__Empty => comp,
        },
        Special::Color(color) => comp.color_rgb(RgbColor {
            r: color.0,
            g: color.1,
            b: color.2,
        }),
        Special::Rainbow(_rainbow) => {
            tracing::warn!("MiniMessage <rainbow> tag is not yet supported");
            comp
        }
    }
}

fn map_named_color(color: NamedColor) -> pumpkin_plugin_api::common::NamedColor {
    use pumpkin_plugin_api::common::NamedColor as PNC;
    match color {
        NamedColor::Black => PNC::Black,
        NamedColor::DarkBlue => PNC::DarkBlue,
        NamedColor::DarkGreen => PNC::DarkGreen,
        NamedColor::DarkAqua => PNC::DarkAqua,
        NamedColor::DarkRed => PNC::DarkRed,
        NamedColor::DarkPurple => PNC::DarkPurple,
        NamedColor::Gold => PNC::Gold,
        NamedColor::Gray => PNC::Gray,
        NamedColor::DarkGray => PNC::DarkGray,
        NamedColor::Blue => PNC::Blue,
        NamedColor::Green => PNC::Green,
        NamedColor::Aqua => PNC::Aqua,
        NamedColor::Red => PNC::Red,
        NamedColor::LightPurple => PNC::LightPurple,
        NamedColor::Yellow => PNC::Yellow,
        NamedColor::White => PNC::White,
    }
}

/// Parses a `MiniMessage`-style color descriptor into an [`RgbColor`].
///
/// Accepts any of:
/// - A named color: `red`, `dark_aqua`, ...
/// - A bare or prefixed hex triplet: `FF8800`, `#FF8800`
/// - A `MiniMessage`-wrapped variant of either: `<red>`, `<#FF8800>`, `<color:#FF8800>`, `<colour:#FF8800>`, `<c:#FF8800>`
///
/// Returns `None` if the descriptor is blank or unrecognized.
#[must_use]
pub(crate) fn parse_color_arg(input: &str) -> Option<RgbColor> {
    use minimessage_impl::style::color::Color;

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let unwrapped = trimmed
        .strip_prefix('<')
        .and_then(|s| s.strip_suffix('>'))
        .map_or(trimmed, |inner| inner.trim());

    let descriptor = match unwrapped.split_once(':') {
        Some(("color" | "colour" | "c", value)) => value.trim(),
        _ => unwrapped,
    };

    if let Ok(named) = NamedColor::from_str(descriptor) {
        return Some(named_color_to_rgb(named));
    }

    Color::from_str(descriptor)
        .ok()
        .map(|Color(r, g, b)| RgbColor { r, g, b })
}

/// Converts a `MiniMessage` [`NamedColor`] to its vanilla RGB value.
fn named_color_to_rgb(color: NamedColor) -> RgbColor {
    let (r, g, b) = match color {
        NamedColor::Black => (0x00, 0x00, 0x00),
        NamedColor::DarkBlue => (0x00, 0x00, 0xAA),
        NamedColor::DarkGreen => (0x00, 0xAA, 0x00),
        NamedColor::DarkAqua => (0x00, 0xAA, 0xAA),
        NamedColor::DarkRed => (0xAA, 0x00, 0x00),
        NamedColor::DarkPurple => (0xAA, 0x00, 0xAA),
        NamedColor::Gold => (0xFF, 0xAA, 0x00),
        NamedColor::Gray => (0xAA, 0xAA, 0xAA),
        NamedColor::DarkGray => (0x55, 0x55, 0x55),
        NamedColor::Blue => (0x55, 0x55, 0xFF),
        NamedColor::Green => (0x55, 0xFF, 0x55),
        NamedColor::Aqua => (0x55, 0xFF, 0xFF),
        NamedColor::Red => (0xFF, 0x55, 0x55),
        NamedColor::LightPurple => (0xFF, 0x55, 0xFF),
        NamedColor::Yellow => (0xFF, 0xFF, 0x55),
        NamedColor::White => (0xFF, 0xFF, 0xFF),
    };
    RgbColor { r, g, b }
}
