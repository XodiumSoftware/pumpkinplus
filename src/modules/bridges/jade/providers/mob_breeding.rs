//! Mob breeding provider — reports breeding status to Jade.
//!
//! `-1` while an animal is in love mode; otherwise the breeding cooldown
//! in ticks (positive values only). Non-ageable mobs are skipped.

use pumpkin_plugin_api::Entity;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::MobData;

use crate::modules::bridges::jade::buf::Buf;
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeEntityProvider;

/// Reports a mob's breeding status to Jade.
pub(crate) struct MobBreeding;

impl JadeEntityProvider for MobBreeding {
    fn key(&self) -> &'static str {
        "minecraft:mob_breeding"
    }

    fn write(&self, entity: &Entity, tag: &mut NbtCompound) -> bool {
        let Some(mob) = entity.as_mob() else {
            return false;
        };
        let MobData::Ageable(data) = mob.get_mob_data() else {
            return false;
        };

        let value = if data.in_love_ticks > 0 {
            -1
        } else if data.age > 0 {
            data.age
        } else {
            return false;
        };

        let mut buf = Buf::new();
        buf.write_var_int(value);
        let _ = tag.put_byte_array(self.key(), buf.into_vec());
        true
    }
}
