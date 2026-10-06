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
/// Placeholder replacement must happen before parsing; this function only
/// interprets `MiniMessage` markup. If the input fails to parse, the error is
/// logged and the raw input is returned as a plain text component so a typo
/// in the config never breaks message delivery.
#[must_use]
pub fn parse_minimessage(input: &str) -> TextComponent {
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

        Node::Expression(Expression::Unnamed | Expression::Named(_)) => {
            // Expressions are only meaningful when formatting args are provided.
            // We don't pass args, so these become literal text.
            TextComponent::text("")
        }

        Node::Element {
            tag,
            tag_descriptors,
            children,
        } => {
            let mut comp = TextComponent::text("");
            let mut matched = false;

            // Decorations (bold, italic, etc.)
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

            // Special tags (click, hover, color, rainbow)
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

            // Named colors
            if !matched && let Ok(color) = NamedColor::from_str(tag) {
                matched = true;
                comp = comp.color_named(map_named_color(color));
            }

            // Unknown tag — treat as literal text
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
            // Rainbow is applied per-character, which requires splitting text
            // into per-char children — not yet supported in our simple converter.
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
