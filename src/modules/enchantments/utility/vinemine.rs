//! Vinemine custom enchantment.
//!
//! Main-hand enchantment that mines connected matching ore blocks when breaking
//! an ore. Supports an optional Tether synergy to teleport drops into the
//! player's inventory.
//!
//! ## Status
//!
//! TODO: Implement once the following Pumpkin APIs are verified:
//! - Custom enchantment registration via [`EnchantmentBuilder`].
//! - `BlockBreakEvent` with player and block information.
//! - Reading/setting item data components (damage/durability).
//! - Breaking / replacing blocks in a world and reading block registry names.
//! - Optional: `BlockDropItemEvent` or drop simulation for Tether synergy.
//!
//! `IllyriaPlus` behavior reference:
//! - Register as level 1–3 enchantment active in the main hand.
//! - On `BlockBreakEvent`: if target is an ore and the held item has Vinemine,
//!   flood-fill connected ores up to the level limit.
//! - Break each connected ore naturally (or directly set to air and drop items
//!   if Tether is present).
//! - Increment damage on the tool and stop if durability runs out.

use crate::modules::enchantments::enchantment::Enchantment;

/// Stub for the Vinemine custom enchantment.
#[derive(Default)]
pub struct Vinemine;

impl Enchantment for Vinemine {
    fn enabled(&self) -> bool {
        // TODO: wire to config toggle once registration path is implemented
        false
    }
}
