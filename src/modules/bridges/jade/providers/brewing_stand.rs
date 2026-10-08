//! Brewing stand provider — reports remaining blaze-powder fuel and, while
//! brewing, the time left until the current batch finishes.

use pumpkin_plugin_api::wit::pumpkin::plugin::block_entity::BlockEntityType;
use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::World;

use crate::modules::bridges::jade::buf::Buf;
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeBlockProvider;

/// Reports brewing-stand state to Jade.
pub(crate) struct BrewingStand;

impl JadeBlockProvider for BrewingStand {
    fn key(&self) -> &'static str {
        "minecraft:brewing_stand"
    }

    fn write(&self, world: &World, pos: BlockPos, tag: &mut NbtCompound) -> bool {
        let Some(BlockEntityType::BrewingStandBlockEntity(brewing)) = world.get_block_entity(pos)
        else {
            return false;
        };

        let fuel = brewing.get_fuel();
        let brew_time = brewing.get_brew_time();

        let mut buf = Buf::new();
        buf.write_var_int(fuel);
        buf.write_var_int(brew_time);
        let _ = tag.put_byte_array(self.key(), buf.into_vec());
        true
    }
}
