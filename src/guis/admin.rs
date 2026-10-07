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
/// Layout (9x3 chest screen, 27 slots):
///
/// ```text
/// [0] [1] [2] [3] [4] [5] [6] [7] [8]
/// [9] [10][11][12][13][14][15][16][17]
/// [18][19][20][21][22][23][24][25][26]
/// ```
///
/// - `11` — Teleport compass
/// - `13` — Info book
/// - `15` — Close barrier
/// - All other slots — black stained-glass panes as filler
///
/// Grabbing and placing items are disabled so the GUI behaves like a menu.
#[must_use]
pub fn build_admin_menu() -> gui::Gui {
    GuiBuilder::new(Screen::Generic9x3)
        .title("<dark_gray><bold>Admin Menu")
        .allow_grab(false)
        .allow_put(false)
        .lore_item(
            11,
            Item::Compass,
            "<green><bold>Teleport",
            &["<gray>Click to open the", "<gray>teleport menu."],
        )
        .lore_item(
            13,
            Item::Book,
            "<yellow><bold>Server Info",
            &["<gray>View rules, links,", "<gray>and other server info."],
        )
        .lore_item(
            15,
            Item::Barrier,
            "<red><bold>Close",
            &["<gray>Click to close this menu."],
        )
        .fill_empty(Item::BlackStainedGlassPane)
        .build()
}
