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
use crate::modules::recipes::recipe::{Ingredient, Recipe, RecipeItemStack, ShapelessRecipe};
use crate::namespaced_id;

/// Handles ice breakdown shapeless recipes.
#[derive(Default)]
pub struct IceBreakdown;

impl Recipe for IceBreakdown {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.ice_breakdown)
    }

    fn shapeless(&self) -> Vec<ShapelessRecipe> {
        vec![
            ShapelessRecipe {
                id: namespaced_id!("blue_ice_breakdown").into(),
                ingredients: vec![Ingredient::Item {
                    id: "minecraft:blue_ice".into(),
                }],
                result: RecipeItemStack {
                    id: "minecraft:packed_ice".into(),
                    count: 9,
                },
            },
            ShapelessRecipe {
                id: namespaced_id!("packed_ice_breakdown").into(),
                ingredients: vec![Ingredient::Item {
                    id: "minecraft:packed_ice".into(),
                }],
                result: RecipeItemStack {
                    id: "minecraft:ice".into(),
                    count: 9,
                },
            },
        ]
    }
}
