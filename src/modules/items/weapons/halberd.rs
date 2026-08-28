//! Halberd item builder.
//!
//! Builds a polearm based on the netherite spear with a custom name and a
//! persistent data marker. Advanced component configuration such as attack
//! damage, attack speed, attack range, and item model is left for downstream
//! modules that know the exact Pumpkin component serialization format.

use crate::items::item::Item;
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::common::{NbtTag, NbtTree};
use pumpkin_plugin_api::text::TextComponent;

/// Represents a Halberd.
#[derive(Default)]
pub struct Halberd;

impl Item for Halberd {
    fn key(&self) -> &'static str {
        namespaced_id!("halberd")
    }

    fn build(&self) -> ItemStack {
        let (namespace, key) = self.key_parts();
        let stack = ItemStack::new("minecraft:netherite_spear", 1);
        stack.set_custom_name(Some(TextComponent::text("Halberd")));
        stack.set_custom_data(
            namespace,
            key,
            &NbtTree {
                root: 0,
                tags: vec![NbtTag::Byte(1)],
            },
        );
        stack
    }
}
