//! GUI registry for click dispatch and per-player session tracking.
//!
//! The Pumpkin `InventoryClickEvent` doesn't identify *which* GUI is open —
//! only the screen type and slot clicked. To route clicks to the correct
//! button callbacks, this module maintains two pieces of global state:
//!
//! - **`GUIS`** — maps [`GuiId`] to per-slot click handlers registered via
//!   [`GuiBuilder::on_click`](crate::guis::gui::GuiBuilder::on_click).
//! - **`SESSIONS`** — maps a player's UUID to the [`GuiId`] they currently
//!   have open, populated when a GUI is opened via [`open`] and cleared on
//!   `InventoryCloseEvent` via [`close`].
//!
//! When a player clicks, [`dispatch_click`] looks up the player's open GUI,
//! then invokes the slot's handler if one is registered.
//!
//! ## Usage
//!
//! GUIs register their slot handlers on build (via [`GuiBuilder::id`] +
//! [`GuiBuilder::on_click`]). Open the GUI for a player with [`open`] instead
//! of calling `player.open_gui` directly so the session is tracked.
//!
//! The plugin registers [`dispatch_click`] and [`dispatch_close`] as global
//! `InventoryClickEvent` / `InventoryCloseEvent` handlers on load.
//!
//! [`GuiBuilder::id`]: crate::guis::gui::GuiBuilder::id
//! [`GuiBuilder::on_click`]: crate::guis::gui::GuiBuilder::on_click

use std::cell::RefCell;
use std::collections::HashMap;

use pumpkin_plugin_api::events::{EventData, InventoryClickEvent, InventoryCloseEvent};
use pumpkin_plugin_api::player::Player;

/// Identifier for a registered GUI.
///
/// Typically a `&'static str` like `"admin_menu"` or `"rules"` — one per call
/// site using [`GuiBuilder::id`](crate::guis::gui::GuiBuilder::id).
pub type GuiId = &'static str;

/// Click handler callback for a single GUI slot.
///
/// Receives the full click event so the handler can read the slot index,
/// click type, or cancel the interaction.
pub type GuiSlotHandler =
    Box<dyn Fn(EventData<InventoryClickEvent>) -> EventData<InventoryClickEvent> + Send + Sync>;

thread_local! {
    /// Registered click handlers per GUI ID, keyed by slot.
    static GUIS: RefCell<HashMap<GuiId, HashMap<i16, GuiSlotHandler>>> =
        RefCell::new(HashMap::new());

    /// Which GUI each player currently has open, keyed by `(high, low)` UUID halves.
    static SESSIONS: RefCell<HashMap<(u64, u64), GuiId>> = RefCell::new(HashMap::new());
}

/// Returns the registry key for a player.
fn key_for(player: &Player) -> (u64, u64) {
    let id = player.get_id();
    (id.high, id.low)
}

/// Registers the slot handlers for a GUI ID.
///
/// Called by [`GuiBuilder::build`](crate::guis::gui::GuiBuilder::build) when
/// the builder has an `id` and at least one `on_click` handler. Re-registering
/// the same ID replaces its handlers (used when rebuilding a GUI each call).
pub(crate) fn register_gui(id: GuiId, handlers: HashMap<i16, GuiSlotHandler>) {
    GUIS.with(|g| {
        g.borrow_mut().insert(id, handlers);
    });
}

/// Marks `player` as having the GUI with `id` open.
///
/// Must be called every time [`Gui`] is opened so click dispatch can route
/// clicks from this player back to the right handlers.
///
/// [`Gui`]: pumpkin_plugin_api::gui::Gui
pub fn open(player: &Player, id: GuiId) {
    SESSIONS.with(|s| {
        s.borrow_mut().insert(key_for(player), id);
    });
}

/// Clears the open-GUI session for a player.
///
/// Called automatically by the plugin's `InventoryCloseEvent` handler.
pub fn close(player: &Player) {
    SESSIONS.with(|s| {
        s.borrow_mut().remove(&key_for(player));
    });
}

/// Returns the GUI ID the player currently has open, if any.
#[must_use]
pub fn current(player: &Player) -> Option<GuiId> {
    SESSIONS.with(|s| s.borrow().get(&key_for(player)).copied())
}

/// Routes a click event to the slot handler of the player's open GUI.
///
/// Looks up the player's open GUI by UUID, finds the handler for the clicked
/// slot, and invokes it. If no handler is registered for that GUI/slot, the
/// event is returned unchanged (still marked cancelled for tracked GUIs to
/// prevent item grabbing when `allow_grab` is false).
///
/// Slots outside the GUI's own slots (i.e., the player's own inventory below
/// the menu) are routed based on `raw_slot` as well — for chest-style screens
/// the GUI occupies the top rows of `raw_slot`.
pub fn dispatch_click(mut event: EventData<InventoryClickEvent>) -> EventData<InventoryClickEvent> {
    let Some(id) = current(&event.player) else {
        return event;
    };

    // Any click in a tracked GUI is cancelled to prevent item pickup.
    event.cancelled = true;

    GUIS.with(|g| {
        let g = g.borrow();
        if let Some(handlers) = g.get(id)
            && let Some(handler) = handlers.get(&event.slot)
        {
            event = handler(event);
        }
        event
    })
}

/// Routes a close event, clearing the player's tracked GUI session.
pub fn dispatch_close(event: EventData<InventoryCloseEvent>) -> EventData<InventoryCloseEvent> {
    close(&event.player);
    event
}
