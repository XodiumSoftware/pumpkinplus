//! Zombie villager conversion provider — reports remaining ticks while a
//! zombie villager is being cured.
//!
//! **Not implemented** — Pumpkin's `MobData::Zombie(ZombieData)` only
//! carries `is_baby` and `can_break_doors`; there's no `is_converting` or
//! `conversion_time` field.
//!
//! TODO(pumpkin-api): Blocked until Pumpkin adds conversion state to
//! `ZombieData` (or a dedicated `ZombieVillager` mob-data variant).
//! Upstream PR needed.

use pumpkin_plugin_api::Entity;

use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::provider::JadeEntityProvider;

/// Reports a zombie villager's remaining conversion time (stubbed).
pub(crate) struct ZombieVillager;

impl JadeEntityProvider for ZombieVillager {
    fn key(&self) -> &'static str {
        "minecraft:zombie_villager"
    }

    fn write(&self, _entity: &Entity, _tag: &mut NbtCompound) -> bool {
        // Always decline to write: conversion state is not exposed.
        false
    }
}
