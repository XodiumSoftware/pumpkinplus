//! Dialog system for `PumpkinPlus`.
//!
//! Provides a contract for building interactive dialogs using the Pumpkin
//! Java dialog API. Dialogs can display rich content, collect user input,
//! and execute custom actions.

use pumpkin_plugin_api::java_dialog::Dialog;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::text::TextComponent;

/// A trait representing a dialog that can be shown to a player.
///
/// Each dialog implementation defines its layout, inputs, buttons,
/// and behavior when shown to a player.
pub trait DialogBuilder {
    /// Builds the dialog for the given player.
    ///
    /// This method constructs the [`Dialog`] instance with all
    /// configured elements, ready to be shown to the player.
    fn build(&self, player: &Player) -> Dialog;

    /// Shows the dialog to the given player.
    ///
    /// Builds the dialog using [`DialogBuilder::build`] and displays it
    /// to the player via the Java player's [`show_dialog`] method.
    ///
    /// # Note
    ///
    /// This only works for Java Edition players. Bedrock players
    /// will not see the dialog and will receive a message instead.
    fn show(&self, player: &Player) {
        // Try to get the Java player handle
        let java_player = player.as_java();

        match java_player {
            Some(java) => {
                // Java player - show the dialog
                java.show_dialog(self.build(player));
            }
            None => {
                // Bedrock player - send a message explaining dialogs aren't supported
                player.send_system_message(
                    TextComponent::text(
                        "Interactive dialogs are not supported on Bedrock Edition.",
                    ),
                    false,
                );
            }
        }
    }
}
