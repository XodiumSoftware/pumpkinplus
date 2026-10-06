//! Husk module — custom sand drops from husk deaths.
//!
//! When a husk dies, it drops sand with a configurable chance and amount.
//! Husks riding camels drop bonus sand. The drop amount scales with the
//! killer's Looting enchantment level.
//!
//! ## Status
//!
//! **Stub module** — Cannot be implemented due to missing Pumpkin plugin APIs.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to implement this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `EntityDeathEvent` entity type | Verify the dead entity is a husk |
//! | `EntityDeathEvent` killer reference | Get the player who killed the husk (for Looting) |
//! | `EntityDeathEvent` drops list | Add sand items to the drop list |
//! | `Entity.get_vehicle()` | Check if husk was riding a camel (available but unusable without entity access) |
//! | `ItemStack` in drops | Add sand to drops (needs drop list modification) |
//!
//! `IllyriaPlus` behavior reference:
//! - On `EntityDeathEvent`: if entity is a husk, drop sand with chance
//! - Base drop: 0-2 sand (100% chance)
//! - Camel rider bonus: 0-3 sand base, +2 per Looting level
//! - Normal husk bonus: +1 per Looting level

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles husk death sand drops.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Husk;

impl Mechanic for Husk {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.husk.enabled)
    }

    // No events registered — stub module due to missing APIs.
    // The EntityDeathEvent exists but lacks entity type, killer, and drops list.
}

/// Configuration for the husk mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HuskConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
