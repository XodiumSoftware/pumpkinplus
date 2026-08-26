//! Serde remote definition for the upstream `EntityType` enum.
//!
//! `pumpkin_plugin_api::world::EntityType` is generated from WIT and does not
//! implement `Serialize`/`Deserialize`. This module defines a matching enum
//! annotated with `#[serde(remote = "...")]`, which lets config fields hold
//! the real upstream type while serde uses the local definition for conversion.
//!
//! For a single value:
//!
//! ```ignore
//! #[serde(with = "crate::mirror_types::entity_type::EntityTypeDef")]
//! pub entity: EntityType,
//! ```
//!
//! For a vector of values:
//!
//! ```ignore
//! #[serde(with = "crate::mirror_types::entity_type::entity_type_vec")]
//! pub entities: Vec<EntityType>,
//! ```

use crate::mirror_enum;

mirror_enum! {
    /// Serde remote definition for `pumpkin_plugin_api::world::EntityType`.
    ///
    /// Covers all entity types exposed by the Pumpkin plugin API.
    #[serde(remote = "pumpkin_plugin_api::world::EntityType")]
    pub enum EntityTypeDef from pumpkin_plugin_api::world::EntityType {
        AcaciaBoat,
        AcaciaChestBoat,
        Allay,
        AreaEffectCloud,
        Armadillo,
        ArmorStand,
        Arrow,
        Axolotl,
        BambooChestRaft,
        BambooRaft,
        Bat,
        Bee,
        BirchBoat,
        BirchChestBoat,
        Blaze,
        BlockDisplay,
        Bogged,
        Breeze,
        BreezeWindCharge,
        Camel,
        CamelHusk,
        Cat,
        CaveSpider,
        CherryBoat,
        CherryChestBoat,
        ChestMinecart,
        Chicken,
        Cod,
        CommandBlockMinecart,
        CopperGolem,
        Cow,
        Creaking,
        Creeper,
        DarkOakBoat,
        DarkOakChestBoat,
        Dolphin,
        Donkey,
        DragonFireball,
        Drowned,
        Egg,
        ElderGuardian,
        EndCrystal,
        EnderDragon,
        Enderman,
        Endermite,
        EnderPearl,
        Evoker,
        EvokerFangs,
        ExperienceBottle,
        ExperienceOrb,
        EyeOfEnder,
        FallingBlock,
        Fireball,
        FireworkRocket,
        FishingBobber,
        Fox,
        Frog,
        FurnaceMinecart,
        Ghast,
        Giant,
        GlowItemFrame,
        GlowSquid,
        Goat,
        Guardian,
        HappyGhast,
        Hoglin,
        HopperMinecart,
        Horse,
        Husk,
        Illusioner,
        Interaction,
        IronGolem,
        Item,
        ItemDisplay,
        ItemFrame,
        JungleBoat,
        JungleChestBoat,
        LeashKnot,
        LightningBolt,
        LingeringPotion,
        Llama,
        LlamaSpit,
        MagmaCube,
        MangroveBoat,
        MangroveChestBoat,
        Mannequin,
        Marker,
        Minecart,
        Mooshroom,
        Mule,
        Nautilus,
        OakBoat,
        OakChestBoat,
        Ocelot,
        OminousItemSpawner,
        Painting,
        PaleOakBoat,
        PaleOakChestBoat,
        Panda,
        Parched,
        Parrot,
        Phantom,
        Pig,
        Piglin,
        PiglinBrute,
        Pillager,
        Player,
        PolarBear,
        Pufferfish,
        Rabbit,
        Ravager,
        Salmon,
        Sheep,
        Shulker,
        ShulkerBullet,
        Silverfish,
        Skeleton,
        SkeletonHorse,
        Slime,
        SmallFireball,
        Sniffer,
        Snowball,
        SnowGolem,
        SpawnerMinecart,
        SpectralArrow,
        Spider,
        SplashPotion,
        SpruceBoat,
        SpruceChestBoat,
        Squid,
        Stray,
        Strider,
        SulfurCube,
        Tadpole,
        TextDisplay,
        Tnt,
        TntMinecart,
        TraderLlama,
        Trident,
        TropicalFish,
        Turtle,
        Vex,
        Villager,
        Vindicator,
        WanderingTrader,
        Warden,
        WindCharge,
        Witch,
        Wither,
        WitherSkeleton,
        WitherSkull,
        Wolf,
        Zoglin,
        Zombie,
        ZombieHorse,
        ZombieNautilus,
        ZombieVillager,
        ZombifiedPiglin,
    }
}

/// Helper module for serializing/deserializing `Vec<EntityType>` via the
/// remote `EntityTypeDef` definition.
pub mod entity_type_vec {
    use pumpkin_plugin_api::world::EntityType;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Wrapper that serializes an upstream `EntityType` through `EntityTypeDef`.
    struct SerializeRef<'a>(&'a EntityType);

    impl Serialize for SerializeRef<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            super::EntityTypeDef::serialize(self.0, serializer)
        }
    }

    /// Wrapper that deserializes into an upstream `EntityType`.
    struct DeserializeOwned(EntityType);

    impl<'de> Deserialize<'de> for DeserializeOwned {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            super::EntityTypeDef::deserialize(deserializer).map(Self)
        }
    }

    /// Serializes a slice of upstream `EntityType` values.
    pub fn serialize<S: Serializer>(
        value: &[EntityType],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for entity_type in value {
            seq.serialize_element(&SerializeRef(entity_type))?;
        }
        seq.end()
    }

    /// Deserializes a vector of upstream `EntityType` values.
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<EntityType>, D::Error> {
        let wrapped = Vec::<DeserializeOwned>::deserialize(deserializer)?;
        Ok(wrapped.into_iter().map(|w| w.0).collect())
    }
}
