//! Embertread custom enchantment.
//!
//! Feet-slot enchantment that cancels contact and fire damage while worn.
//!
//! ## Status
//!
//! TODO: Implement once the following Pumpkin APIs are verified:
//! - Custom enchantment registration via [`EnchantmentBuilder`].
//! - Reading equipped boots from a player.
//! - `EntityDamageEvent` with damage-type filtering.
//!
//! `IllyriaPlus` behavior reference:
//! - Register as level-1 enchantment active in the feet slot.
//! - Listen to `EntityDamageEvent`.
//! - Cancel if cause is `CONTACT` or `FIRE` and boots have Embertread.

use crate::modules::enchantments::enchantment::Enchantment;

/// Stub for the Embertread custom enchantment.
#[derive(Default)]
pub struct Embertread;

impl Enchantment for Embertread {
    fn enabled(&self) -> bool {
        // TODO: wire to config toggle once registration path is implemented
        false
    }
}
