//! Recipe system for `PumpkinPlus`.
//!
//! All recipes are config-driven: admins define shaped, shapeless, and cooking
//! recipes in `recipes.toml` and the plugin registers them on startup. No
//! Rust code is required to add new recipes; see [`GUIDE.md`](../../../GUIDE.md)
//! for the file format reference.
//!
//! ## Supported Recipe Types
//!
//! | Type           | Pumpkin API Status | Description                                   |
//! |----------------|--------------------|-----------------------------------------------|
//! | `shaped`       | ✅ Available       | Crafting recipes with a fixed layout          |
//! | `shapeless`    | ✅ Available       | Crafting recipes with loose items             |
//! | `cooking`      | ✅ Available       | Furnace, smoker, campfire, blast furnace      |
//! | `complex`      | ⛔ Unavailable     | Dynamic special recipes (map cloning, dyeing) |
//! | `merchant`     | ⛔ Unavailable     | Villager/trader trade offers                  |
//! | `potion`       | ⛔ Unavailable     | Potion brewing recipes                        |
//! | `smithing`     | ⛔ Unavailable     | Smithing table transform/trim recipes         |
//! | `stonecutting` | ⛔ Unavailable     | Stonecutter recipes                           |
//!
//! Unavailable types have no host-side registration function in the Pumpkin
//! plugin API yet; the table tracks the parity gaps with Paper.

use pumpkin_plugin_api::recipe::{
    CookingRecipeBuilder, RecipeCategory, ShapedRecipeBuilder, ShapelessRecipeBuilder,
};
use pumpkin_plugin_api::{Context, ItemStack};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::error;

use crate::config::ConfigManager;

/// A single recipe ready to be registered with the server.
///
/// Wraps one of the upstream Pumpkin recipe builders so a heterogeneous list
/// of recipes can be returned from a single source.
pub enum RecipeEntry {
    /// A shaped crafting recipe builder.
    Shaped(ShapedRecipeBuilder),
    /// A shapeless crafting recipe builder.
    Shapeless(ShapelessRecipeBuilder),
    /// A furnace / smoker / blast furnace / campfire recipe builder.
    Cooking(CookingRecipeBuilder),
}

impl RecipeEntry {
    /// Registers this recipe with the server.
    fn register(self, context: &Context) {
        let result = match self {
            Self::Shaped(builder) => builder.register_to_context(context),
            Self::Shapeless(builder) => builder.register_to_context(context),
            Self::Cooking(builder) => builder.register_to_context(context),
        };
        if let Err(e) = result {
            error!("Failed to register recipe: {e}");
        }
    }
}

/// Registers all recipes defined in `recipes.toml` with the server.
///
/// Any malformed entry is logged and skipped; the others still register.
pub fn register_all(context: &Context) {
    let Some(config) = ConfigManager::get() else {
        return;
    };

    let mut shaped_count = 0u32;
    let mut shapeless_count = 0u32;
    let mut cooking_count = 0u32;

    for entry in config.recipes.build_entries() {
        match &entry {
            RecipeEntry::Shaped(_) => shaped_count += 1,
            RecipeEntry::Shapeless(_) => shapeless_count += 1,
            RecipeEntry::Cooking(_) => cooking_count += 1,
        }
        entry.register(context);
    }

    let total = shaped_count + shapeless_count + cooking_count;
    tracing::info!(
        "Registered: {total} recipe(s) ({shaped_count} shaped, {shapeless_count} shapeless, {cooking_count} cooking)"
    );
}

