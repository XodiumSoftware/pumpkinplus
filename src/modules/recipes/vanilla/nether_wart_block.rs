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
use crate::modules::recipes::recipe::{Ingredient, Recipe, RecipeItemStack, ShapelessRecipe};
use crate::namespaced_id;

/// Handles nether-wart-block-to-nether-wart shapeless recipe.
#[derive(Default)]
pub struct NetherWartBlock;

impl Recipe for NetherWartBlock {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.nether_wart_block)
    }

    fn shapeless(&self) -> Vec<ShapelessRecipe> {
        vec![ShapelessRecipe {
            id: namespaced_id!("nether_wart_block_breakdown").into(),
            ingredients: vec![Ingredient::Item {
                id: "minecraft:nether_wart_block".into(),
            }],
            result: RecipeItemStack {
                id: "minecraft:nether_wart".into(),
                count: 9,
            },
        }]
    }
}
