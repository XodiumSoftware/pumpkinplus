//! Bat module — custom phantom membrane drops from bat deaths.
//!
//! When a bat dies, it drops phantom membranes with a configurable chance and
//! amount. The drop amount scales with the killer's Looting enchantment level.
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
//! | `EntityDeathEvent` entity type | Verify the dead entity is a bat |
//! | `EntityDeathEvent` killer reference | Get the player who killed the bat (for Looting) |
//! | `EntityDeathEvent` drops list | Add phantom membranes to the drop list |
//! | `World.get_game_rule()` | Check `SPAWN_PHANTOMS` game rule |
//! | `ItemStack` in drops | Add phantom membranes to drops (needs drop list modification) |
//!
//! `IllyriaPlus` behavior reference:
//! - On `EntityDeathEvent`: if entity is a bat and `SPAWN_PHANTOMS` is false,
//!   drop phantom membranes with chance
//! - Base drop: 0-1 membranes (100% chance)
//! - Looting bonus: +1 per level

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles bat death phantom membrane drops.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Bat;

impl Mechanic for Bat {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.bat.enabled)
    }

    // No events registered — stub module due to missing APIs.
    // The EntityDeathEvent exists but lacks entity type, killer, drops list,
    // and game rule access.
}

/// Configuration for the bat mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
