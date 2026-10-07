//! GUI builder utility for creating custom inventory-based menus.
//!
//! Provides a fluent [`GuiBuilder`] for constructing [`Gui`] instances with
//! title, size, items, and interaction rules, plus a [`GuiSession`] helper for
//! tracking which players have which GUI open.
//!
//! ## Example
//!
//! ```rust,ignore
//! use pumpkin_plugin_api::gui::Gui;
//! use pumpkin_plugin_api::screens_wit::Screen;
//! use crate::guis::gui::{GuiBuilder, GuiSession};
//!
//! let gui = GuiBuilder::new(Screen::Generic9x3, "<gold>My Menu")
//!     .item(13, "minecraft:diamond", 1)
//!     .allow_grab(false)
//!     .allow_put(false)
//!     .build();
//!
//! player.open_gui(&gui);
//! ```

use pumpkin_plugin_api::gui;
use pumpkin_plugin_api::screens_wit::Screen;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{IntoItemKey, ItemStack, ItemStackExt};

/// A fluent builder for creating [`Gui`] instances.
///
/// The builder allows setting the GUI type, title, items, and interaction
/// permissions before constructing the final `Gui`.
#[allow(dead_code)]
pub struct GuiBuilder {
    /// The screen type (size/layout) of the GUI.
    screen: Screen,
    /// The title component displayed at the top of the GUI.
    title: TextComponent,
    /// Items to place in the GUI slots.
    items: Vec<(u32, ItemStack)>,
    /// Whether players can take items out of the GUI.
    allow_grab: bool,
    /// Whether players can put items into the GUI.
    allow_put: bool,
}

#[allow(dead_code)]
impl GuiBuilder {
    /// Creates a new builder for a GUI with the given screen type and title.
    ///
    /// The `title` is parsed as a `MiniMessage` string if it contains `<` tags;
    /// otherwise it is treated as plain text.
    #[must_use]
    pub fn new(screen: Screen, title: &str) -> Self {
        let title = if title.contains('<') {
            crate::utils::text::parse_minimessage(title)
        } else {
            TextComponent::text(title)
        };

        Self {
            screen,
            title,
            items: Vec::new(),
            allow_grab: true,
            allow_put: true,
        }
    }

    /// Sets whether players can take items out of the GUI.
    #[must_use]
    pub fn allow_grab(mut self, allow: bool) -> Self {
        self.allow_grab = allow;
        self
    }

    /// Sets whether players can put items into the GUI.
    #[must_use]
    pub fn allow_put(mut self, allow: bool) -> Self {
        self.allow_put = allow;
        self
    }

    /// Places an item in the specified slot.
    ///
    /// `slot` is the inventory index (0-based). `count` is clamped to the
    /// item's maximum stack size by the server.
    #[must_use]
    pub fn item(mut self, slot: u32, item: impl IntoItemKey, count: u8) -> Self {
        self.items.push((slot, ItemStack::of(item, count)));
        self
    }

    /// Places an item with a custom display name in the specified slot.
    ///
    /// The `name` is parsed as `MiniMessage` if it contains `<` tags.
    #[must_use]
    pub fn named_item(mut self, slot: u32, item: impl IntoItemKey, count: u8, name: &str) -> Self {
        let stack = ItemStack::of(item, count);
        let name_component = if name.contains('<') {
            crate::utils::text::parse_minimessage(name)
        } else {
            TextComponent::text(name)
        };
        stack.set_custom_name(Some(name_component));
        self.items.push((slot, stack));
        self
    }

    /// Places an item with a custom display name and lore in the specified slot.
    ///
    /// Both `name` and each line of `lore` are parsed as `MiniMessage` if they
    /// contain `<` tags; otherwise they are treated as plain text.
    #[must_use]
    pub fn lore_item(
        mut self,
        slot: u32,
        item: impl IntoItemKey,
        count: u8,
        name: &str,
        lore: &[&str],
    ) -> Self {
        let stack = ItemStack::of(item, count);
        let name_component = if name.contains('<') {
            crate::utils::text::parse_minimessage(name)
        } else {
            TextComponent::text(name)
        };
        stack.set_custom_name(Some(name_component));

        let lore_components: Vec<TextComponent> = lore
            .iter()
            .map(|line| {
                if line.contains('<') {
                    crate::utils::text::parse_minimessage(line)
                } else {
                    TextComponent::text(line)
                }
            })
            .collect();
        stack.set_lore(lore_components);

        self.items.push((slot, stack));
        self
    }

    /// Builds the [`Gui`] instance with all configured items and settings.
    #[must_use]
    pub fn build(self) -> gui::Gui {
        let gui = gui::Gui::new(self.screen, self.title);
        gui.set_allow_grab_items(self.allow_grab);
        gui.set_allow_put_items(self.allow_put);
        for (slot, stack) in self.items {
            gui.set_item(slot, stack);
        }
        gui
    }
}
