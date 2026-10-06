//! XP module — convert experience to experience bottles.
//!
//! When a player right-clicks an enchanting table while holding a glass bottle,
//! 11 XP points are consumed and converted into an experience bottle.
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
//! | `Player.get_experience()` | Get total XP points (not just level) |
//! | `Player.give_exp()` | Add/remove XP points |
//! | `World.drop_item()` | Spawn the bottle if inventory is full |
//! | `PlayerInteractEvent.clicked_block` precise type | Check if clicked block is enchanting table |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerInteractEvent`: if clicked enchanting table with glass bottle,
//!   consume 11 XP and give experience bottle
//! - Costs 11 XP points (not levels)
//! - Bottle drops at player location if inventory is full

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles XP to bottle conversion.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Xp;

impl Mechanic for Xp {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.xp.enabled)
    }

    // No events registered — stub module due to missing APIs.
    // The PlayerInteractEvent exists but lacks XP manipulation and world drop APIs.
}

/// Configuration for the XP mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct XpConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