/// A shaped crafting recipe entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShapedRecipe {
    /// Recipe id (auto-prefixed with the plugin id if not already namespaced).
    pub id: String,
    /// Pattern rows, e.g. `["AAA", "A A"]`. Each character must appear in `keys`.
    pub pattern: Vec<String>,
    /// Maps each pattern character to an ingredient. Values may be:
    /// - `"minecraft:diamond"` — a single item
    /// - `"#minecraft:logs"` — a tag
    /// - `["minecraft:oak_log", "minecraft:birch_log"]` — any of these items
    pub keys: HashMap<String, IngredientValue>,
    /// The resulting item.
    pub result: RecipeResult,
    /// Recipe book category (`misc`, `building`, `blocks`, `equipment`, `food`, `redstone`).
    pub category: String,
    /// Optional group name (recipes in the same group collapse in the recipe book).
    pub group: String,
}

impl Default for ShapedRecipe {
    fn default() -> Self {
        Self {
            id: String::new(),
            pattern: Vec::new(),
            keys: HashMap::new(),
            result: RecipeResult::default(),
            category: "misc".to_string(),
            group: String::new(),
        }
    }
}

/// A shapeless crafting recipe entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShapelessRecipe {
    /// Recipe id.
    pub id: String,
    /// List of required ingredients. Each entry follows the same format as
    /// [`ShapedRecipe::keys`] values.
    pub ingredients: Vec<IngredientValue>,
    /// The resulting item.
    pub result: RecipeResult,
    /// Recipe book category.
    pub category: String,
    /// Optional group name.
    pub group: String,
}

impl Default for ShapelessRecipe {
    fn default() -> Self {
        Self {
            id: String::new(),
            ingredients: Vec::new(),
            result: RecipeResult::default(),
            category: "misc".to_string(),
            group: String::new(),
        }
    }
}

/// A cooking (furnace-style) recipe entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CookingRecipe {
    /// Recipe id.
    pub id: String,
    /// Cooking type: `smelting`, `blasting`, `smoking`, or `campfire`.
    #[serde(rename = "type")]
    pub cooking_type: String,
    /// The input ingredient (same format as [`ShapedRecipe::keys`] values).
    pub ingredient: IngredientValue,
    /// The resulting item.
    pub result: RecipeResult,
    /// XP awarded per craft. Defaults to `0.0`.
    pub experience: f32,
    /// Cooking time in ticks. If `0`, uses the type's default (200 smelting /
    /// 100 blasting/smoking / 600 campfire).
    pub cooking_time: u32,
    /// Recipe book category.
    pub category: String,
    /// Optional group name.
    pub group: String,
}

impl Default for CookingRecipe {
    fn default() -> Self {
        Self {
            id: String::new(),
            cooking_type: "smelting".to_string(),
            ingredient: IngredientValue::default(),
            result: RecipeResult::default(),
            experience: 0.0,
            cooking_time: 0,
            category: "misc".to_string(),
            group: String::new(),
        }
    }
}

/// The output of a recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RecipeResult {
    /// Result item id, e.g. `"minecraft:diamond"`.
    pub item: String,
    /// Stack size (1–64). Defaults to `1`.
    pub count: u8,
}

impl Default for RecipeResult {
    fn default() -> Self {
        Self {
            item: String::new(),
            count: 1,
        }
    }
}

/// A config-format ingredient, mapping 1:1 onto the API's `Ingredient` cases.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IngredientValue {
    /// A single item id (`"minecraft:diamond"`) or tag (`"#minecraft:logs"`).
    Single(String),
    /// Any of the listed item ids.
    OneOf(Vec<String>),
}

impl Default for IngredientValue {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

impl From<&IngredientValue> for pumpkin_plugin_api::recipe::Ingredient {
    fn from(value: &IngredientValue) -> Self {
        use pumpkin_plugin_api::recipe::Ingredient;
        match value {
            IngredientValue::Single(s) => {
                if let Some(tag) = s.strip_prefix('#') {
                    Ingredient::tag(tag)
                } else {
                    Ingredient::item(s)
                }
            }
            IngredientValue::OneOf(items) => Ingredient::one_of(items),
        }
    }
}

/// Top-level configuration for all custom recipes.
///
/// Ships as an empty default; the first-run `recipes.toml` is generated from
/// these defaults so admins can edit or remove entries without touching Rust code.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RecipesConfig {
    /// Shaped crafting recipes.
    pub shaped: Vec<ShapedRecipe>,
    /// Shapeless crafting recipes.
    pub shapeless: Vec<ShapelessRecipe>,
    /// Cooking recipes (furnace, blast furnace, smoker, campfire).
    pub cooking: Vec<CookingRecipe>,
}

