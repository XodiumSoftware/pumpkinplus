//! Copper golem waxed provider — reports whether a copper golem is waxed
//! (preventing further oxidation). The payload is a zero-byte marker:
//! its presence means waxed, its absence means un-waxed.
//!
//! **Not implemented** — Pumpkin doesn't currently expose a copper golem's
//! oxidation/waxed state via the typed entity API. The `MobData` variant
//! has no copper-golem branch, and there is no other entity-level accessor.
//!
//! TODO(pumpkin-api): Blocked until Pumpkin exposes copper golem
//! oxidation/waxed state. Upstream PR needed.

use pumpkin_plugin_api::Entity;

use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeEntityProvider;

/// Reports a copper golem's waxed state to Jade (stubbed).
pub(crate) struct Waxed;

impl JadeEntityProvider for Waxed {
    fn key(&self) -> &'static str {
        "minecraft:waxed"
    }

    fn write(&self, _entity: &Entity, _tag: &mut NbtCompound) -> bool {
        // Always decline to write: no waxed-state data is available.
        false
    }
}
