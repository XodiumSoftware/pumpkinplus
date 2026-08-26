//! Nimbus custom enchantment.
//!
//! Saddle-slot enchantment for Happy Ghasts that increases flying speed based
//! on enchantment level.
//!
//! ## Status
//!
//! TODO: Implement once the following Pumpkin APIs are verified:
//! - Custom enchantment registration via `pumpkin_plugin_api::enchantment::EnchantmentBuilder` with saddle slot.
//! - `EntityEquipmentChangedEvent` (or equivalent) for mount equipment changes.
//! - Happy Ghast entity type and `flying-speed` attribute access.
//!
//! `IllyriaPlus` behavior reference:
//! - Register as level 1–5 enchantment active in the saddle slot.
//! - Listen to equipment changes on Happy Ghasts.
//! - Scale base flying speed by level using a lookup table.
//! - Reset to default speed when the harness is no longer enchanted.

use crate::modules::enchantments::enchantment::Enchantment;

/// Stub for the Nimbus custom enchantment.
#[derive(Default)]
pub struct Nimbus;

impl Enchantment for Nimbus {
    fn enabled(&self) -> bool {
        // TODO: wire to config toggle once registration path is implemented
        false
    }
}
