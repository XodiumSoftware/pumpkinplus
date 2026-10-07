//! GUI builder utility for creating custom inventory-based menus.
//!
//! Provides a fluent [`GuiBuilder`] for constructing [`Gui`](pumpkin_plugin_api::gui::Gui) instances with
//! title, size, items, and interaction rules.
//!
//! ## Item identifiers
//!
//! Item-taking methods accept any [`IntoItemKey`], which includes both the
//! typed [`Item`](pumpkin_plugin_api::Item) enum (preferred — compile-time checked) and string keys like
//! `"minecraft:diamond"`. Prefer the enum when the item is a known vanilla
//! item; fall back to strings only for custom/modded items.
//!
//! ## Example
//!
//! ```rust,ignore
//! use pumpkin_plugin_api::screens_wit::Screen;
//! use pumpkin_plugin_api::Item;
//! use crate::guis::gui::{GuiBuilder, ItemBuilder};
//!
//! let gui = GuiBuilder::new(Screen::Generic9x3)
//!     .title("<gold>My Menu")
//!     .item(13, ItemBuilder::new(Item::Diamond)
//!         .title("<aqua>Shiny")
//!         .lore(&["<gray>Click me"])
//!         .on_click(|event| event)
//!         .build())
//!     .allow_grab(false)
//!     .allow_put(false)
//!     .build();
//!
//! player.open_gui(gui);
//! ```

use pumpkin_plugin_api::events::{EventData, InventoryClickEvent};
use pumpkin_plugin_api::gui;
use pumpkin_plugin_api::screens_wit::Screen;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{IntoItemKey, ItemStack, ItemStackExt};
use std::collections::HashMap;

use crate::guis::registry::{GuiId, GuiSlotHandler};

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

/// A fluent builder for creating [`Gui`](pumpkin_plugin_api::gui::Gui) instances.
///
/// The builder allows setting the GUI type, title, items, and interaction
/// permissions before constructing the final `Gui`.
///
/// Set an [`id`](Self::id) and register [`on_click`](Self::on_click) handlers
/// to make specific slots act as buttons. Handlers are stored in the global
/// [`registry`](crate::guis::registry) on `build()`, and routed to players
/// via the plugin's `InventoryClickEvent` handler.
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
    /// Optional registry ID. Required for click dispatch.
    id: Option<GuiId>,
    /// Per-slot click handlers, registered in the global registry on `build()`.
    click_handlers: HashMap<i16, GuiSlotHandler>,
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
            id: None,
            click_handlers: HashMap::new(),
        }
    }

    /// Assigns a unique identifier to this GUI.
    ///
    /// The ID is used by the click [`registry`](crate::guis::registry) to
    /// associate click events from a player with the GUI's slot handlers.
    /// Required for [`on_click`](Self::on_click) handlers to fire.
    #[must_use]
    pub fn id(mut self, id: GuiId) -> Self {
        self.id = Some(id);
        self
    }

    /// Registers a click handler for a specific slot.
    ///
    /// The handler receives the full `InventoryClickEvent` so it can read the
    /// clicked slot, click type, and the player who clicked. Clicks on tracked
    /// GUIs are cancelled by default; cancel further behavior manually by
    /// setting `event.cancelled = true` (usually not needed).
    ///
    /// Handlers are stored in the global registry on `build()` and require
    /// the GUI to have an [`id`](Self::id) set. Players must be tracked via
    /// [`registry::open`] for the handlers to fire.
    ///
    /// [`registry::open`]: crate::guis::registry::open
    #[must_use]
    pub fn on_click<F>(mut self, slot: u32, handler: F) -> Self
    where
        F: Fn(
                pumpkin_plugin_api::events::EventData<
                    pumpkin_plugin_api::events::InventoryClickEvent,
                >,
            ) -> pumpkin_plugin_api::events::EventData<
                pumpkin_plugin_api::events::InventoryClickEvent,
            > + Send
            + Sync
            + 'static,
    {
        #[allow(clippy::cast_possible_truncation)]
        self.click_handlers.insert(slot as i16, Box::new(handler));
        self
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
    /// `slot` is the inventory index (0-based). Accepts any [`IntoGuiItem`]:
    /// - An [`Item`](pumpkin_plugin_api::Item) or `&str` key places a single plain item (count 1).
    /// - A [`GuiItem`] from [`ItemBuilder::build`] places the built stack and
    ///   registers any click handler attached to it.
    ///
    /// Use [`stack`](Self::stack) with a custom-built `ItemStack` to control
    /// the stack size, or [`ItemBuilder`] for a fluent item-with-handler API.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Plain item:
    /// builder.item(0, Item::Diamond)
    ///
    /// // Item with title, lore, and click handler:
    /// builder.item(1, ItemBuilder::new(Item::Compass)
    ///     .title("<green>Teleport")
    ///     .lore(&["<gray>Click to open"])
    ///     .on_click(|event| event)
    ///     .build())
    /// ```
    #[must_use]
    pub fn item(mut self, slot: u32, item: impl IntoGuiItem) -> Self {
        let gui_item = item.into_gui_item();
        self.items.push((slot, gui_item.stack));
        if let Some(handler) = gui_item.on_click {
            #[allow(clippy::cast_possible_truncation)]
            self.click_handlers.insert(slot as i16, handler);
        }
        self
    }

    /// Fills every empty slot with a blank-named filler item.
    ///
    /// Slots that already have an item placed via [`item`](Self::item),
    /// [`stack`](Self::stack), or [`fill_empty_with`](Self::fill_empty_with) are
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

    /// Builds the [`Gui`](pumpkin_plugin_api::gui::Gui) instance with all configured items and settings.
    ///
    /// If no title was set via [`GuiBuilder::title`], an empty title is used.
    ///
    /// If an `id` is set via [`GuiBuilder::id`], any registered `on_click`
    /// handlers are stored in the global registry. Rebuilding the same GUI
    /// replaces its handlers (safe to call on every command invocation).
    #[must_use]
    pub fn build(self) -> gui::Gui {
        if let Some(id) = self.id {
            crate::guis::registry::register_gui(id, self.click_handlers);
        }
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

/// A built GUI slot item returned by [`ItemBuilder::build`].
///
/// Pairs the [`ItemStack`] to display with an optional click handler. Use
/// [`ItemBuilder`] to construct one — `GuiItem` itself is an opaque value
/// consumed by [`GuiBuilder::item`].
pub struct GuiItem {
    /// The stack placed in the slot.
    stack: ItemStack,
    /// Optional handler invoked when the slot is clicked.
    on_click: Option<GuiSlotHandler>,
}

/// Conversion trait for values that can be placed into a GUI slot.
///
/// Implemented for [`Item`](pumpkin_plugin_api::Item), `&Item`, `&str`, `String`, [`ItemStack`], and
/// [`GuiItem`]. Lets [`GuiBuilder::item`] accept either a plain item key
/// (for purely decorative entries) or a fully-built [`GuiItem`] (for entries
/// that may carry a title, lore, or click handler).
pub trait IntoGuiItem {
    /// Converts into a [`GuiItem`].
    fn into_gui_item(self) -> GuiItem;
}

impl IntoGuiItem for GuiItem {
    fn into_gui_item(self) -> GuiItem {
        self
    }
}

impl IntoGuiItem for ItemStack {
    fn into_gui_item(self) -> GuiItem {
        GuiItem {
            stack: self,
            on_click: None,
        }
    }
}

macro_rules! impl_into_gui_item_for_key {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoGuiItem for $ty {
                fn into_gui_item(self) -> GuiItem {
                    GuiItem {
                        stack: ItemStack::of(self, 1),
                        on_click: None,
                    }
                }
            }
        )*
    };
}

