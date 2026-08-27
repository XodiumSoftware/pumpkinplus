//! Chainmail armor recipes.
//!
//! Provides shaped crafting recipes for chainmail armor pieces using
//! iron bars as the primary ingredient. Registered via the [`Recipe`] trait.
//!
//! ## Recipes
//!
//! | Output              | Pattern                | Ingredients |
//! |---------------------|------------------------|-------------|
//! | Chainmail Helmet    | `AAA`, `A A`           | `A` = iron bars |
//! | Chainmail Chestplate| `A A`, `AAA`, `AAA`    | `A` = iron bars |
//! | Chainmail Leggings  | `AAA`, `A A`, `A A`    | `A` = iron bars |
//! | Chainmail Boots     | `A A`, `A A`           | `A` = iron bars |

use crate::config::ConfigManager;
use crate::modules::recipes::recipe::{Recipe, RecipeEntry};
use crate::namespaced_id;
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::recipe::{RecipeCategory, ShapedRecipeBuilder};

/// Handles chainmail armor recipe registration.
#[derive(Default)]
pub struct Chainmail;

impl Recipe for Chainmail {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.chainmail)
    }

    fn recipes(&self) -> Vec<RecipeEntry> {
        vec![
            RecipeEntry::Shaped(
                ShapedRecipeBuilder::new(
                    namespaced_id!("chainmail_helmet"),
                    ItemStack::new("minecraft:chainmail_helmet", 1),
                )
                .pattern(["AAA", "A A"])
                .key('A', "minecraft:iron_bars")
                .category(RecipeCategory::Misc),
            ),
            RecipeEntry::Shaped(
                ShapedRecipeBuilder::new(
                    namespaced_id!("chainmail_chestplate"),
                    ItemStack::new("minecraft:chainmail_chestplate", 1),
                )
                .pattern(["A A", "AAA", "AAA"])
                .key('A', "minecraft:iron_bars")
                .category(RecipeCategory::Misc),
            ),
            RecipeEntry::Shaped(
                ShapedRecipeBuilder::new(
                    namespaced_id!("chainmail_leggings"),
                    ItemStack::new("minecraft:chainmail_leggings", 1),
                )
                .pattern(["AAA", "A A", "A A"])
                .key('A', "minecraft:iron_bars")
                .category(RecipeCategory::Misc),
            ),
            RecipeEntry::Shaped(
                ShapedRecipeBuilder::new(
                    namespaced_id!("chainmail_boots"),
                    ItemStack::new("minecraft:chainmail_boots", 1),
                )
                .pattern(["A A", "A A"])
                .key('A', "minecraft:iron_bars")
                .category(RecipeCategory::Misc),
            ),
        ]
    }
}
