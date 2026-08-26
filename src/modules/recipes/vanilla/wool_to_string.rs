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
use crate::modules::recipes::recipe::{Ingredient, Recipe, RecipeItemStack, ShapelessRecipe};

/// Handles wool-to-string shapeless recipe.
#[derive(Default)]
pub struct WoolToString;

impl Recipe for WoolToString {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.wool_to_string)
    }

    fn shapeless(&self) -> Vec<ShapelessRecipe> {
        vec![ShapelessRecipe {
            id: "pumpkinplus:wool_to_string".into(),
            ingredients: vec![Ingredient::Tag {
                id: "minecraft:wool".into(),
            }],
            result: RecipeItemStack {
                id: "minecraft:string".into(),
                count: 4,
            },
        }]
    }
}
