//! Head module — rare player head drops on death.
//!
//! When a player dies, there is a small chance (1%) for their head to drop
//! as a player skull item with their profile texture.
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
//! | `World.drop_item()` | Spawn the head item at the death location |
//! | `ItemStack.set_profile()` | Set the player profile on the skull item |
//! | `Profile` component | Player head texture data (Mojang API integration) |
//! | `ItemStack` in drops | Alternative to world spawn — add to death drops |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerDeathEvent`: 1% chance to drop player head with skin texture
//! - Uses `ItemStack.of(Material.PLAYER_HEAD)` with `ResolvableProfile` data component

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles player head drops on death.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Head;

impl Mechanic for Head {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.head.enabled)
    }

    // No events registered — stub module due to missing APIs.
    // The PlayerDeathEvent exists but lacks world drop and item profile APIs.
}

/// Configuration for the head mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeadConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
