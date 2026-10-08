//! Hopper lock provider — reports whether a hopper is currently disabled by
//! redstone power. Jade renders a locked indicator over the hopper's name
//! when the payload is `true`.

use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::World;

use crate::modules::bridges::jade::buf::Buf;
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeBlockProvider;

/// Reports a hopper's lock state to Jade.
pub(crate) struct HopperLock;

impl JadeBlockProvider for HopperLock {
    fn key(&self) -> &'static str {
        "minecraft:hopper_lock"
    }

    fn write(&self, world: &World, pos: BlockPos, tag: &mut NbtCompound) -> bool {
        // Hopper "enabled" is a block-state property, not block entity data.
        // A hopper is *enabled* when NOT receiving redstone power.
        //
        // TODO(pumpkin-api): We'd like to check the block is actually a
        // hopper first so we return false for non-hoppers, but the current
        // block-state record doesn't carry the block's registry key in a
        // convenient form here. For now we look up the `enabled` property
        // and return false if it's absent.
        let state = world.get_block_state(pos);
        let Some(enabled) = state
            .properties
            .iter()
            .find(|(k, _)| k == "enabled")
            .map(|(_, v)| v == "true")
        else {
            return false;
        };

        // Jade expects "locked" — i.e. inverted.
        let locked = !enabled;

        let mut buf = Buf::new();
        buf.write_bool(locked);
        let _ = tag.put_byte_array(self.key(), buf.into_vec());
        true
    }
}
