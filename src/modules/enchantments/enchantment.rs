//! Enchantment system for `PumpkinPlus`.
//!
//! Each enchantment behavior override is implemented as a module implementing the
//! [`Enchantment`] trait. Enchantment overrides listen to events and react when a
//! player is using an item with a specific vanilla enchantment.
//!
//! They do **not** register new enchantments; they check existing vanilla enchantments
//! via [`ItemStack::get_enchantments`].
//!
//! ## Configuration
//!
//! Enchantment overrides can be toggled individually via the `enchantments` section
//! of `config.json`. Each override is disabled by default.

use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// A trait representing a vanilla enchantment behavior override.
///
/// Types implementing this trait provide event-based behavior that activates when
/// a player uses an item with a specific vanilla enchantment.
pub trait Enchantment {
    /// Returns `true` if this enchantment override is enabled in the configuration.
    ///
    /// Defaults to `false`.
    fn enabled(&self) -> bool {
        false
    }

    /// Registers this override's event handlers with the server.
    ///
    /// Override this to call [`Context::register_event_handler`] for each event
    /// this enchantment handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers this override with the server if it is enabled.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        self.events(context);
    }
}

/// Top-level configuration for all enchantment behavior overrides.
///
/// Each boolean toggles one override. All overrides are disabled by default
/// to match the behavior of other gameplay modules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct EnchantmentsConfig {
    /// Cancel farmland trampling when wearing Feather Falling boots.
    pub feather_falling: bool,
    /// Auto-replant mature crops when broken with a Fortune hoe.
    pub fortune: bool,
    /// Drop spawners and budding amethyst when broken with Silk Touch pickaxes.
    pub silk_touch: bool,
}
