//! Incendium Key item — custom key for the Incendium dimension.
//!
//! ## Status
//!
//! **Stub item** — Cannot be fully implemented due to missing Pumpkin plugin APIs.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to complete this item:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `ItemStack.set_custom_name()` | Set the display name with gradient formatting |
//! | `ItemStack.set_item_model()` | Set custom model for resource pack texture |
//! | `MiniMessage` gradient support | `<gradient:#8B0000:#FF4500:#FF6347>` formatting |
//!
//! `IllyriaPlus` behavior reference:
//! - Base item: `minecraft:trial_key`
//! - Custom name: Gradient from dark red to orange (`<gradient:#8B0000:#FF4500:#FF6347>Incendium Key`)
//! - Custom model: `pumpkinplus:incendium_key` (requires resource pack)

use crate::modules::items::item::Item;
use pumpkin_plugin_api::ItemStack;

/// The Incendium Key custom item.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct IncendiumKey;

impl Item for IncendiumKey {
    fn key(&self) -> &'static str {
        "pumpkinplus:incendium_key"
    }

    fn build(&self) -> ItemStack {
        // TODO: Set custom name and item model once Pumpkin exposes data component APIs.
        // For now, return plain trial key without custom display properties.
        ItemStack::new("minecraft:trial_key", 1)
    }
}
