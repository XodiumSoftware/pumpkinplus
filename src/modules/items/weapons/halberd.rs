//! Halberd item builder.
//!
//! Builds a polearm based on the netherite spear with a custom name, a
//! persistent data marker, and boosted `minecraft:attribute_modifiers`.
//!
//! ## Item components
//!
//! | Component | Value |
//! |-----------|-------|
//! | Base item | `minecraft:netherite_spear` |
//! | Custom name | `Halberd` |
//! | `minecraft:custom_data` | `{pumpkinplus:halberd: 1b}` |
//! | `minecraft:attribute_modifiers` | `+11.0 attack_damage`, `+0.8 attack_speed`, `+0.5 entity_interaction_range` in `mainhand` |

use crate::items::item::Item as ItemTrait;
use crate::namespaced_id;
use pumpkin_plugin_api::common::{NbtTag, NbtTree};
use pumpkin_plugin_api::item::{Item, ItemStackExt};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{
    Attribute, AttributeModifier, AttributeModifierSlot, ItemAttributeModifier, ItemStack,
};

/// Represents a Halberd.
#[derive(Default)]
pub struct Halberd;

impl ItemTrait for Halberd {
    fn key(&self) -> &'static str {
        namespaced_id!("halberd")
    }

    fn build(&self) -> ItemStack {
        let (namespace, key) = self.key_parts();
        let stack = ItemStack::of(Item::NetheriteSpear, 1);
        stack.set_custom_name(Some(TextComponent::text("Halberd")));
        stack.set_custom_data(
            namespace,
            key,
            &NbtTree {
                root: 0,
                tags: vec![NbtTag::Byte(1)],
            },
        );
        stack.add_attribute_modifier(&ItemAttributeModifier::new(
            Attribute::AttackDamage,
            AttributeModifier::add(format!("{namespace}:{key}.attack_damage"), 11.0),
            AttributeModifierSlot::MainHand,
        ));
        stack.add_attribute_modifier(&ItemAttributeModifier::new(
            Attribute::AttackSpeed,
            AttributeModifier::add(format!("{namespace}:{key}.attack_speed"), 0.8),
            AttributeModifierSlot::MainHand,
        ));
        stack.add_attribute_modifier(&ItemAttributeModifier::new(
            Attribute::EntityInteractionRange,
            AttributeModifier::add(format!("{namespace}:{key}.entity_interaction_range"), 0.5),
            AttributeModifierSlot::MainHand,
        ));
        stack
    }
}
