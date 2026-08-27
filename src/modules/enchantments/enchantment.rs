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
use pumpkin_plugin_api::enchantment::EnchantmentBuilder;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use pumpkin_plugin_api::text::TextComponent;
use serde::{Deserialize, Serialize};
use tracing::error;

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

    /// Builds this enchantment's custom definition for registration.
    ///
    /// Override this to return an [`EnchantmentBuilder`] for custom enchantments
    /// that should appear in the enchanting table and on items. Returns a builder
    /// with a placeholder id by default; vanilla behavior overrides can leave this
    /// as-is.
    fn enchantment(&self) -> EnchantmentBuilder {
        EnchantmentBuilder::new("pumpkinplus:noop", TextComponent::text("Noop"))
    }

    /// Registers this enchantment's event handlers with the server.
    ///
    /// Override this to call [`Enchantment::register_event`] for each event this
    /// enchantment handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers `Self` as the handler for event `T` and panics on failure.
    ///
    /// This is a thin wrapper around [`Context::register_event_handler`] that
    /// supplies the enchantment-specific error message so individual
    /// enchantments don't have to repeat it.
    fn register_event<T>(&self, context: &Context, priority: EventPriority, ignore_cancelled: bool)
    where
        T: FromIntoEvent + Send + Sync + 'static,
        Self: EventHandler<T> + Default + Send + Sync + 'static,
    {
        context
            .register_event_handler::<T, _>(Self::default(), priority, ignore_cancelled)
            .expect("failed to register enchantment event handler");
    }

    /// Registers this enchantment with the server if it is enabled.
    ///
    /// This calls [`Enchantment::enchantment`] to build the definition, registers it
    /// with the server, then calls [`Enchantment::events`]. Any registration error
    /// is logged here so individual enchantments only need to build and return the
    /// builder.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        if let Err(e) = context.register_enchantment(self.enchantment()) {
            error!("Failed to register enchantment: {e}");
        }
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
