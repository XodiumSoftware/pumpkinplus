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
//!
//! > **Note:** Pumpkin's host currently does not implement the
//! > `minecraft:attribute_modifiers` data-component codec, so this component
//! > is attached as an empty serialized payload and will be ignored at runtime
//! > until server-side serialization is implemented. The builder still calls
//! > `set_component` so the plugin is ready once support lands.

use crate::items::item::Item as ItemTrait;
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::common::{NbtTag, NbtTree};
use pumpkin_plugin_api::data_components::DataComponent;
use pumpkin_plugin_api::item::{Item, ItemStackExt};
use pumpkin_plugin_api::text::TextComponent;

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
        // Empty payload because Pumpkin's `attribute_modifiers` codec is not yet
        // implemented. The component is registered here so the builder is ready
        // once the host supports serializing this component.
        stack.set_component(DataComponent::AttributeModifiers, &[]);
        stack
    }
}
