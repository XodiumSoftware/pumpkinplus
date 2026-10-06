//! Nickname dialog — set player nicknames via an interactive dialog.
//!
//! ## Dialog Layout
//!
//! - **Title**: "Nickname"
//! - **Body**: Instructions for configuring nicknames
//! - **Inputs**: Text field for entering a nickname (supports `MiniMessage`)
//! - **Actions**: "Discard" (close without saving), "Save" (apply nickname)
//!
//! ## Behavior
//!
//! - **Empty input** — clears the player's nickname
//! - **Non-empty input** — sets the player's nickname (supports `MiniMessage`)
//!
//! ## `MiniMessage` Support
//!
//! Nicknames support [MiniMessage](https://docs.advntr.dev/minimessage/format.html)
//! formatting tags (e.g. `<red><bold>Illyrius`).
//!
//! ## Platform Support
//!
//! This dialog system is Java Edition only. Bedrock players will receive
//! a chat message explaining dialogs are not supported.

use crate::dialogs::dialog::DialogBuilder;
use crate::modules::mechanics::player::nickname::{DATA_NAMESPACE, NICKNAME_KEY, update_player};
use crate::utils::text::parse_minimessage;
use pumpkin_plugin_api::PersistentDataHolder;
use pumpkin_plugin_api::java_dialog::{
    Action, ActionButton, CustomClickAction, Dialog, DialogBody, DialogInput, DialogInputText,
    DialogType,
};
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::text::TextComponent;

/// Dialog identifier for the nickname save action.
pub const NICKNAME_SAVE_ACTION: &str = "pumpkinplus:nickname_save";

/// Dialog for configuring a player's nickname.
pub struct NicknameDialog;

impl DialogBuilder for NicknameDialog {
    fn build(&self, player: &Player) -> Dialog {
        // Get current nickname or default to empty string
        let current_nickname = player
            .get_string(DATA_NAMESPACE, NICKNAME_KEY)
            .unwrap_or_default();

        Dialog {
            title: TextComponent::text("Nickname"),
            type_: DialogType::Confirmation,
            body: vec![
                DialogBody::PlainMessage(parse_minimessage(
                    "<yellow>1.</yellow> Configure your nickname at:\n\
                    <aqua>www.birdflop.com/resources/rgb/</aqua>",
                )),
                DialogBody::PlainMessage(parse_minimessage(
                    "<yellow>2.</yellow> Set output format to: <green>MiniMessage</green>",
                )),
                DialogBody::PlainMessage(parse_minimessage(
                    "<yellow>3.</yellow> Copy output from the site → paste into the input below.",
                )),
            ],
            inputs: vec![DialogInput::Text(DialogInputText {
                label: TextComponent::text("Enter nickname"),
                placeholder: parse_minimessage("<gray>Your nickname here</gray>"),
                default_value: current_nickname,
            })],
            buttons: vec![
                // Discard button
                ActionButton {
                    text: TextComponent::text("Discard"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: "discard".to_string(),
                        payload: None,
                    }),
                },
                // Save button
                ActionButton {
                    text: TextComponent::text("Save"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: NICKNAME_SAVE_ACTION.to_string(),
                        payload: None,
                    }),
                },
            ],
            links: vec![],
            after_action: None,
            can_close_with_escape: true,
            external_title: None,
        }
    }
}

/// Handles the nickname dialog save action.
///
/// This function is called when the player clicks "Save" in the nickname
/// dialog. It retrieves the input value, applies the nickname, and updates
/// the player's display name.
pub fn handle_save_action(player: &Player, payload: Option<&[u8]>) {
    // Extract nickname from payload (JSON string array)
    // The payload format from forms is: ["nickname_value"]
    let nickname = payload
        .as_ref()
        .and_then(|bytes| String::from_utf8(bytes.to_vec()).ok())
        .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
        .and_then(|value| {
            value
                .as_array()?
                .first()?
                .as_str()
                .map(std::string::ToString::to_string)
        })
        .unwrap_or_default();

    let trimmed = nickname.trim();

    if trimmed.is_empty() {
        player.remove_custom_data(DATA_NAMESPACE, NICKNAME_KEY);
        update_player(player, None);
        player.send_system_message(TextComponent::text("Nickname cleared."), false);
    } else {
        player.set_string(DATA_NAMESPACE, NICKNAME_KEY, trimmed);
        update_player(player, Some(trimmed));
        player.send_system_message(
            TextComponent::text(&format!("Nickname updated to: {trimmed}")),
            false,
        );
    }
}
