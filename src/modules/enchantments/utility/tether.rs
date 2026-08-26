//! Tether custom enchantment.
//!
//! Main-hand enchantment that teleports block drops and mob death XP directly
//! into the player's inventory.
//!
//! ## Status
//!
//! TODO: Implement once the following Pumpkin APIs are verified:
//! - Custom enchantment registration via [`EnchantmentBuilder`].
//! - `BlockDropItemEvent` and `EntityDeathEvent`.
//! - Reading the killer from `EntityDeathEvent`.
//! - Adding items / experience points to a player inventory.
//!
//! `IllyriaPlus` behavior reference:
//! - Register as level-1 enchantment active in the main hand.
//! - On `BlockDropItemEvent`: add dropped item stacks to the breaker inventory.
//! - On `EntityDeathEvent`: if the killer holds Tether, give XP to the killer
//!   and clear dropped XP.

use crate::modules::enchantments::enchantment::Enchantment;

/// Stub for the Tether custom enchantment.
#[derive(Default)]
pub struct Tether;

impl Enchantment for Tether {
    fn enabled(&self) -> bool {
        // TODO: wire to config toggle once registration path is implemented
        false
    }
}
