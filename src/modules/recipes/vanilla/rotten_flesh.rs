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
use crate::modules::recipes::recipe::{Recipe, RecipeEntry};
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::recipe::{CookingRecipeBuilder, RecipeCategory};

/// Handles rotten-flesh-to-leather conversion recipes.
#[derive(Default)]
pub struct RottenFlesh;

impl Recipe for RottenFlesh {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.rotten_flesh)
    }

    fn recipes(&self) -> Vec<RecipeEntry> {
        vec![
            // Furnace
            RecipeEntry::Cooking(
                CookingRecipeBuilder::smelting(
                    namespaced_id!("rotten_flesh_furnace"),
                    "minecraft:rotten_flesh",
                    ItemStack::new("minecraft:leather", 1),
                )
                .cooking_time(200)
                .experience(0.1)
                .category(RecipeCategory::Misc),
            ),
            // Smoker
            RecipeEntry::Cooking(
                CookingRecipeBuilder::smoking(
                    namespaced_id!("rotten_flesh_smoker"),
                    "minecraft:rotten_flesh",
                    ItemStack::new("minecraft:leather", 1),
                )
                .cooking_time(100)
                .experience(0.1)
                .category(RecipeCategory::Misc),
            ),
            // Campfire
            RecipeEntry::Cooking(
                CookingRecipeBuilder::campfire(
                    namespaced_id!("rotten_flesh_campfire"),
                    "minecraft:rotten_flesh",
                    ItemStack::new("minecraft:leather", 1),
                )
                .cooking_time(600)
                .experience(0.05)
                .category(RecipeCategory::Misc),
            ),
        ]
    }
}
