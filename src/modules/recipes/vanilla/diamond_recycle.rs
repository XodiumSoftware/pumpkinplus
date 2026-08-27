//! Diamond armor/tool recycling recipes.
//!
//! Provides blast-furnace recipes that recycle damaged or unwanted diamond
//! gear back into raw diamonds.
//!
//! ## Recipe
//!
//! | Input | Output | XP | Time (ticks) |
//! |-------|--------|----|--------------|
//! | Any diamond tool or armor (see list) | 1× Diamond | 1.0 | 100 |
//!
//! ## Supported Inputs
//!
//! `diamond_axe`, `diamond_boots`, `diamond_chestplate`, `diamond_helmet`,
//! `diamond_hoe`, `diamond_horse_armor`, `diamond_leggings`,
//! `diamond_nautilus_armor`, `diamond_pickaxe`, `diamond_shovel`,
//! `diamond_spear`, `diamond_sword`

use crate::config::ConfigManager;
use crate::modules::recipes::recipe::{Recipe, RecipeEntry};
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::recipe::{CookingRecipeBuilder, RecipeCategory};

/// Handles diamond gear recycling via blast furnace.
#[derive(Default)]
pub struct DiamondRecycle;

impl Recipe for DiamondRecycle {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.diamond_recycle)
    }

    fn recipes(&self) -> Vec<RecipeEntry> {
        let inputs: Vec<&str> = vec![
            "minecraft:diamond_axe",
            "minecraft:diamond_boots",
            "minecraft:diamond_chestplate",
            "minecraft:diamond_helmet",
            "minecraft:diamond_hoe",
            "minecraft:diamond_horse_armor",
            "minecraft:diamond_leggings",
            "minecraft:diamond_nautilus_armor",
            "minecraft:diamond_pickaxe",
            "minecraft:diamond_shovel",
            "minecraft:diamond_spear",
            "minecraft:diamond_sword",
        ];

        inputs
            .into_iter()
            .map(|id| {
                let suffix = id.rsplit_once(':').map_or(id, |(_, s)| s);
                RecipeEntry::Cooking(
                    CookingRecipeBuilder::blasting(
                        format!("{}:diamond_recycle_{suffix}", env!("CARGO_PKG_NAME")),
                        id,
                        ItemStack::new("minecraft:diamond", 1),
                    )
                    .cooking_time(100)
                    .experience(1.0)
                    .category(RecipeCategory::Misc),
                )
            })
            .collect()
    }
}
