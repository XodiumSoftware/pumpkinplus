//! Incendium Key item — custom key for the Incendium dimension.
//!
//! ## Status
//!
//! **Partial implementation** — Custom name works with gradient, item model
//! requires resource pack assets.
//!
//! ## Missing / Limited APIs
//!
//! | API | Status | Purpose |
//! |-----|--------|---------|
//! | `ItemStack.set_custom_name()` | ✅ Available via data component | Set the display name |
//! | `ItemStack.set_item_model()` | ⚠️ Partial | Component exists but requires resource pack |
//! | `MiniMessage` gradient | ❌ Not supported | `minimessage-impl` crate lacks gradient tag support |
//!
//! `IllyriaPlus` behavior reference:
//! - Base item: `minecraft:trial_key`
//! - Custom name: Gradient from dark red to orange
//! - Custom model: `pumpkinplus:incendium_key` (requires resource pack)

use crate::modules::items::item::Item;
use crate::namespaced_id;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Item as McItem, ItemStack, data_components::DataComponent};

/// The Incendium Key custom item.
#[derive(Default)]
pub struct IncendiumKey;

impl Item for IncendiumKey {
    fn key(&self) -> &'static str {
        namespaced_id!("incendium_key")
    }

    fn build(&self) -> ItemStack {
        let item = ItemStack::new(McItem::TrialKey.resource_location(), 1);

        // TODO: Use MiniMessage gradient once `minimessage-impl` crate supports it:
        // `<gradient:#8B0000:#FF4500:#FF6347>Incendium Key</gradient>`
        // For now, use plain custom name component.
        let name = TextComponent::text("Incendium Key");
        item.set_component(DataComponent::CustomName, &name.encode());

        // TODO: Set item model when resource pack assets are available
        // item.set_component(DataComponent::ItemModel, self.key().as_bytes());

        item
    }
}
