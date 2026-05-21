//! Griefing module - prevents mob griefing (block changes and explosions).
//!
//! ## Configuration
//!
//! | Field               | Default                                                                  | Description                                         |
//! |---------------------|--------------------------------------------------------------------------|-----------------------------------------------------|
//! | `enabled`           | `false`                                                                  | Whether this module is active                       |
//! | `cancelled_entities`| `["Blaze", "Creeper", "EnderDragon", "Enderman", "Fireball", "SmallFireball", "Wither"]` | Entity types whose griefing is blocked              |
//!
//! ## Notes
//!
//! This module is currently a stub. The Pumpkin plugin API does not yet expose
//! `EntityChangeBlockEvent` or `EntityExplodeEvent` (or equivalents), so mob
//! griefing prevention cannot be hooked until upstream support is added.

use crate::EntityType;
use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles mob griefing prevention.
#[derive(Default)]
pub struct Griefing;

impl Mechanic for Griefing {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<GriefingConfig>().enabled)
            .unwrap_or(false)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when EntityChangeBlockEvent and EntityExplodeEvent
        // (or equivalents) are available in the Pumpkin plugin API.
        //
        // The intended logic is:
        //
        // 1. Listen for entity change block events.
        //    - If the entity type is in `config.cancelled_entities`, cancel the event.
        //
        // 2. Listen for entity explode events.
        //    - If the entity type is in `config.cancelled_entities`, clear the block list.
    }
}

/// Configuration for the griefing mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GriefingConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Entity types whose griefing actions (block changes and explosions) are blocked.
    /// Leave empty to disable entity filtering.
    pub cancelled_entities: Vec<EntityType>,
}

impl Default for GriefingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cancelled_entities: vec![
                EntityType::Blaze,
                EntityType::Creeper,
                EntityType::EnderDragon,
                EntityType::Enderman,
                EntityType::Fireball,
                EntityType::SmallFireball,
                EntityType::Wither,
            ],
        }
    }
}
