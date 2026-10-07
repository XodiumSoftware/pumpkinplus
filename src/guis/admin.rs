//! Admin menu GUI.
//!
//! Provides the main admin menu opened by the `/pumpkinplus` command. Built with
//! [`GuiBuilder`] using styled items, lore, and interaction rules.
//!
//! Click handling must be wired separately via `InventoryClickEvent` if you
//! want buttons to do anything.
//!
//! ## Example
//!
//! ```rust,ignore
//! use pumpkin_plugin_api::player::Player;
//! use crate::guis::admin::build_admin_menu;
//!
//! fn open_menu(player: &Player) {
//!     let gui = build_admin_menu();
//!     player.open_gui(gui);
//! }
//! ```

use pumpkin_plugin_api::Item;
use pumpkin_plugin_api::gui;
use pumpkin_plugin_api::screens_wit::Screen;
use pumpkin_plugin_api::text::TextComponent;

use crate::GuiBuilder;
use crate::guis::gui::ItemBuilder;

/// Registry ID for the admin menu (used by click dispatch).
pub const ADMIN_MENU_ID: &str = "admin_menu";

/// Builds the admin menu GUI.
///
/// Layout (hopper screen, 5 slots):
///
/// ```text
/// [0] [1] [2] [3] [4]
/// ```
///
/// - `1` — Teleport compass
/// - `2` — Info book
/// - `3` — Close barrier
/// - All other slots — black stained-glass panes as filler
///
/// Grabbing and placing items are disabled so the GUI behaves like a menu.
#[must_use]
pub fn build_admin_menu() -> gui::Gui {
    GuiBuilder::new(Screen::Hopper)
        .id(ADMIN_MENU_ID)
        .title("<dark_gray><bold>Admin Menu")
        .allow_grab(false)
        .allow_put(false)
        .item(
            1,
            ItemBuilder::new(Item::Compass)
                .title("<green><bold>Teleport")
                .lore(&["<gray>Click to open the", "<gray>teleport menu."])
                .on_click(|event| {
                    event.player.send_system_message(
                        TextComponent::text("Teleport menu not yet wired up."),
                        false,
                    );
                    event
                })
                .build(),
        )
        .item(
            2,
            ItemBuilder::new(Item::Book)
                .title("<yellow><bold>Server Info")
                .lore(&["<gray>View rules, links,", "<gray>and other server info."])
                .on_click(|event| {
                    event.player.send_system_message(
                        TextComponent::text("Server info not yet wired up."),
                        false,
                    );
                    event
                })
                .build(),
        )
        .item(
            3,
            ItemBuilder::new(Item::Barrier)
                .title("<red><bold>Close")
                .lore(&["<gray>Click to close this menu."])
                .on_click(|event| {
                    // TODO: Close the inventory once the API exposes a close method.
                    event.player.send_system_message(
                        TextComponent::text("Close button pressed (close API pending)."),
                        false,
                    );
                    event
                })
                .build(),
        )
        .fill_empty(Item::BlackStainedGlassPane)
        .build()
}
