//! Admin menu GUI.
//!
//! Provides the main admin menu opened by the `/pumpkinplus` command. Built with
//! [`GuiBuilder`] using styled items, lore, and interaction rules.
//!
//! The menu shows buttons for toggling the master module groups
//! (`mechanics`, `enchantments`, `recipes`) on/off. Clicks persist the new
//! state to `config.toml`. Re-opening the menu reflects the latest state.
//!
//! Note: toggles only take effect on the next plugin load (server restart
//! or plugin reload). This menu does not unregister live event handlers or
//! commands.
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
use pumpkin_plugin_api::{screens_wit::Screen, text::TextComponent};

use crate::config::ConfigManager;
use crate::guis::gui::{GuiBuilder, ItemBuilder};

/// Registry ID for the admin menu (used by click dispatch).
pub const ADMIN_MENU_ID: &str = "admin_menu";

/// Label for the mechanics module group shown in the menu.
const MECHANICS_LABEL: &str = "Mechanics";
/// Label for the enchantments module group shown in the menu.
const ENCHANTMENTS_LABEL: &str = "Enchantments";
/// Label for the recipes module group shown in the menu.
const RECIPES_LABEL: &str = "Recipes";

/// Builds the admin menu GUI.
///
/// Layout (hopper screen, 5 slots):
///
/// ```text
/// [0] [1] [2] [3] [4]
/// ```
///
/// - `1` — Toggle `mechanics`
/// - `2` — Toggle `enchantments`
/// - `3` — Toggle `recipes`
/// - All other slots — black stained-glass panes as filler
///
/// Grabbing and placing items are disabled so the GUI behaves like a menu.
#[must_use]
pub fn build_admin_menu() -> gui::Gui {
    let config = ConfigManager::get().unwrap_or_default();
    let toggles = &config.modules;

    GuiBuilder::new(Screen::Hopper)
        .id(ADMIN_MENU_ID)
        .title("<dark_gray><bold>Admin Menu")
        .allow_grab(false)
        .allow_put(false)
        .item(
            1,
            toggle_button(
                Item::CommandBlock,
                MECHANICS_LABEL,
                toggles.mechanics,
                |t| t.mechanics = !t.mechanics,
            ),
        )
        .item(
            2,
            toggle_button(
                Item::EnchantingTable,
                ENCHANTMENTS_LABEL,
                toggles.enchantments,
                |t| t.enchantments = !t.enchantments,
            ),
        )
        .item(
            3,
            toggle_button(Item::CraftingTable, RECIPES_LABEL, toggles.recipes, |t| {
                t.recipes = !t.recipes;
            }),
        )
        .fill_empty(Item::BlackStainedGlassPane)
        .build()
}

/// Builds a toggle button for a single module group.
///
/// The title shows the group name and current state (green when enabled,
/// red when disabled). Clicking flips the toggle via `apply`, persists the
/// new `config.toml`, sends feedback to the player, and re-opens the menu
/// so the updated state is visible immediately.
///
/// `apply` should mutate the passed `ModuleToggles` to flip the relevant
/// boolean field.
fn toggle_button(
    item: impl pumpkin_plugin_api::IntoItemKey,
    label: &'static str,
    enabled: bool,
    apply: fn(&mut crate::config::ModuleToggles),
) -> crate::guis::gui::GuiItem {
    let state_text = if enabled {
        "<green>Enabled"
    } else {
        "<red>Disabled"
    };
    let title = format!("<bold>{label}</bold> — {state_text}");
    let action_verb = if enabled { "Disable" } else { "Enable" };
    let lore_lines = [
        format!("<gray>Click to {}", action_verb.to_lowercase()),
        "<dark_gray>Takes effect on restart.".to_string(),
    ];
    let lore_refs: Vec<&str> = lore_lines.iter().map(String::as_str).collect();

    ItemBuilder::new(item)
        .title(&title)
        .lore(&lore_refs)
        .on_click(move |event| {
            ConfigManager::save_toggles(apply);
            let new_state = !enabled;
            let msg = if new_state {
                format!("{label} enabled.")
            } else {
                format!("{label} disabled.")
            };
            event
                .player
                .send_system_message(TextComponent::text(&msg), false);

            event
        })
        .build()
}