impl_into_gui_item_for_key!(
    pumpkin_plugin_api::Item,
    &pumpkin_plugin_api::Item,
    &str,
    String,
);

/// A fluent builder for a single GUI slot entry.
///
/// Produces a [`GuiItem`] via [`build`](Self::build), ready to pass to
/// [`GuiBuilder::item`]. Combines the item, optional title (display name),
/// lore lines, and click handler into one chain — no parallel per-slot
/// placement / click-handler calls on [`GuiBuilder`].
///
/// All `&str` text inputs are parsed as `MiniMessage` strings; plain text
/// without tags passes through unchanged.
///
/// # Example
///
/// ```rust,ignore
/// use pumpkin_plugin_api::Item;
/// use crate::guis::gui::{GuiBuilder, ItemBuilder};
///
/// GuiBuilder::new(Screen::Hopper)
///     // ... other builder calls ...
///     .item(1, ItemBuilder::new(Item::Compass)
///         .title("<green><bold>Teleport")
///         .lore(&["<gray>Click to open the teleport menu."])
///         .on_click(|event| {
///             // handle the click...
///             event
///         })
///         .build())
///     .build();
/// ```
#[allow(dead_code)]
pub struct ItemBuilder {
    /// Stack being built. Name and lore are applied in place.
    stack: ItemStack,
    /// Optional click handler to attach to the slot.
    on_click: Option<GuiSlotHandler>,
}

#[allow(dead_code)]
impl ItemBuilder {
    /// Starts a new builder for the given item. Places a single item
    /// (count 1); use [`count`](Self::count) to override.
    #[must_use]
    pub fn new(item: impl IntoItemKey) -> Self {
        Self {
            stack: ItemStack::of(item, 1),
            on_click: None,
        }
    }

    /// Starts a builder from a pre-constructed [`ItemStack`].
    ///
    /// Useful when you need to set fields (custom model data, damage, etc.)
    /// not exposed through this builder.
    #[must_use]
    pub fn from_stack(stack: ItemStack) -> Self {
        Self {
            stack,
            on_click: None,
        }
    }

    /// Sets the stack size.
    #[must_use]
    pub fn count(self, count: u8) -> Self {
        self.stack.set_count(count);
        self
    }

    /// Sets the item's display name (parsed as `MiniMessage`).
    #[must_use]
    pub fn title(self, title: &str) -> Self {
        self.stack
            .set_custom_name(Some(crate::utils::text::parse_minimessage(title)));
        self
    }

    /// Sets the item's lore lines (each parsed as `MiniMessage`).
    #[must_use]
    pub fn lore(self, lore: &[&str]) -> Self {
        let lines: Vec<TextComponent> = lore
            .iter()
            .map(|line| crate::utils::text::parse_minimessage(line))
            .collect();
        self.stack.set_lore(lines);
        self
    }

    /// Attaches a click handler to the slot.
    ///
    /// Only fires if the [`GuiBuilder`] this item is placed into has an
    /// [`id`](GuiBuilder::id) set and is opened via [`registry::open`].
    /// Calling `on_click` multiple times replaces the previous handler.
    ///
    /// [`registry::open`]: crate::guis::registry::open
    #[must_use]
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(EventData<InventoryClickEvent>) -> EventData<InventoryClickEvent>
            + Send
            + Sync
            + 'static,
    {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Builds the [`GuiItem`] ready to be placed by [`GuiBuilder::item`].
    #[must_use]
    pub fn build(self) -> GuiItem {
        GuiItem {
            stack: self.stack,
            on_click: self.on_click,
        }
    }
}
