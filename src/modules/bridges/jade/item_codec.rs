//! ItemStack wire-format encoding for Jade payloads.
//!
//! Jade reads ItemStacks in Minecraft's *optional* NMS stream-codec form:
//!
//! ```text
//! VarInt count            // 0 → empty stack, no further fields
//! Identifier item_id      // e.g. `minecraft:diamond_sword`
//! VarInt added_components // number of component entries present
//!   For each: Identifier component_key + encoded_value_bytes
//! VarInt removed_components // number of removed-component ids
//!   For each: Identifier component_key
//! ```
//!
//! Pumpkin's `ItemStack::get_components()` already returns each component's
//! value pre-encoded in NMS stream-codec form (`Vec<u8>` ready to splice
//! into the wire). We map each `DataComponent` variant to its registry key
//! via [`data_component_key`] and concatenate.

use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::wit::pumpkin::plugin::data_components::DataComponent;

use crate::modules::bridges::jade::buf::Buf;

/// Serializes a Pumpkin `ItemStack` into Jade's NMS stream-codec bytes.
///
/// Returns `Vec<u8>` ready to append to a Jade payload buffer.
///
/// Notes on fidelity:
///
/// - `custom_name` is **lossy** on Pumpkin's side: the API strips text
///   styling (colors, bold, hover, click events) before serialization and
///   emits a bare NBT string. Plain-text names round-trip correctly; styled
///   names don't. TODO(pumpkin-api): expose the full `text-component` NBT
///   for `custom_name` upstream.
/// - Pumpkin-specific components (e.g. `sulfur-cube-content`) have no
///   vanilla registry key; we skip them silently.
pub(crate) fn encode_item_stack(stack: &ItemStack) -> Vec<u8> {
    let mut buf = Buf::new();

    let count = stack.get_count();
    buf.write_var_int(i32::from(count));
    if count == 0 {
        return buf.into_vec();
    }

    // Identifier: registry key for the item type.
    buf.write_utf(&stack.get_registry_key());

    // Components: Pumpkin returns each component's bytes pre-encoded in
    // NMS stream-codec form, so we only need to write key + bytes.
    let components = stack.get_components();
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    buf.write_var_int(components.len() as i32);
    for entry in &components {
        let Some(key) = data_component_key(&entry.component) else {
            // Pumpkin-only component with no vanilla counterpart; skip.
            continue;
        };
        buf.write_utf(key);
        buf.write_bytes(&entry.value);
    }

    // No components are removed by Pumpkin's representation.
    buf.write_var_int(0);

    buf.into_vec()
}

/// Writes an `Option<ItemStack>` to the given buffer in NMS optional form.
///
/// This is equivalent to `ItemStack.OPTIONAL_STREAM_CODEC` on the NMS side:
/// `count == 0` collapses to a single VarInt.
pub(crate) fn write_item_stack(buf: &mut Buf, stack: Option<&ItemStack>) {
    match stack {
        Some(s) => buf.write_bytes(&encode_item_stack(s)),
        None => buf.write_var_int(0),
    }
}

