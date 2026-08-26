//! Text and legacy formatting helpers.
//!
//! Provides utilities for converting strings with Minecraft `&` color/formatting
//! codes into Pumpkin `TextComponent` trees.

use pumpkin_plugin_api::text::{NamedColor, TextComponent};

/// Parses a string containing legacy `&` color/formatting codes and returns a
/// `TextComponent` with styled children. A plain text component is returned if
/// no codes are present.
///
/// Supported codes match the standard Minecraft color/formatting codes:
/// `0-9`, `a-f` for colors; `k` obfuscated, `l` bold, `m` strikethrough,
/// `n` underlined, `o` italic, `r` reset.
#[must_use]
pub fn parse_legacy_text(input: &str) -> TextComponent {
    let mut root = TextComponent::text("");
    let mut current_text = String::new();
    let mut current = TextComponent::text("");
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '&' {
            // Flush accumulated text before applying the new code.
            if !current_text.is_empty() {
                current = current.add_text(&current_text);
                current_text.clear();
                root = root.add_child(current);
                current = TextComponent::text("");
            }

            let Some(code) = chars.next() else {
                break;
            };

            let code_lower = code.to_ascii_lowercase();
            match code_lower {
                'r' => {
                    current = TextComponent::text("");
                }
                'k' => {
                    current = current.obfuscated(true);
                }
                'l' => {
                    current = current.bold(true);
                }
                'm' => {
                    current = current.strikethrough(true);
                }
                'n' => {
                    current = current.underlined(true);
                }
                'o' => {
                    current = current.italic(true);
                }
                _ => {
                    if let Some(color) = color_from_code(code_lower) {
                        current = TextComponent::text("");
                        current = current.color_named(color);
                    }
                    // Unknown codes are ignored.
                }
            }
        } else {
            current_text.push(ch);
        }
    }

    if !current_text.is_empty() {
        current = current.add_text(&current_text);
    }
    // Only add the final child if it has text or styling.
    if !current.get_text().is_empty() || has_style(&current) {
        root = root.add_child(current);
    }

    root
}

/// Maps a legacy formatting code character to a `NamedColor`.
#[must_use]
pub fn color_from_code(code: char) -> Option<NamedColor> {
    Some(match code {
        '0' => NamedColor::Black,
        '1' => NamedColor::DarkBlue,
        '2' => NamedColor::DarkGreen,
        '3' => NamedColor::DarkAqua,
        '4' => NamedColor::DarkRed,
        '5' => NamedColor::DarkPurple,
        '6' => NamedColor::Gold,
        '7' => NamedColor::Gray,
        '8' => NamedColor::DarkGray,
        '9' => NamedColor::Blue,
        'a' => NamedColor::Green,
        'b' => NamedColor::Aqua,
        'c' => NamedColor::Red,
        'd' => NamedColor::LightPurple,
        'e' => NamedColor::Yellow,
        'f' => NamedColor::White,
        _ => return None,
    })
}

/// Returns true if the text component has any style applied.
///
/// `TextComponent` doesn't expose style getters, so this infers it from the
/// encoded NBT length. This is a heuristic; a better API would expose style
/// flags directly.
#[must_use]
pub fn has_style(component: &TextComponent) -> bool {
    component.encode().len() > 1
}
