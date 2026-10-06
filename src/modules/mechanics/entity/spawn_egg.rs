//! Spawn Egg module — rare spawn egg drops from mob deaths.
//!
//! When any entity dies, there is a small chance (0.1%) for it to drop its
//! corresponding spawn egg.
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
//! | `EntityDeathEvent` entity type | Get the dead entity's type to determine spawn egg |
//! | `EntityDeathEvent` drops list | Add spawn egg to the drop list |
//! | Spawn egg lookup | Convert entity type to spawn egg item (e.g., `Zombie` → `ZombieSpawnEgg`) |
//! | `ItemStack` in drops | Add spawn egg to drops (needs drop list modification) |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! `IllyriaPlus` behavior reference:
//! - On `EntityDeathEvent`: 0.1% chance to drop the entity's spawn egg
//! - Uses `entityType.spawnEgg()` to get the matching spawn egg item

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles spawn egg drops from mob deaths.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct SpawnEgg;

impl Mechanic for SpawnEgg {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.spawn_egg.enabled)
    }

    // No events registered — stub module due to missing APIs.
    // The EntityDeathEvent exists but lacks entity type and drops list.
}

/// Configuration for the spawn egg mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawnEggConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
