//! Greatsword item builder.
//!
//! Builds a heavy two-handed sword based on the netherite sword with a custom
//! name, a persistent data marker, and boosted `minecraft:attribute_modifiers`.
//!
//! ## Item components
//!
//! | Component | Value |
//! |-----------|-------|
//! | Base item | `minecraft:netherite_sword` |
//! | Custom name | `Greatsword` |
//! | `minecraft:custom_data` | `{pumpkinplus:greatsword: 1b}` |
//! | `minecraft:attribute_modifiers` | `+10.0 attack_damage`, `+1.2 attack_speed` in `mainhand` |

use crate::items::item::Item as ItemTrait;
use crate::namespaced_id;
use pumpkin_plugin_api::common::{NbtTag, NbtTree};
use pumpkin_plugin_api::item::{Item, ItemStackExt};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{
    Attribute, AttributeModifier, AttributeModifierSlot, ItemAttributeModifier, ItemStack,
};

/// Represents a Greatsword.
#[derive(Default)]
pub struct Greatsword;

impl ItemTrait for Greatsword {
    fn key(&self) -> &'static str {
        namespaced_id!("greatsword")
    }

    fn build(&self) -> ItemStack {
        let (namespace, key) = self.key_parts();
        let stack = ItemStack::of(Item::NetheriteSword, 1);
        stack.set_custom_name(Some(TextComponent::text("Greatsword")));
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
            AttributeModifier::add(format!("{namespace}:{key}.attack_damage"), 10.0),
            AttributeModifierSlot::MainHand,
        ));
        stack.add_attribute_modifier(&ItemAttributeModifier::new(
            Attribute::AttackSpeed,
            AttributeModifier::add(format!("{namespace}:{key}.attack_speed"), 1.2),
            AttributeModifierSlot::MainHand,
        ));
        stack
    }
}
