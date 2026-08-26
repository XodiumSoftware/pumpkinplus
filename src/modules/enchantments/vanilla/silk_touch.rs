//! Silk Touch enchantment behavior override.
//!
//! Allows spawners and budding amethyst to drop themselves when broken with a
//! pickaxe carrying Silk Touch.
//!
//! ## Status
//!
//! TODO: Implement once the following Pumpkin APIs are verified:
//! - `BlockBreakEvent` with player, block, drop-flag, and XP fields.
//! - Reading vanilla enchantments from the held pickaxe.
//! - Cancelling default drops and optionally dropping custom item stacks.
//! - Reading spawner block-entity data to obtain the spawned mob type.
//!
//! `IllyriaPlus` behavior reference:
//! - On `BlockBreakEvent`, check if the held pickaxe has Silk Touch.
//! - For `Material.SPAWNER`: cancel default drops, drop the spawner block and a
//!   matching spawn egg.
//! - For `Material.BUDDING_AMETHYST`: cancel default drops and drop the block.

use crate::modules::enchantments::enchantment::Enchantment;

/// Stub for the Silk Touch enchantment override.
#[derive(Default)]
pub struct SilkTouch;

impl Enchantment for SilkTouch {
    fn enabled(&self) -> bool {
        // TODO: wire to config toggle once drop/event APIs are sufficient
        false
    }
}
