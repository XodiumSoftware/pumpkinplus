//! GUI builder utility for creating custom inventory-based menus.
//!
//! Provides a fluent [`GuiBuilder`] for constructing [`Gui`] instances with
//! title, size, items, and interaction rules.
//!
//! ## Item identifiers
//!
//! Item-taking methods accept any [`IntoItemKey`], which includes both the
//! typed [`Item`] enum (preferred — compile-time checked) and string keys like
//! `"minecraft:diamond"`. Prefer the enum when the item is a known vanilla
//! item; fall back to strings only for custom/modded items.
//!
//! ## Example
//!
//! ```rust,ignore
//! use pumpkin_plugin_api::screens_wit::Screen;
//! use pumpkin_plugin_api::Item;
//! use crate::guis::gui::GuiBuilder;
//!
//! let gui = GuiBuilder::new(Screen::Generic9x3)
//!     .title("<gold>My Menu")
//!     .item(13, Item::Diamond)
//!     .allow_grab(false)
//!     .allow_put(false)
//!     .build();
//!
//! player.open_gui(gui);
//! ```

use pumpkin_plugin_api::gui;
use pumpkin_plugin_api::screens_wit::Screen;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{IntoItemKey, ItemStack, ItemStackExt};

/// Extension trait providing slot-count lookup for [`Screen`] variants.
///
/// The WIT `Screen` enum doesn't expose how many slots each screen has, so
/// this trait fills in the well-known values from vanilla Minecraft.
trait ScreenSlots {
    /// Returns the number of inventory slots in this screen.
    fn slot_count(&self) -> u32;
}

impl ScreenSlots for Screen {
    fn slot_count(&self) -> u32 {
        match self {
            Screen::Generic9x1 | Screen::Generic3x3 | Screen::Crafter3x3 => 9,
            Screen::Generic9x2 => 18,
            Screen::Generic9x3 | Screen::ShulkerBox => 27,
            Screen::Generic9x4 => 36,
            Screen::Generic9x5 => 45,
            Screen::Generic9x6 => 54,
            Screen::Anvil | Screen::Smithing | Screen::Loom => 4,
            Screen::Beacon | Screen::Lectern => 1,
            Screen::BlastFurnace
            | Screen::Furnace
            | Screen::Smoker
            | Screen::Grindstone
            | Screen::Merchant
            | Screen::CartographyTable => 3,
            Screen::BrewingStand | Screen::Hopper => 5,
            Screen::Crafting => 10,
            Screen::Enchantment | Screen::Stonecutter => 2,
        }
    }
}

/// A fluent builder for creating [`Gui`] instances.
///
/// The builder allows setting the GUI type, title, items, and interaction
/// permissions before constructing the final `Gui`.
#[allow(dead_code)]
pub struct GuiBuilder {
    /// The screen type (size/layout) of the GUI.
    screen: Screen,
    /// The title component displayed at the top of the GUI.
    title: Option<TextComponent>,
    /// Items to place in the GUI slots.
    items: Vec<(u32, ItemStack)>,
    /// Whether players can take items out of the GUI.
    allow_grab: bool,
    /// Whether players can put items into the GUI.
    allow_put: bool,
}

#[allow(dead_code)]
impl GuiBuilder {
    /// Creates a new builder for a GUI with the given screen type.
    ///
    /// No title is set by default; call [`GuiBuilder::title`] to set one.
    #[must_use]
    pub fn new(screen: Screen) -> Self {
        Self {
            screen,
            title: None,
            items: Vec::new(),
            allow_grab: true,
            allow_put: true,
        }
    }

    /// Sets the GUI title.
    ///
    /// The `title` is parsed as a `MiniMessage` string. Plain text without tags
    /// passes through unchanged.
    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(crate::utils::text::parse_minimessage(title));
        self
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

    /// Places a pre-built [`ItemStack`] in the specified slot.
    ///
    /// This is the low-level primitive that all other slot methods build on.
    /// Use this when you need full control over the item's name, lore, stack
    /// size, or other data.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use pumpkin_plugin_api::Item;
    /// use pumpkin_plugin_api::text::TextComponent;
    ///
    /// let stack = ItemStack::of(Item::Compass, 1);
    /// stack.set_custom_name(Some(TextComponent::text("Menu")));
    /// builder.stack(11, stack)
    /// ```
    #[must_use]
    pub fn stack(mut self, slot: u32, stack: ItemStack) -> Self {
        self.items.push((slot, stack));
        self
    }

