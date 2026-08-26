//! Enchantment system for `PumpkinPlus`.
//!
//! Each enchantment is implemented as a module implementing the [`Enchantment`] trait.
//! Enchantments can register a brand new custom definition through
//! `pumpkin_plugin_api::enchantment::EnchantmentBuilder`, listen to events to provide
//! behavior, or both.
//!
//! ## Configuration
//!
//! Enchantments can be toggled individually via the `enchantments` section of
//! `config.json`. Each enchantment is disabled by default.

use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// A trait representing a `PumpkinPlus` enchantment.
///
/// Types implementing this trait may register a custom enchantment definition
/// and/or event-based behavior.
pub trait Enchantment {
    /// Returns `true` if this enchantment is enabled in the configuration.
    ///
    /// Defaults to `false`.
    fn enabled(&self) -> bool {
        false
    }

    /// Registers this enchantment's custom definition with the server.
    ///
    /// Override this to call [`Context::register_enchantment`] for custom
    /// enchantments that should appear in the enchanting table and on items.
    /// No-op by default.
    fn register_custom(&self, _context: &Context) {}

    /// Registers this enchantment's event handlers with the server.
    ///
    /// Override this to call [`Context::register_event_handler`] for each event
    /// this enchantment handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers this enchantment with the server if it is enabled.
    ///
    /// This calls [`Enchantment::register_custom`] followed by
    /// [`Enchantment::events`].
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        self.register_custom(context);
        self.events(context);
    }
}

/// Top-level configuration for all enchantments.
///
/// Each boolean toggles one enchantment. All enchantments are disabled by default
/// to match the behavior of other gameplay modules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct EnchantmentsConfig {
    /// Cancel contact and fire damage when wearing Embertread boots.
    pub embertread: bool,
    /// Cancel farmland trampling when wearing Feather Falling boots.
    pub feather_falling: bool,
    /// Auto-replant mature crops when broken with a Fortune hoe.
    pub fortune: bool,
    /// Drop spawners and budding amethyst when broken with Silk Touch pickaxes.
    pub silk_touch: bool,
}