/// Returns the vanilla `minecraft:*` registry *path* for a `DataComponent`,
/// or `None` for Pumpkin-specific variants that have no vanilla counterpart.
///
/// This is a one-to-one mapping to the keys used by NMS's
/// `BuiltInRegistries.DATA_COMPONENT_TYPE`. It must be kept in sync with
/// the WIT enum in `pumpkin-plugin-wit/v0.*/data-components.wit`.
///
/// TODO(pumpkin-api): if the WIT adds or renames variants in a future rev,
/// this match will fail to compile. That's intentional — we want a build
/// failure rather than silently sending the wrong key on the wire.
pub(crate) const fn data_component_key(c: &DataComponent) -> Option<&'static str> {
    use DataComponent as D;
    Some(match c {
        D::CustomData => "minecraft:custom_data",
        D::MaxStackSize => "minecraft:max_stack_size",
        D::MaxDamage => "minecraft:max_damage",
        D::Damage => "minecraft:damage",
        D::Unbreakable => "minecraft:unbreakable",
        D::UseEffects => "minecraft:use_effects",
        D::CustomName => "minecraft:custom_name",
        D::MinimumAttackCharge => "minecraft:minimum_attack_charge",
        D::DamageType => "minecraft:damage_type",
        D::ItemName => "minecraft:item_name",
        D::ItemModel => "minecraft:item_model",
        D::Lore => "minecraft:lore",
        D::Rarity => "minecraft:rarity",
        D::Enchantments => "minecraft:enchantments",
        D::CanPlaceOn => "minecraft:can_place_on",
        D::CanBreak => "minecraft:can_break",
        D::AttributeModifiers => "minecraft:attribute_modifiers",
        D::CustomModelData => "minecraft:custom_model_data",
        D::TooltipDisplay => "minecraft:tooltip_display",
        D::RepairCost => "minecraft:repair_cost",
        D::CreativeSlotLock => "minecraft:creative_slot_lock",
        D::EnchantmentGlintOverride => "minecraft:enchantment_glint_override",
        D::IntangibleProjectile => "minecraft:intangible_projectile",
        D::Food => "minecraft:food",
        D::Consumable => "minecraft:consumable",
        D::UseRemainder => "minecraft:use_remainder",
        D::UseCooldown => "minecraft:use_cooldown",
        D::DamageResistant => "minecraft:damage_resistant",
        D::Tool => "minecraft:tool",
        D::Weapon => "minecraft:weapon",
        D::AttackRange => "minecraft:attack_range",
        D::Enchantable => "minecraft:enchantable",
        D::Equippable => "minecraft:equippable",
        D::Repairable => "minecraft:repairable",
        D::Glider => "minecraft:glider",
        D::TooltipStyle => "minecraft:tooltip_style",
        D::DeathProtection => "minecraft:death_protection",
        D::BlocksAttacks => "minecraft:blocks_attacks",
        D::PiercingWeapon => "minecraft:piercing_weapon",
        D::KineticWeapon => "minecraft:kinetic_weapon",
        D::AttackAnimation => "minecraft:attack_animation",
        D::InteractAnimation => "minecraft:interact_animation",
        D::AdditionalTradeCost => "minecraft:additional_trade_cost",
        D::BlockTransformer => "minecraft:block_transformer",
        D::VillagerFood => "minecraft:villager_food",
        D::StoredEnchantments => "minecraft:stored_enchantments",
        D::Dye => "minecraft:dye",
        D::DyedColor => "minecraft:dyed_color",
        D::MapId => "minecraft:map_id",
        D::MapDecorations => "minecraft:map_decorations",
        D::MapPostProcessing => "minecraft:map_post_processing",
        D::ChargedProjectiles => "minecraft:charged_projectiles",
        D::BundleContents => "minecraft:bundle_contents",
        D::PotionContents => "minecraft:potion_contents",
        D::PotionDurationScale => "minecraft:potion_duration_scale",
        D::SuspiciousStewEffects => "minecraft:suspicious_stew_effects",
        D::WritableBookContent => "minecraft:writable_book_content",
        D::WrittenBookContent => "minecraft:written_book_content",
        D::Trim => "minecraft:trim",
        D::DebugStickState => "minecraft:debug_stick_state",
        D::EntityData => "minecraft:entity_data",
        D::BucketEntityData => "minecraft:bucket_entity_data",
        D::BlockEntityData => "minecraft:block_entity_data",
        D::Instrument => "minecraft:instrument",
        D::ProvidesTrimMaterial => "minecraft:provides_trim_material",
        D::OminousBottleAmplifier => "minecraft:ominous_bottle_amplifier",
        D::JukeboxPlayable => "minecraft:jukebox_playable",
        D::ProvidesBannerPatterns => "minecraft:provides_banner_patterns",
        D::Recipes => "minecraft:recipes",
        D::LodestoneTracker => "minecraft:lodestone_tracker",
        D::FireworkExplosion => "minecraft:firework_explosion",
        D::Fireworks => "minecraft:fireworks",
        D::Profile => "minecraft:profile",
        D::NoteBlockSound => "minecraft:note_block_sound",
        D::BannerPatterns => "minecraft:banner_patterns",
        D::BaseColor => "minecraft:base_color",
        D::PotDecorations => "minecraft:pot_decorations",
        D::Container => "minecraft:container",
        D::BlockState => "minecraft:block_state",
        D::Bees => "minecraft:bees",
        D::SulfurCubeContent => return None, // Pumpkin-only, no vanilla key
        D::Lock => "minecraft:lock",
        D::ContainerLoot => "minecraft:container_loot",
        D::BreakSound => "minecraft:break_sound",
        D::Compostable => "minecraft:compostable",
        D::CookingFuel => "minecraft:cooking_fuel",
        D::BrewingFuel => "minecraft:brewing_fuel",
        D::MobVisibility => "minecraft:mob_visibility",
        D::VillagerVariant => "minecraft:villager_variant",
        D::WolfVariant => "minecraft:wolf_variant",
        D::WolfSoundVariant => "minecraft:wolf_sound_variant",
        D::WolfCollar => "minecraft:wolf_collar",
        D::FoxVariant => "minecraft:fox_variant",
        D::SalmonSize => "minecraft:salmon_size",
        D::ParrotVariant => "minecraft:parrot_variant",
        D::TropicalFishPattern => "minecraft:tropical_fish_pattern",
        D::TropicalFishBaseColor => "minecraft:tropical_fish_base_color",
        D::TropicalFishPatternColor => "minecraft:tropical_fish_pattern_color",
        D::MooshroomVariant => "minecraft:mooshroom_variant",
        D::RabbitVariant => "minecraft:rabbit_variant",
        D::PigVariant => "minecraft:pig_variant",
        D::PigSoundVariant => "minecraft:pig_sound_variant",
        D::CowVariant => "minecraft:cow_variant",
        D::CowSoundVariant => "minecraft:cow_sound_variant",
        D::ChickenVariant => "minecraft:chicken_variant",
        D::ChickenSoundVariant => "minecraft:chicken_sound_variant",
        D::ZombieNautilusVariant => "minecraft:zombie_nautilus_variant",
        D::FrogVariant => "minecraft:frog_variant",
        D::HorseVariant => "minecraft:horse_variant",
        D::PaintingVariant => "minecraft:painting_variant",
        D::LlamaVariant => "minecraft:llama_variant",
        D::AxolotlVariant => "minecraft:axolotl_variant",
        D::CatVariant => "minecraft:cat_variant",
        D::CatSoundVariant => "minecraft:cat_sound_variant",
        D::CatCollar => "minecraft:cat_collar",
        D::SheepColor => "minecraft:sheep_color",
        D::ShulkerColor => "minecraft:shulker_color",
        D::ProvidesPotteryPattern => "minecraft:provides_pottery_pattern",
        D::SignTextFront => "minecraft:sign_text_front",
        D::SignTextBack => "minecraft:sign_text_back",
        D::Waxed => "minecraft:waxed",
        D::CushionColor => "minecraft:cushion_color",
    })
}
