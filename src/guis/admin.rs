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

use crate::GuiBuilder;

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
        .title("<dark_gray><bold>Admin Menu")
        .allow_grab(false)
        .allow_put(false)
        .lore_item(
            1,
            Item::Compass,
            "<green><bold>Teleport",
            &["<gray>Click to open the", "<gray>teleport menu."],
        )
        .lore_item(
            2,
            Item::Book,
            "<yellow><bold>Server Info",
            &["<gray>View rules, links,", "<gray>and other server info."],
        )
        .lore_item(
            3,
            Item::Barrier,
            "<red><bold>Close",
            &["<gray>Click to close this menu."],
        )
        .fill_empty(Item::BlackStainedGlassPane)
        .build()
}
