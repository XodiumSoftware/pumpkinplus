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

use crate::modules::recipes::recipe::{
    CookingKind, CookingRecipe, Ingredient, Recipe, RecipeItemStack,
};

/// Handles diamond gear recycling via blast furnace.
#[derive(Default)]
pub struct DiamondRecycle;

impl Recipe for DiamondRecycle {
    fn cooking(&self) -> Vec<CookingRecipe> {
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
            .map(|id| CookingRecipe {
                id: format!(
                    "pumpkinplus:diamond_recycle_{}",
                    id.rsplit_once(':').map_or(id, |(_, s)| s)
                ),
                ingredient: Ingredient::Item { id: id.into() },
                result: RecipeItemStack {
                    id: "minecraft:diamond".into(),
                    count: 1,
                },
                cook_time: 100,
                experience: 1.0,
                kind: CookingKind::Blasting,
            })
            .collect()
    }
}
