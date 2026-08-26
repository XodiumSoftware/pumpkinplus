//! Feather Falling enchantment behavior override.
//!
//! Cancels farmland trampling when a player wearing Feather Falling boots
//! would trigger a physical interaction with farmland.
//!
//! ## Status
//!
//! TODO: Implement once the following Pumpkin APIs are verified:
//! - A farmland trample / physical-interaction event, or a way to detect trampling.
//! - Reading equipped boots from a player (`equipment-slot::feet`).
//! - Reading vanilla enchantments from an item stack.
//!
//! `IllyriaPlus` behavior reference:
//! - Listen to `PlayerInteractEvent` with `Action.PHYSICAL` on `Material.FARMLAND`.
//! - If the player's boots have Feather Falling, cancel the event.

use crate::modules::enchantments::enchantment::Enchantment;

/// Stub for the Feather Falling enchantment override.
#[derive(Default)]
pub struct FeatherFalling;

impl Enchantment for FeatherFalling {
    fn enabled(&self) -> bool {
        // TODO: wire to config toggle once the trample/physical event is available
        false
    }
}
