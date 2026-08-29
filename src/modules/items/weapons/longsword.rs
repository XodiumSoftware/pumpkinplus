//! Longsword item builder.
//!
//! Builds a balanced sword based on the netherite sword with a custom name,
//! a persistent data marker, and boosted `minecraft:attribute_modifiers`.
//!
//! ## Item components
//!
//! | Component | Value |
//! |-----------|-------|
//! | Base item | `minecraft:netherite_sword` |
//! | Custom name | `Longsword` |
//! | `minecraft:custom_data` | `{pumpkinplus:longsword: 1b}` |
//! | `minecraft:attribute_modifiers` | `+8.0 attack_damage`, `+1.6 attack_speed` in `mainhand` |
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

/// Represents a Longsword.
#[derive(Default)]
pub struct Longsword;

impl Item for Longsword {
    fn key(&self) -> &'static str {
        namespaced_id!("longsword")
    }

    fn build(&self) -> ItemStack {
        let (namespace, key) = self.key_parts();
        let stack = ItemStack::new("minecraft:netherite_sword", 1);
        stack.set_custom_name(Some(TextComponent::text("Longsword")));
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
