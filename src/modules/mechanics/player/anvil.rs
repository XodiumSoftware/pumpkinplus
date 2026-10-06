//! Anvil module — custom anvil operations including disenchantment and cost bypass.
//!
//! Provides advanced anvil features:
//! - Disenchant: Extract enchantments from items to books
//! - Cost bypass: Remove "Too Expensive!" limit and allow high-cost operations
//! - `MiniMessage` rename: Apply formatting to anvil rename text
//!
//! ## Status
//!
//! **Stub module** — Cannot be implemented due to missing Pumpkin plugin APIs.
//! This module requires extensive anvil GUI and packet manipulation APIs that
//! don't exist in the current Pumpkin plugin API.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to implement this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `InventoryOpenEvent` | Configure anvil when opened |
//! | `PrepareAnvilEvent` | Intercept anvil result preparation |
//! | `AnvilView` | Access/modify repair cost, result, rename text |
//! | `AnvilInventory` | Access anvil slots (input, material, result) |
//! | `ItemStack.set_item_on_cursor()` | Place result on cursor after disenchant |
//! | Enchantment data components | Read/write `STORED_ENCHANTMENTS` on books |
//! | `Player.give_exp_levels()` | Deduct XP levels for cost |
//! | Player abilities packet | Creative mode bypass for cost limit |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! `IllyriaPlus` behavior reference:
//! - Disenchant: place enchanted item + book → extract enchantments to enchanted book
//! - Cost bypass: allows operations > 40 levels by manipulating player abilities
//! - `MiniMessage`: rename text uses formatted text instead of plain

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles custom anvil operations.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Anvil;

impl Mechanic for Anvil {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.anvil.enabled)
    }

    // No events registered — stub module due to missing APIs.
    // The required anvil events (InventoryOpen, PrepareAnvil) don't exist.
}

/// Configuration for the anvil mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnvilConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
