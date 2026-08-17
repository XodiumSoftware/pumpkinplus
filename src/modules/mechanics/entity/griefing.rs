//! Griefing module - prevents mob griefing (block changes and explosions).
//!
//! ## Configuration
//!
//! | Field               | Default                                                                  | Description                                         |
//! |---------------------|--------------------------------------------------------------------------|-----------------------------------------------------|
//! | `enabled`           | `false`                                                                  | Whether this module is active                       |
//! | `cancelled_entities`| `["Blaze", "Creeper", "EnderDragon", "Enderman", "Fireball", "SmallFireball", "Wither"]` | Entity types whose griefing is blocked              |
//!
//! ## Mechanics
//!
//! When enabled, this module cancels two kinds of griefing events for configured
//! entity types:
//!
//! - `EntityChangeBlockEvent` — prevents the entity from changing blocks
//!   (e.g., Endermen picking up blocks, Creepers igniting fire).
//! - `EntityExplodeEvent` — prevents the entity from exploding
//!   (e.g., Creeper explosions, Wither projectile explosions).
//!
//! ## Notes
//!
//! `EntityExplodeEvent` only supports cancelling the whole explosion; the API does
//! not expose a per-block explode list to selectively remove affected blocks.

use crate::EntityType;
use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::utils::entity::entity_type_by_id;
use pumpkin_plugin_api::events::{
    EntityChangeBlockEvent, EntityExplodeEvent, EventData, EventHandler, EventPriority,
};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles mob griefing prevention.
#[derive(Default)]
pub struct Griefing;

impl Mechanic for Griefing {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.griefing.enabled)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<EntityChangeBlockEvent, _>(
                Griefing,
                EventPriority::Normal,
                true,
            )
            .expect("failed to register entity change block event handler");
        context
            .register_event_handler::<EntityExplodeEvent, _>(Griefing, EventPriority::Normal, true)
            .expect("failed to register entity explode event handler");
    }
}

impl EventHandler<EntityChangeBlockEvent> for Griefing {
    fn handle(
        &self,
        server: Server,
        mut event: EventData<EntityChangeBlockEvent>,
    ) -> EventData<EntityChangeBlockEvent> {
        if let Some(entity_type) = entity_type_by_id(
            &server,
            event
                .entity_id
                .try_into()
                .expect("entity id should fit in u32"),
        ) {
            let config: GriefingConfig = ConfigManager::get()
                .map(|cm| cm.griefing)
                .unwrap_or_default();
            if config.cancelled_entities.contains(&entity_type) {
                event.cancelled = true;
            }
        }
        event
    }
}

impl EventHandler<EntityExplodeEvent> for Griefing {
    fn handle(
        &self,
        server: Server,
        mut event: EventData<EntityExplodeEvent>,
    ) -> EventData<EntityExplodeEvent> {
        if let Some(entity_type) = entity_type_by_id(
            &server,
            event
                .entity_id
                .try_into()
                .expect("entity id should fit in u32"),
        ) {
            let config: GriefingConfig = ConfigManager::get()
                .map(|cm| cm.griefing)
                .unwrap_or_default();
            if config.cancelled_entities.contains(&entity_type) {
                event.cancelled = true;
            }
        }
        event
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
