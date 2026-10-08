//! Entity health provider — reports a living entity's absorption amount.
//!
//! The main hearts and armor values are read client-side from the entity's
//! own attributes; the server only contributes the absorption portion.

use pumpkin_plugin_api::Entity;

use crate::modules::bridges::jade::buf::Buf;
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeEntityProvider;

/// Reports a living entity's absorption to Jade.
pub(crate) struct EntityHealth;

impl JadeEntityProvider for EntityHealth {
    fn key(&self) -> &'static str {
        "minecraft:entity_health"
    }

    fn write(&self, entity: &Entity, tag: &mut NbtCompound) -> bool {
        let Some(living) = entity.as_living() else {
            return false;
        };
        let absorption = living.get_absorption();

        let mut buf = Buf::new();
        buf.write_f32(absorption);
        let _ = tag.put_byte_array(self.key(), buf.into_vec());
        true
    }
}
