//! Reusable item builder system for `PumpkinPlus`.
//!
//! Provides the [`Item`] trait for defining custom item stacks that can be
//! created on demand by commands, recipes, or other modules. Items defined here
//! are plain builders: they are not registered with the server because Pumpkin
//! currently exposes no custom item registry API.
//!
//! ## Available Items
//!
//! | Item        | Path                                                     | Base Item              | Attack damage | Attack speed |
//! |-------------|----------------------------------------------------------|------------------------|---------------|--------------|
//! | `Greatsword`| [`crate::items::weapons::greatsword::Greatsword`]        | `minecraft:netherite_sword` | `+10.0` | `+1.2` |
//! | `Longsword` | [`crate::items::weapons::longsword::Longsword`]          | `minecraft:netherite_sword` | `+8.0`  | `+1.6` |
//! | `Halberd`   | [`crate::items::weapons::halberd::Halberd`]              | `minecraft:netherite_spear` | `+11.0` | `+0.8` |

use pumpkin_plugin_api::ItemStack;

/// A trait representing a reusable custom item builder.
///
/// Implementors define a stable namespaced key and a method that constructs
/// the configured [`ItemStack`]. Other modules can call [`Item::build`] to
/// obtain a fresh stack whenever one is needed.
pub trait Item {
    /// The namespaced registry key identifying this item type.
    ///
    /// Used for custom data lookups and, when component serialization is added,
    /// for the `minecraft:item_model` component.
    fn key(&self) -> &'static str;

    /// Splits [`Item::key`] into its `(namespace, path)` components.
    ///
    /// Falls back to `(key, "")` if the key is not namespaced.
    #[must_use]
    fn key_parts(&self) -> (&'static str, &'static str) {
        self.key().split_once(':').unwrap_or((self.key(), ""))
    }

    /// Builds and returns the configured [`ItemStack`].
    fn build(&self) -> ItemStack;
}
