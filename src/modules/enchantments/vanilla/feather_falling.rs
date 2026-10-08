//! Feather Falling enchantment behavior override.
//!
//! Cancels farmland trampling when a player wearing Feather Falling boots
//! would trigger a physical interaction with farmland.
//!
//! ## Status
//!
//! **Stub module** — Boots-side detection is implemented
//! ([`has_feather_falling_boots`]), but farmland trampling cannot be hooked
//! because the Pumpkin plugin API does not currently emit a physical-interaction
//! event for it.
//!
//! ## Missing APIs
//!
//! | API | Purpose |
//! |-----|---------|
//! | Physical-interaction / trample event | Detect when a player steps on farmland |
//! | `PlayerInteractAction::Physical` variant | Alternative: reuse the existing interact event with a physical arm |
//!
//! Boot-side reads are available today: `Player.get_inventory().get_boots()`
//! and `ItemStack.get_enchantments()` return the vanilla [`Enchantment`]
//! variant, which includes `FeatherFalling`.
//!
//! `IllyriaPlus` behavior reference:
//! - Listen to `PlayerInteractEvent` with `Action.PHYSICAL` on `Material.FARMLAND`.
//! - If the player's boots have Feather Falling, cancel the event.

use crate::modules::enchantments::enchantment::Enchantment as EnchantmentBehavior;
use pumpkin_plugin_api::enchantment::Enchantment;
use pumpkin_plugin_api::player::Player;

/// Stub for the Feather Falling enchantment override.
#[derive(Default)]
pub struct FeatherFalling;

impl EnchantmentBehavior for FeatherFalling {
    fn enabled(&self) -> bool {
        // TODO: wire to config toggle once a trample event exists in the API
        false
    }

    // TODO: register a physical-interaction event handler once the upstream API
    // exposes one (either a dedicated `FarmlandTrampleEvent`, or an additional
    // `Physical` variant on the existing `InteractAction` enum). Inside the
    // handler, use [`has_feather_falling_boots`] on `event.player` and cancel
    // the event when it returns `true`.
}

/// Returns `true` when the player is wearing boots enchanted with
/// vanilla Feather Falling (any level).
///
/// This helper is used by the (currently stubbed) trample-cancel handler; it
/// is also exposed for other modules that need the same check.
#[must_use]
pub fn has_feather_falling_boots(player: &Player) -> bool {
    let inventory = player.get_inventory();
    inventory.get_boots().is_some_and(|boots| {
        boots
            .get_enchantments()
            .iter()
            .any(|e| e.enchantment == Enchantment::FeatherFalling)
    })
}
