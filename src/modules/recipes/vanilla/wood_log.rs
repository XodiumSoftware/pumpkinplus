//! Wood block to log conversion recipes.
//!
//! Provides shapeless crafting recipes that convert wood/hyphae blocks
//! back into 4 logs/stems. Covers all vanilla wood types including stripped
//! variants.
//!
//! ## Recipe
//!
//! | Input (1×)            | Output (4×)          |
//! |-----------------------|----------------------|
//! | `oak_wood`            | `oak_log`            |
//! | `spruce_wood`         | `spruce_log`         |
//! | `birch_wood`          | `birch_log`          |
//! | `jungle_wood`         | `jungle_log`         |
//! | `acacia_wood`         | `acacia_log`         |
//! | `dark_oak_wood`       | `dark_oak_log`       |
//! | `mangrove_wood`       | `mangrove_log`       |
//! | `cherry_wood`         | `cherry_log`         |
//! | `pale_oak_wood`       | `pale_oak_log`       |
//! | `crimson_hyphae`      | `crimson_stem`       |
//! | `warped_hyphae`       | `warped_stem`        |
//! | `stripped_oak_wood`   | `stripped_oak_log`   |
//! | `stripped_spruce_wood`| `stripped_spruce_log`|
//! | ... (all stripped)    | ...                  |

use crate::config::ConfigManager;
use crate::modules::recipes::recipe::{Recipe, RecipeEntry};
use pumpkin_plugin_api::ItemStack;
use pumpkin_plugin_api::recipe::{RecipeCategory, ShapelessRecipeBuilder};

/// Handles wood-to-log shapeless conversion recipes.
#[derive(Default)]
pub struct WoodLog;

impl Recipe for WoodLog {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.recipes.wood_log)
    }

    fn recipes(&self) -> Vec<RecipeEntry> {
        let pairs: Vec<(&str, &str)> = vec![
            // Regular
            ("minecraft:oak_wood", "minecraft:oak_log"),
            ("minecraft:spruce_wood", "minecraft:spruce_log"),
            ("minecraft:birch_wood", "minecraft:birch_log"),
            ("minecraft:jungle_wood", "minecraft:jungle_log"),
            ("minecraft:acacia_wood", "minecraft:acacia_log"),
            ("minecraft:dark_oak_wood", "minecraft:dark_oak_log"),
            ("minecraft:mangrove_wood", "minecraft:mangrove_log"),
            ("minecraft:cherry_wood", "minecraft:cherry_log"),
            ("minecraft:pale_oak_wood", "minecraft:pale_oak_log"),
            ("minecraft:crimson_hyphae", "minecraft:crimson_stem"),
            ("minecraft:warped_hyphae", "minecraft:warped_stem"),
            // Stripped
            ("minecraft:stripped_oak_wood", "minecraft:stripped_oak_log"),
            (
                "minecraft:stripped_spruce_wood",
                "minecraft:stripped_spruce_log",
            ),
            (
                "minecraft:stripped_birch_wood",
                "minecraft:stripped_birch_log",
            ),
            (
                "minecraft:stripped_jungle_wood",
                "minecraft:stripped_jungle_log",
            ),
            (
                "minecraft:stripped_acacia_wood",
                "minecraft:stripped_acacia_log",
            ),
            (
                "minecraft:stripped_dark_oak_wood",
                "minecraft:stripped_dark_oak_log",
            ),
            (
                "minecraft:stripped_mangrove_wood",
                "minecraft:stripped_mangrove_log",
            ),
            (
                "minecraft:stripped_cherry_wood",
                "minecraft:stripped_cherry_log",
            ),
            (
                "minecraft:stripped_pale_oak_wood",
                "minecraft:stripped_pale_oak_log",
            ),
            (
                "minecraft:stripped_crimson_hyphae",
                "minecraft:stripped_crimson_stem",
            ),
            (
                "minecraft:stripped_warped_hyphae",
                "minecraft:stripped_warped_stem",
            ),
        ];

        pairs
            .into_iter()
            .map(|(wood, log)| {
                let name = log.rsplit_once(':').map_or(log, |(_, s)| s);
                RecipeEntry::Shapeless(
                    ShapelessRecipeBuilder::new(
                        format!("{}:{name}_from_wood", env!("CARGO_PKG_NAME")),
                        ItemStack::new(log, 4),
                    )
                    .ingredient(wood)
                    .category(RecipeCategory::Misc),
                )
            })
            .collect()
    }
}
