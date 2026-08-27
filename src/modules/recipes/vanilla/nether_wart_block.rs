//! Nether wart block breakdown recipe.
//!
//! Provides a shapeless crafting recipe that breaks a nether wart block
//! back into 9 nether warts.
//!
//! ## Recipe
//!
//! | Input              | Output       | Count |
//! |--------------------|--------------|-------|
//! | Nether Wart Block  | Nether Wart  | 9     |

use crate::config::ConfigManager;
use crate::modules::recipes::recipe::{Recipe, RecipeEntry};
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::recipe::{RecipeCategory, ShapelessRecipeBuilder};

/// Handles nether-wart-block-to-nether-wart shapeless recipe.
#[derive(Default)]
pub struct NetherWartBlock;

impl Recipe for NetherWartBlock {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.nether_wart_block)
    }

    fn recipes(&self) -> Vec<RecipeEntry> {
        vec![RecipeEntry::Shapeless(
            ShapelessRecipeBuilder::new(
                namespaced_id!("nether_wart_block_breakdown"),
                ItemStack::new("minecraft:nether_wart", 9),
            )
            .ingredient("minecraft:nether_wart_block")
            .category(RecipeCategory::Misc),
        )]
    }
}
