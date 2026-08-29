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
//!
//! > **Note:** Pumpkin's host currently does not implement the
//! > `minecraft:attribute_modifiers` data-component codec, so this component
//! > is attached as an empty serialized payload and will be ignored at runtime
//! > until server-side serialization is implemented. The builder still calls
//! > `set_component` so the plugin is ready once support lands.

use crate::items::item::Item;
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::common::{NbtTag, NbtTree};
use pumpkin_plugin_api::data_components::DataComponent;
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
        // Empty payload because Pumpkin's `attribute_modifiers` codec is not yet
        // implemented. The component is registered here so the builder is ready
        // once the host supports serializing this component.
        stack.set_component(DataComponent::AttributeModifiers, &[]);
        stack
    }
}
