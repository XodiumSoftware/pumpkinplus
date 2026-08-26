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

use crate::modules::recipes::recipe::{Ingredient, RecipeItemStack, Recipe, ShapedRecipe};

/// Handles chainmail armor recipe registration.
#[derive(Default)]
pub struct Chainmail;

impl Recipe for Chainmail {
    fn shaped(&self) -> Vec<ShapedRecipe> {
        vec![
            // Chainmail Helmet
            ShapedRecipe {
                id: "pumpkinplus:chainmail_helmet".into(),
                height: 2,
                width: 3,
                pattern: vec!["AAA".into(), "A A".into()],
                keys: vec![(
                    'A',
                    Ingredient::Item {
                        id: "minecraft:iron_bars".into(),
                    },
                )],
                result: RecipeItemStack {
                    id: "minecraft:chainmail_helmet".into(),
                    count: 1,
                },
            },
            // Chainmail Chestplate
            ShapedRecipe {
                id: "pumpkinplus:chainmail_chestplate".into(),
                height: 3,
                width: 3,
                pattern: vec!["A A".into(), "AAA".into(), "AAA".into()],
                keys: vec![(
                    'A',
                    Ingredient::Item {
                        id: "minecraft:iron_bars".into(),
                    },
                )],
                result: RecipeItemStack {
                    id: "minecraft:chainmail_chestplate".into(),
                    count: 1,
                },
            },
            // Chainmail Leggings
            ShapedRecipe {
                id: "pumpkinplus:chainmail_leggings".into(),
                height: 3,
                width: 3,
                pattern: vec!["AAA".into(), "A A".into(), "A A".into()],
                keys: vec![(
                    'A',
                    Ingredient::Item {
                        id: "minecraft:iron_bars".into(),
                    },
                )],
                result: RecipeItemStack {
                    id: "minecraft:chainmail_leggings".into(),
                    count: 1,
                },
            },
            // Chainmail Boots
            ShapedRecipe {
                id: "pumpkinplus:chainmail_boots".into(),
                height: 2,
                width: 3,
                pattern: vec!["A A".into(), "A A".into()],
                keys: vec![(
                    'A',
                    Ingredient::Item {
                        id: "minecraft:iron_bars".into(),
                    },
                )],
                result: RecipeItemStack {
                    id: "minecraft:chainmail_boots".into(),
                    count: 1,
                },
            },
        ]
    }
}
