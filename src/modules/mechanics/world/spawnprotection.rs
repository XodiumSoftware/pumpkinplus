//! Spawn protection module — keeps hostile mobs out of the spawn protection area.
//!
//! ## What it does
//!
//! While inside the spawn protection area (a square of `(2 * radius + 1)` blocks
//! centered on the world spawn point), the module:
//!
//! - Cancels hostile-mob spawns inside the area.
//!
//! The protected square uses the same Chebyshev distance check as vanilla
//! Minecraft: a position is inside the protected area when both
//! `|x - spawn.x| <= radius` and `|z - spawn.z| <= radius` are true.
//!
//! ## Radius note
//!
//! Unlike Bukkit, the Pumpkin plugin API does not currently expose the server's
//! `spawn-protection` setting from `server.properties`. The value lives on the
//! Rust server core (`Server.basic_config.spawn_protection`) but has no WIT
//! host function, so WASM plugins cannot read it. This module therefore uses
//! its own `radius` config field (default `16`, matching the vanilla default).
//!
//! **TODO:** Once the upstream API exposes the server's spawn-protection
//! radius (e.g. via `Context.get_server_config()` or a `Server` extension),
//! remove the `radius` config field and read it directly here.
//!
//! ## Not yet implemented
//!
//! The following behaviors require API support that is not currently available
//! or not practical, and are intentionally left out:
//!
//! - **Damage protection** for players/villagers/tamed animals: the
//!   `EntityDamageEvent` carries only an entity id (no entity handle or
//!   position), so resolving the position requires a full-world entity scan on
//!   every damage event.
//! - **Chunk-load despawn** of monsters: the `ChunkLoadEvent` carries only
//!   chunk coordinates (no entity list), so this too would require a
//!   full-world entity scan per chunk load.
//! - **Periodic sweep**: requires a scheduler with repeating timers, which is
//!   not currently available in the plugin API.
//!
//! ## Configuration
//!
//! | Field     | Default | Description                                         |
//! |-----------|---------|-----------------------------------------------------|
//! | `enabled` | `false` | Whether this module is active                       |
//! | `radius`  | `16`    | Spawn protection radius in blocks (vanilla default) |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::events::{CreatureSpawnEvent, EventData, EventHandler, EventPriority};
use pumpkin_plugin_api::world::World;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles spawn protection (hostile-mob spawn cancellation).
#[derive(Default)]
pub struct SpawnProtection;

impl Mechanic for SpawnProtection {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.spawnprotection.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<CreatureSpawnEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<CreatureSpawnEvent> for SpawnProtection {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<CreatureSpawnEvent>,
    ) -> EventData<CreatureSpawnEvent> {
        if !self.enabled() {
            return event;
        }

        if !is_hostile_entity_type(&event.entity_type) {
            return event;
        }

        let config: SpawnProtectionConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.spawnprotection)
            .unwrap_or_default();

        if is_protected_position(event.position, &event.target_world, config.radius) {
            event.cancelled = true;
        }

        event
    }
}

/// Checks whether a position is inside the world's spawn protection area.
///
/// Resolves the world's spawn location via [`World::get_spawn_location`] and
/// performs the Chebyshev distance check against `radius`. Returns `false`
/// when `radius <= 0`.
fn is_protected_position(position: (f64, f64, f64), world: &World, radius: i32) -> bool {
    if radius <= 0 {
        return false;
    }
    let spawn = world.get_spawn_location().pos;
    let dx = (position.0 - f64::from(spawn.x)).abs();
    let dz = (position.2 - f64::from(spawn.z)).abs();
    let radius = f64::from(radius);
    dx <= radius && dz <= radius
}

/// Checks whether an entity type identifier refers to a vanilla hostile mob
/// (the equivalent of Bukkit's `Monster` interface).
///
/// The Pumpkin plugin API exposes the spawning entity's type only as a string
/// (e.g. `minecraft:zombie`), so hostile mobs are matched against this list.
/// New hostile mobs added to Minecraft will need to be added here.
fn is_hostile_entity_type(entity_type: &str) -> bool {
    let name = entity_type
        .strip_prefix("minecraft:")
        .unwrap_or(entity_type);
    matches!(
        name,
        "blaze"
            | "bogged"
            | "breeze"
            | "cave_spider"
            | "creaking"
            | "creeper"
            | "drowned"
            | "elder_guardian"
            | "ender_dragon"
            | "enderman"
            | "endermite"
            | "evoker"
            | "ghast"
            | "giant"
            | "guardian"
            | "hoglin"
            | "husk"
            | "illusioner"
            | "magma_cube"
            | "phantom"
            | "piglin"
            | "piglin_brute"
            | "pillager"
            | "ravager"
            | "shulker"
            | "silverfish"
            | "skeleton"
            | "slime"
            | "spider"
            | "stray"
            | "vex"
            | "vindicator"
            | "warden"
            | "witch"
            | "wither"
            | "wither_skeleton"
            | "zoglin"
            | "zombie"
            | "zombie_villager"
            | "zombified_piglin"
    )
}

/// Configuration for the spawn protection mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnProtectionConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Spawn protection radius in blocks. The protected area is a square of
    /// `(2 * radius + 1)` blocks centered on the world spawn point (vanilla default: 16).
    pub radius: i32,
}

impl Default for SpawnProtectionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            radius: 16,
        }
    }
}
