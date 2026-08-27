//! Ice breakdown recipes.
//!
//! Provides shapeless crafting recipes that break packed ice and blue ice
//! down into their lower-tier ice variants.
//!
//! ## Recipes
//!
//! | Input       | Output        | Count |
//! |-------------|---------------|-------|
//! | Blue Ice    | Packed Ice    | 9     |
//! | Packed Ice  | Ice           | 9     |

use crate::config::ConfigManager;
use crate::modules::recipes::recipe::{Recipe, RecipeEntry};
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::recipe::{RecipeCategory, ShapelessRecipeBuilder};

/// Handles ice breakdown shapeless recipes.
#[derive(Default)]
pub struct IceBreakdown;

impl Recipe for IceBreakdown {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.ice_breakdown)
    }

    fn recipes(&self) -> Vec<RecipeEntry> {
        vec![
            RecipeEntry::Shapeless(
                ShapelessRecipeBuilder::new(
                    namespaced_id!("blue_ice_breakdown"),
                    ItemStack::new("minecraft:packed_ice", 9),
                )
                .ingredient("minecraft:blue_ice")
                .category(RecipeCategory::Misc),
            ),
            RecipeEntry::Shapeless(
                ShapelessRecipeBuilder::new(
                    namespaced_id!("packed_ice_breakdown"),
                    ItemStack::new("minecraft:ice", 9),
                )
                .ingredient("minecraft:packed_ice")
                .category(RecipeCategory::Misc),
            ),
        ]
    }
}