impl RecipesConfig {
    /// Builds the recipe entries from the loaded config, skipping malformed ones.
    fn build_entries(&self) -> Vec<RecipeEntry> {
        let mut entries = Vec::new();

        for recipe in &self.shaped {
            let mut builder = ShapedRecipeBuilder::new(
                namespaced_id(&recipe.id),
                ItemStack::new(&recipe.result.item, recipe.result.count),
            )
            .pattern(
                recipe
                    .pattern
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
            )
            .category(parse_category(&recipe.category));

            for (sym, ingredient) in &recipe.keys {
                let Some(ch) = sym.chars().next() else {
                    error!("Shaped recipe '{}': key symbol '{sym}' is empty", recipe.id);
                    continue;
                };
                builder = builder.key(ch, pumpkin_plugin_api::recipe::Ingredient::from(ingredient));
            }
            if !recipe.group.is_empty() {
                builder = builder.group(&recipe.group);
            }
            entries.push(RecipeEntry::Shaped(builder));
        }

        for recipe in &self.shapeless {
            let mut builder = ShapelessRecipeBuilder::new(
                namespaced_id(&recipe.id),
                ItemStack::new(&recipe.result.item, recipe.result.count),
            )
            .category(parse_category(&recipe.category));

            for ingredient in &recipe.ingredients {
                builder =
                    builder.ingredient(pumpkin_plugin_api::recipe::Ingredient::from(ingredient));
            }
            if !recipe.group.is_empty() {
                builder = builder.group(&recipe.group);
            }
            entries.push(RecipeEntry::Shapeless(builder));
        }

        for recipe in &self.cooking {
            let ingredient = pumpkin_plugin_api::recipe::Ingredient::from(&recipe.ingredient);
            let output = ItemStack::new(&recipe.result.item, recipe.result.count);
            let id = namespaced_id(&recipe.id);

            let mut builder = match recipe.cooking_type.as_str() {
                "smelting" => CookingRecipeBuilder::smelting(id, ingredient, output),
                "blasting" => CookingRecipeBuilder::blasting(id, ingredient, output),
                "smoking" => CookingRecipeBuilder::smoking(id, ingredient, output),
                "campfire" => CookingRecipeBuilder::campfire(id, ingredient, output),
                other => {
                    error!("Cooking recipe '{}' has unknown type '{other}'", recipe.id);
                    continue;
                }
            };

            if recipe.cooking_time > 0 {
                builder = builder.cooking_time(recipe.cooking_time);
            }
            if recipe.experience > 0.0 {
                builder = builder.experience(recipe.experience);
            }
            builder = builder.category(parse_category(&recipe.category));
            if !recipe.group.is_empty() {
                builder = builder.group(&recipe.group);
            }
            entries.push(RecipeEntry::Cooking(builder));
        }

        entries
    }
}

/// Prepends the plugin id unless the input is already namespaced.
fn namespaced_id(id: &str) -> String {
    if id.contains(':') {
        id.to_string()
    } else {
        format!("{}:{id}", env!("CARGO_PKG_NAME"))
    }
}

/// Maps a category string to a `RecipeCategory` enum variant.
fn parse_category(s: &str) -> RecipeCategory {
    match s.to_ascii_lowercase().as_str() {
        "blocks" => RecipeCategory::Blocks,
        "building" => RecipeCategory::Building,
        "equipment" => RecipeCategory::Equipment,
        "food" => RecipeCategory::Food,
        "redstone" => RecipeCategory::Redstone,
        _ => RecipeCategory::Misc,
    }
}
