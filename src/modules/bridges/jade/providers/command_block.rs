//! Command block provider — reports the command text to Jade, truncated to
//! 40 characters. Only op players can see it; the client suppresses the
//! request entirely for anyone without gamemaster-block access.

use pumpkin_plugin_api::wit::pumpkin::plugin::block_entity::BlockEntityType;
use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::World;

use crate::modules::bridges::jade::buf::Buf;
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeBlockProvider;

/// Maximum command length before truncation.
const MAX_LENGTH: usize = 40;

/// Reports the command string of a command block to Jade.
pub(crate) struct CommandBlock;

impl JadeBlockProvider for CommandBlock {
    fn key(&self) -> &'static str {
        "minecraft:command_block"
    }

    fn write(&self, world: &World, pos: BlockPos, tag: &mut NbtCompound) -> bool {
        let Some(BlockEntityType::CommandBlockEntity(cb)) = world.get_block_entity(pos) else {
            return false;
        };

        let command = cb.command();
        let truncated = if command.chars().count() > MAX_LENGTH {
            // Take the first MAX_LENGTH-3 chars and append "...".
            let head: String = command.chars().take(MAX_LENGTH - 3).collect();
            format!("{head}...")
        } else {
            command
        };

        let mut buf = Buf::new();
        buf.write_utf(&truncated);
        let _ = tag.put_byte_array(self.key(), buf.into_vec());
        true
    }
}
