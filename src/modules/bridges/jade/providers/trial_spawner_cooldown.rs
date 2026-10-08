//! Trial spawner cooldown provider — reports remaining cooldown ticks.
//!
//! **Not implemented** — Pumpkin's `TrialSpawnerBlockEntity` only exposes
//! `get_block_entity()`; cooldown fields (`cooldown_end`, etc.) are not
//! available through the typed API.
//!
//! TODO(pumpkin-api): Blocked until Pumpkin exposes trial-spawner
//! cooldown state on `TrialSpawnerBlockEntity`. Upstream PR needed.

use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::World;

use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeBlockProvider;

/// Reports a trial spawner's remaining cooldown to Jade (stubbed).
pub(crate) struct TrialSpawnerCooldown;

impl JadeBlockProvider for TrialSpawnerCooldown {
    fn key(&self) -> &'static str {
        "minecraft:mob_spawner.cooldown"
    }

    fn write(&self, _world: &World, _pos: BlockPos, _tag: &mut NbtCompound) -> bool {
        // Always decline to write: no cooldown data is available.
        false
    }
}
