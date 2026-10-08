//! Provider contracts for Jade.
//!
//! A *provider* answers one specific question about a block or entity —
//! e.g. "how many bees are in this beehive?" or "what's the brewing
//! progress of this brewing stand?" — and writes a small NBT payload under
//! its key into the response compound. The provider's key is also the
//! string advertised to the client in the `jade:server_handshake`.
//!
//! Providers are intentionally *single-purpose*, mirroring the original
//! Paper module layout. The `Jade` bridge routes incoming requests to the
//! providers registered in its two static lists.

use pumpkin_plugin_api::Entity;
use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::World;

use crate::modules::bridges::jade::nbt::NbtCompound;

/// Provider for block-targeted Jade queries.
pub(crate) trait JadeBlockProvider: Send + Sync {
    /// Provider identifier advertised in the handshake, e.g. `"minecraft:beehive"`.
    fn key(&self) -> &'static str;

    /// Writes NBT for this block into `tag`.
    ///
    /// Returns `true` if any data was written; `false` to skip this provider
    /// for the given block (e.g., block isn't the right type).
    fn write(&self, world: &World, pos: BlockPos, tag: &mut NbtCompound) -> bool;
}

/// Provider for entity-targeted Jade queries.
pub(crate) trait JadeEntityProvider: Send + Sync {
    /// Provider identifier advertised in the handshake, e.g. `"minecraft:entity_health"`.
    fn key(&self) -> &'static str;

    /// Writes NBT for this entity into `tag`.
    ///
    /// Returns `true` if any data was written; `false` to skip this provider
    /// for the given entity (e.g., not living, not the right kind).
    fn write(&self, entity: &Entity, tag: &mut NbtCompound) -> bool;
}