    /// Places an item in the specified slot.
    ///
    /// `slot` is the inventory index (0-based). Places a single item (count 1);
    /// use [`stack`](Self::stack) with a custom-built `ItemStack` to control
    /// the stack size.
    #[must_use]
    pub fn item(self, slot: u32, item: impl IntoItemKey) -> Self {
        self.stack(slot, ItemStack::of(item, 1))
    }

    /// Places an item with a custom display name in the specified slot.
    ///
    /// The `name` is parsed as a `MiniMessage` string. Plain text without tags
    /// passes through unchanged. Places a single item (count 1); use
    /// [`stack`](Self::stack) with a custom-built `ItemStack` to control the
    /// stack size.
    #[must_use]
    pub fn named_item(self, slot: u32, item: impl IntoItemKey, name: &str) -> Self {
        let stack = ItemStack::of(item, 1);
        stack.set_custom_name(Some(crate::utils::text::parse_minimessage(name)));
        self.stack(slot, stack)
    }

    /// Places an item with a custom display name and lore in the specified slot.
    ///
    /// Both `name` and each line of `lore` are parsed as `MiniMessage` strings.
    /// Plain text without tags passes through unchanged. Places a single item
    /// (count 1); use [`stack`](Self::stack) with a custom-built `ItemStack` to
    /// control the stack size.
    #[must_use]
    pub fn lore_item(self, slot: u32, item: impl IntoItemKey, name: &str, lore: &[&str]) -> Self {
        let stack = ItemStack::of(item, 1);
        stack.set_custom_name(Some(crate::utils::text::parse_minimessage(name)));

        let lore_components: Vec<TextComponent> = lore
            .iter()
            .map(|line| crate::utils::text::parse_minimessage(line))
            .collect();
        stack.set_lore(lore_components);

        self.stack(slot, stack)
    }

    /// Fills every empty slot with a blank-named filler item.
    ///
    /// Slots that already have an item placed via [`item`](Self::item),
    /// [`named_item`](Self::named_item), or [`lore_item`](Self::lore_item) are
    /// left untouched. The filler's name is set to a single space (`" "`) so
    /// hovering shows no tooltip — the typical pattern for menu filler panes.
    ///
    /// For full control over the filler's name, lore, or other item data,
    /// use [`fill_empty_with`](Self::fill_empty_with).
    ///
    /// The number of slots is derived from the [`Screen`] type passed to
    /// [`GuiBuilder::new`].
    #[must_use]
    pub fn fill_empty(mut self, item: impl IntoItemKey) -> Self {
        let key = item.into_item_key();
        for slot in 0..self.screen.slot_count() {
            if !self.items.iter().any(|(s, _)| *s == slot) {
                let stack = ItemStack::of(&key, 1);
                stack.set_custom_name(Some(TextComponent::text(" ")));
                self = self.stack(slot, stack);
            }
        }
        self
    }

    /// Fills every empty slot with stacks produced by a builder closure.
    ///
    /// The closure is called once per empty slot, receiving the slot index
    /// and returning a fresh [`ItemStack`]. Use this for full control over
    /// the filler's name, lore, count, or other item data.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// .fill_empty_with(|_slot| {
    ///     let stack = ItemStack::of(Item::GrayStainedGlassPane, 1);
    ///     stack.set_custom_name(Some(TextComponent::text("Panel")));
    ///     stack
    /// })
    /// ```
    #[must_use]
    pub fn fill_empty_with<F>(mut self, build: F) -> Self
    where
        F: Fn(u32) -> ItemStack,
    {
        for slot in 0..self.screen.slot_count() {
            if !self.items.iter().any(|(s, _)| *s == slot) {
                self.items.push((slot, build(slot)));
            }
        }
        self
    }

    /// Builds the [`Gui`] instance with all configured items and settings.
    ///
    /// If no title was set via [`GuiBuilder::title`], an empty title is used.
    #[must_use]
    pub fn build(self) -> gui::Gui {
        let title = self.title.unwrap_or_else(|| TextComponent::text(""));
        let gui = gui::Gui::new(self.screen, title);
        gui.set_allow_grab_items(self.allow_grab);
        gui.set_allow_put_items(self.allow_put);
        for (slot, stack) in self.items {
            gui.set_item(slot, stack);
        }
        gui
    }
}
