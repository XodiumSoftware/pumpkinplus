//! Rotten-flesh-to-leather recipes.
//!
//! Converts rotten flesh into leather via furnace, smoker, and campfire.
//!
//! ## Recipes
//!
//! | Station  | Output   | XP   | Time (ticks) |
//! |----------|----------|------|--------------|
//! | Furnace  | 1× Leather | 0.1  | 200          |
//! | Smoker   | 1× Leather | 0.1  | 100          |
//! | Campfire | 1× Leather | 0.05 | 600          |

use crate::config::ConfigManager;
use crate::modules::recipes::recipe::{
    CookingKind, CookingRecipe, Ingredient, Recipe, RecipeItemStack,
};
use crate::namespaced_id;

/// Handles rotten-flesh-to-leather conversion recipes.
#[derive(Default)]
pub struct RottenFlesh;

impl Recipe for RottenFlesh {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.rotten_flesh)
    }

    fn cooking(&self) -> Vec<CookingRecipe> {
        vec![
            // Furnace
            CookingRecipe {
                id: namespaced_id!("rotten_flesh_furnace").into(),
                ingredient: Ingredient::Item {
                    id: "minecraft:rotten_flesh".into(),
                },
                result: RecipeItemStack {
                    id: "minecraft:leather".into(),
                    count: 1,
                },
                cook_time: 200,
                experience: 0.1,
                kind: CookingKind::Smelting,
            },
            // Smoker
            CookingRecipe {
                id: namespaced_id!("rotten_flesh_smoker").into(),
                ingredient: Ingredient::Item {
                    id: "minecraft:rotten_flesh".into(),
                },
                result: RecipeItemStack {
                    id: "minecraft:leather".into(),
                    count: 1,
                },
                cook_time: 100,
                experience: 0.1,
                kind: CookingKind::Smoking,
            },
            // Campfire
            CookingRecipe {
                id: namespaced_id!("rotten_flesh_campfire").into(),
                ingredient: Ingredient::Item {
                    id: "minecraft:rotten_flesh".into(),
                },
                result: RecipeItemStack {
                    id: "minecraft:leather".into(),
                    count: 1,
                },
                cook_time: 600,
                experience: 0.05,
                kind: CookingKind::Campfire,
            },
        ]
    }
}
