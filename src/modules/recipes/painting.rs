//! Painting variant recipes.
//!
//! Intended to provide stonecutter recipes that produce each painting variant
//! with the correct data component. Currently implemented as shapeless recipes
//! until Pumpkin exposes stonecutter or data-component recipe APIs.
//!
//! ## Recipe
//!
//! | Input (1×) | Output (1×)          | Variant      |
//! |------------|----------------------|--------------|
//! | `painting` | `painting` (variant) | `kebab`      |
//! | `painting` | `painting` (variant) | `aztec`      |
//! | ...        | ...                  | ...          |
//!
//! > **Note:** The original IllyriaPlus implementation uses Paper's
//! > `DataComponentTypes.PAINTING_VARIANT` via the stonecutter. Pumpkin's
//! > current WIT recipe bindings do not expose data components or stonecutter
//! > recipes, so these are stored as shapeless placeholders. They will need
//! > upstream support to function identically.

use crate::modules::recipes::recipe::{Ingredient, RecipeItemStack, Recipe, ShapelessRecipe};

/// Handles painting variant recipes.
///
/// **Stub:** Waiting for Pumpkin API support for stonecutter recipes
/// and item data components.
#[derive(Default)]
pub struct Painting;

impl Recipe for Painting {
    fn shapeless(&self) -> Vec<ShapelessRecipe> {
        // Vanilla painting variants as of Minecraft 1.21.4
        let variants: Vec<&str> = vec![
            "kebab",
            "aztec",
            "alban",
            "aztec2",
            "bomb",
            "plant",
            "wasteland",
            "pool",
            "courbet",
            "sea",
            "sunset",
            "creebet",
            "wanderer",
            "graham",
            "match",
            "bust",
            "stage",
            "void",
            "skull_and_roses",
            "wither",
            "fighters",
            "pointer",
            "pigscene",
            "burning_skull",
            "skeleton",
            "donkey_kong",
            "earth",
            "wind",
            "fire",
            "water",
            "baroque",
            "humble",
            "meditative",
            "owlemons",
            "passage",
            "pond",
            "unpacked",
        ];

        variants
            .into_iter()
            .map(|variant| ShapelessRecipe {
                id: format!("pumpkinplus:painting_{variant}_stonecutting"),
                ingredients: vec![Ingredient::Item {
                    id: "minecraft:painting".into(),
                }],
                result: RecipeItemStack {
                    id: "minecraft:painting".into(),
                    count: 1,
                },
            })
            .collect()
    }
}
