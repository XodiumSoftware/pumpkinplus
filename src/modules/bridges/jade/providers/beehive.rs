//! Beehive provider — reports the number of bees occupying a beehive/bee nest.
//!
//! Honey level is read client-side from the block state; the server only
//! contributes the bee count. Jade's convention: positive when the hive is
//! full, negative when partially full.
//!
//! TODO(pumpkin-api): Pumpkin's `BeehiveBlockEntity` only exposes
//! `get_bee_count`. There's no `is_full` accessor, so we always send a
//! positive count here — Jade will render that as "full" which is wrong
//! for hives with 1–2 bees. Either expose `is_full` upstream or read the
//! `honey_level` block-state property as a proxy.

use pumpkin_plugin_api::wit::pumpkin::plugin::block_entity::BlockEntityType;
use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::World;

use crate::modules::bridges::jade::buf::Buf;
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeBlockProvider;

/// Reports bee count for beehives and bee nests.
pub(crate) struct Beehive;

impl JadeBlockProvider for Beehive {
    fn key(&self) -> &'static str {
        "minecraft:beehive"
    }

    fn write(&self, world: &World, pos: BlockPos, tag: &mut NbtCompound) -> bool {
        let Some(BlockEntityType::BeehiveBlockEntity(beehive)) = world.get_block_entity(pos) else {
            return false;
        };

        let bee_count = beehive.get_bee_count();
        let signed = i8::try_from(bee_count).unwrap_or(i8::MAX);

        let mut buf = Buf::new();
        buf.write_byte(signed.to_be_bytes()[0]);
        let _ = tag.put_byte_array(self.key(), buf.into_vec());
        true
    }
}
