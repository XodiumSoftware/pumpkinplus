//! Spawn Egg module — rare spawn egg drops from mob deaths.
//!
//! When any entity dies, there is a small chance (0.1%) for it to drop its
//! corresponding spawn egg.
//!
//! ## Status
//!
//! **Stub module** — Event handler is wired; the actual spawn-egg drop is
//! pending upstream Pumpkin plugin APIs.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to complete this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `EntityDeathEvent` entity reference | Resolve the dead entity (currently only `entity_id: s32` is exposed) |
//! | `Entity.get_type()` | Determine the dead entity's type |
//! | `EntityType.to_spawn_egg()` | Convert entity type to spawn egg item (e.g., `Zombie` → `ZombieSpawnEgg`) |
//! | `EntityDeathEvent` drops list / `World.drop_item_at()` | Add or spawn the egg at the death location |
//! | Random source or `Context.random_f32()` | Roll the 0.1% drop chance |
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
use pumpkin_plugin_api::events::{EntityDeathEvent, EventData, EventHandler, EventPriority};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Drop chance for a spawn egg (0.1%).
pub const DROP_CHANCE: f32 = 0.001;

/// Handles spawn egg drops from mob deaths.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct SpawnEgg;

impl Mechanic for SpawnEgg {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.spawn_egg.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<EntityDeathEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<EntityDeathEvent> for SpawnEgg {
    fn handle(
        &self,
        _server: Server,
        event: EventData<EntityDeathEvent>,
    ) -> EventData<EntityDeathEvent> {
        if !self.enabled() {
            return event;
        }

        // TODO: Implement the spawn-egg drop once the following plugin APIs
        // exist:
        //   1. Random source (for the 0.1% roll against `DROP_CHANCE`).
        //      `rand` doesn't run under WASI without a host-provided entropy
        //      source; a `Context.random_f32()` host call would suffice.
        //   2. Resolve the dead entity from `event.entity_id` — currently the
        //      event only exposes the raw ID, not an `Entity` reference.
        //   3. `Entity.get_type()` returning the entity type so we can pick
        //      the matching spawn egg.
        //   4. `EntityType.to_spawn_egg()` (or a lookup table in the API) to
        //      map the entity type to the spawn egg `Item`.
        //   5. A drops list on the event, or `World.drop_item_at()` to spawn
        //      the egg at the death location.
        //
        // Reference (IllyriaPlus):
        //   if rand.nextFloat() < DROP_CHANCE {
        //       let egg = entity.entity_type.spawnEgg();
        //       world.dropItem(entity.location(), ItemStack(egg));
        //   }

        let _ = event.entity_id;

        event
    }
}

/// Configuration for the spawn egg mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawnEggConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
