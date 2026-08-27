//! Wool to string recipe.
//!
//! Provides a shapeless crafting recipe that converts any wool block into
//! 4 string.
//!
//! ## Recipe
//!
//! | Input | Output  | Count |
//! |-------|---------|-------|
//! | Wool  | String  | 4     |

use crate::config::ConfigManager;
use crate::modules::recipes::recipe::{Recipe, RecipeEntry};
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::recipe::{RecipeCategory, ShapelessRecipeBuilder};

/// Handles wool-to-string shapeless recipe.
#[derive(Default)]
pub struct WoolToString;

impl Recipe for WoolToString {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.wool_to_string)
    }

    fn recipes(&self) -> Vec<RecipeEntry> {
        vec![RecipeEntry::Shapeless(
            ShapelessRecipeBuilder::new(
                namespaced_id!("wool_to_string"),
                ItemStack::new("minecraft:string", 4),
            )
            .ingredient("#minecraft:wool")
            .category(RecipeCategory::Misc),
        )]
    }
}
