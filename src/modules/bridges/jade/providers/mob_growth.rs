//! Mob growth provider — reports the remaining ticks until a baby mob
//! grows into an adult. Skipped for adults (non-positive ages).

use pumpkin_plugin_api::Entity;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::MobData;

use crate::modules::bridges::jade::buf::Buf;
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeEntityProvider;

/// Reports a baby mob's remaining growth time to Jade.
pub(crate) struct MobGrowth;

impl JadeEntityProvider for MobGrowth {
    fn key(&self) -> &'static str {
        "minecraft:mob_growth"
    }

    fn write(&self, entity: &Entity, tag: &mut NbtCompound) -> bool {
        let Some(mob) = entity.as_mob() else {
            return false;
        };
        let MobData::Ageable(data) = mob.get_mob_data() else {
            return false;
        };

        // In vanilla, babies have negative `age` counting up to 0;
        // `-age` gives the ticks remaining.
        let remaining = -data.age;
        if remaining <= 0 {
            return false;
        }

        let mut buf = Buf::new();
        buf.write_var_int(remaining);
        let _ = tag.put_byte_array(self.key(), buf.into_vec());
        true
    }
}
