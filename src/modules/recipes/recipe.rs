//! Recipe system for `PumpkinPlus`.
//!
//! Each recipe category is implemented as a module implementing the [`Recipe`] trait.
//! Recipes are registered with the server via [`Recipe::register`] using the upstream
//! Pumpkin recipe builders.
//!
//! Recipe packs can be toggled individually via the `recipes` section of `config.json`;
//! each pack is disabled by default.
//!
//! ## Supported Recipe Types
//!
//! | Type        | Pumpkin API Status | Description                          |
//! |-------------|--------------------|--------------------------------------|
//! | `shaped`    | ✅ Available       | Crafting recipes with a fixed layout |
//! | `shapeless` | ✅ Available       | Crafting recipes with loose items    |
//! | `cooking`   | ✅ Available       | Furnace, smoker, campfire, blast     |
//! | `potion`    | ⛔ Unavailable     | Potion brewing recipes               |

use pumpkin_plugin_api::{
    Context,
    recipe::{
        CookingRecipeBuilder, RecipeCategory, RecipeError, ShapedRecipeBuilder,
        ShapelessRecipeBuilder,
    },
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::info;

/// A trait representing a collection of custom recipes that can be registered.
///
/// Types implementing this trait provide one or more recipes to be added to the
/// server when [`Recipe::register`] is called.
///
/// # Example
///
/// ```rust,ignore
/// pub struct MyRecipes;
///
/// impl Recipe for MyRecipes {
///     fn enabled(&self) -> bool {
///         ConfigManager::get().is_some_and(|cm| cm.recipes.my_recipes.enabled)
///     }
///
///     fn shaped(&self) -> Vec<ShapedRecipe> {
///         vec![ShapedRecipe {
///             id: "pumpkinplus:example".into(),
///             height: 2,
///             width: 3,
///             pattern: vec!["AAA".into(), "A A".into()],
///             keys: vec![('A', Ingredient::Item { id: "minecraft:diamond".into() })],
///             result: RecipeItemStack { id: "minecraft:diamond_block".into(), count: 1 },
///         }]
///     }
/// }
/// ```
pub trait Recipe {
    /// Returns `true` if this recipe pack is enabled in the configuration.
    ///
    /// Defaults to `true` so that recipe packs without a dedicated toggle
    /// remain registered. Packs that have a config field should override this
    /// to check `ConfigManager`.
    fn enabled(&self) -> bool {
        true
    }
    /// Returns the shaped crafting recipes to be registered.
    ///
    /// Each entry describes a recipe with a fixed grid pattern. Override this
    /// to provide shaped recipes. Defaults to an empty vector.
    fn shaped(&self) -> Vec<ShapedRecipe> {
        vec![]
    }

    /// Returns the shapeless crafting recipes to be registered.
    ///
    /// Each entry describes a recipe where ingredients can be placed in any
    /// slot of the crafting grid. Override this to provide shapeless recipes.
    /// Defaults to an empty vector.
    fn shapeless(&self) -> Vec<ShapelessRecipe> {
        vec![]
    }

    /// Returns the cooking recipes to be registered.
    ///
    /// Covers furnace, smoker, blast furnace, and campfire recipes. Override
    /// this to provide cooking recipes. Defaults to an empty vector.
    fn cooking(&self) -> Vec<CookingRecipe> {
        vec![]
    }

    /// Returns the total number of recipes provided by this module.
    fn count(&self) -> u32 {
        #[allow(clippy::cast_possible_truncation)]
        {
            self.shaped().len() as u32 + self.shapeless().len() as u32 + self.cooking().len() as u32
        }
    }

    /// Returns `true` if there is at least one recipe to register.
    fn has_recipes(&self) -> bool {
        self.count() > 0
    }

    /// Registers all recipes returned by the trait methods with the server.
    ///
    /// If the recipe pack is disabled via [`Recipe::enabled`], this is a no-op.
    /// Otherwise, logs the count and time taken. If no recipes are present this is a
    /// no-op.
    ///
    /// # Errors
    ///
    /// Returns [`RecipeError`] if any upstream recipe registration fails validation.
    fn register(&self, context: &Context) -> Result<(), RecipeError> {
        if !self.enabled() {
            return Ok(());
        }

        if !self.has_recipes() {
            return Ok(());
        }

        let start = Instant::now();

        let shaped = self.shaped();
        let shapeless = self.shapeless();
        let cooking = self.cooking();

        let total = shaped.len() + shapeless.len() + cooking.len();

        for recipe in &shaped {
            ShapedRecipeBuilder::new(&recipe.id, recipe.result.as_stack())
                .pattern(recipe.pattern.clone())
                .keys(recipe.keys.clone())
                .category(RecipeCategory::Misc)
                .register_to_context(context)?;
        }

        for recipe in &shapeless {
            let mut builder = ShapelessRecipeBuilder::new(&recipe.id, recipe.result.as_stack())
                .category(RecipeCategory::Misc);
            for ingredient in &recipe.ingredients {
                builder = builder.ingredient(ingredient.as_ingredient());
            }
            builder.register_to_context(context)?;
        }

        for recipe in &cooking {
            let builder = match recipe.kind {
                CookingKind::Smelting => CookingRecipeBuilder::smelting(
                    &recipe.id,
                    recipe.ingredient.as_ingredient(),
                    recipe.result.as_stack(),
                ),
                CookingKind::Blasting => CookingRecipeBuilder::blasting(
                    &recipe.id,
                    recipe.ingredient.as_ingredient(),
                    recipe.result.as_stack(),
                ),
                CookingKind::Smoking => CookingRecipeBuilder::smoking(
                    &recipe.id,
                    recipe.ingredient.as_ingredient(),
                    recipe.result.as_stack(),
                ),
                CookingKind::Campfire => CookingRecipeBuilder::campfire(
                    &recipe.id,
                    recipe.ingredient.as_ingredient(),
                    recipe.result.as_stack(),
                ),
            };
            builder
                .cooking_time(recipe.cook_time)
                .experience(recipe.experience)
                .category(RecipeCategory::Misc)
                .register_to_context(context)?;
        }

        let elapsed = start.elapsed().as_millis();
        info!(
            "Registered: {} recipe(s) ({} shaped, {} shapeless, {} cooking) | Took {}ms",
            total,
            shaped.len(),
            shapeless.len(),
            cooking.len(),
            elapsed
        );

        Ok(())
    }
}

/// Top-level configuration for all recipe packs.
///
/// Each boolean toggles one recipe pack. All packs are disabled by default
/// to match the behavior of other gameplay modules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct RecipesConfig {
    /// Craft chainmail armor pieces using iron bars.
    pub chainmail: bool,
    /// Recycle diamond tools and armor back into diamonds.
    pub diamond_recycle: bool,
    /// Break packed and blue ice down into lower-tier ice.
    pub ice_breakdown: bool,
    /// Break nether wart blocks back into nether warts.
    pub nether_wart_block: bool,
    /// Placeholder recipes for painting variants.
    pub painting: bool,
    /// Cook rotten flesh into leather.
    pub rotten_flesh: bool,
    /// Convert wood/hyphae blocks back into logs/stems.
    pub wood_log: bool,
    /// Convert wool blocks into string.
    pub wool_to_string: bool,
}

/// A shaped crafting recipe.
///
/// Patterns use single-character keys mapped to [`Ingredient`] entries.
/// Spaces represent empty slots.
#[derive(Debug, Clone)]
pub struct ShapedRecipe {
    /// Unique recipe identifier (e.g. `"pumpkinplus:diamond_horse_armor"`).
    pub id: String,
    /// Grid height (1–3).
    pub height: u8,
    /// Grid width (1–3).
    pub width: u8,
    /// Pattern rows. Each string must be exactly `width` characters.
    /// Use a space `' '` for an empty slot.
    pub pattern: Vec<String>,
    /// Mapping from pattern characters to ingredients.
    pub keys: Vec<(char, Ingredient)>,
    /// The result item.
    pub result: RecipeItemStack,
}

/// A shapeless crafting recipe.
///
/// Ingredients can be placed in any slot of the crafting grid.
#[derive(Debug, Clone)]
pub struct ShapelessRecipe {
    /// Unique recipe identifier.
    pub id: String,
    /// List of ingredients (may include duplicates for multi-count slots).
    pub ingredients: Vec<Ingredient>,
    /// The result item.
    pub result: RecipeItemStack,
}

/// A furnace / smoker / blast furnace / campfire recipe.
#[derive(Debug, Clone)]
pub struct CookingRecipe {
    /// Unique recipe identifier.
    pub id: String,
    /// The input ingredient.
    pub ingredient: Ingredient,
    /// The result item.
    pub result: RecipeItemStack,
    /// Base cooking time in ticks (e.g. 200 for furnace).
    pub cook_time: u32,
    /// Experience granted when the item is removed.
    pub experience: f32,
    /// Which cooking block this applies to.
    pub kind: CookingKind,
}

/// Variant of cooking recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookingKind {
    /// Standard furnace.
    Smelting,
    /// Blast furnace (2× speed).
    Blasting,
    /// Smoker (2× speed).
    Smoking,
    /// Campfire (3× duration).
    Campfire,
}

/// An ingredient accepted by a recipe.
///
/// Mirrors the Pumpkin WIT `RecipeIngredient` type.
#[derive(Debug, Clone)]
pub enum Ingredient {
    /// Accept an exact item by its identifier (e.g. `"minecraft:diamond"`).
    Item { id: String },
    /// Accept any item in a tag (e.g. `"minecraft:logs"`).
    Tag { id: String },
}

impl Ingredient {
    /// Converts this recipe ingredient into the upstream `pumpkin_plugin_api::recipe::Ingredient`.
    fn as_ingredient(&self) -> pumpkin_plugin_api::recipe::Ingredient {
        match self {
            Self::Item { id } => pumpkin_plugin_api::recipe::Ingredient::item(id.clone()),
            Self::Tag { id } => pumpkin_plugin_api::recipe::Ingredient::tag(id.clone()),
        }
    }
}

/// A stack of items produced by a recipe.
///
/// Mirrors the Pumpkin WIT `ItemStack` type.
#[derive(Debug, Clone)]
pub struct RecipeItemStack {
    /// Item identifier (e.g. `"minecraft:diamond_horse_armor"`).
    pub id: String,
    /// Number of items in the stack.
    pub count: u8,
}

impl RecipeItemStack {
    /// Converts this recipe result into an upstream `pumpkin_plugin_api::ItemStack`.
    fn as_stack(&self) -> pumpkin_plugin_api::ItemStack {
        pumpkin_plugin_api::ItemStack::new(&self.id, self.count)
    }
}

impl Default for RecipeItemStack {
    fn default() -> Self {
        Self {
            id: String::new(),
            count: 1,
        }
    }
}

/// Adapts internal `ShapedRecipe` keys to upstream builder calls.
trait ShapedKeys {
    /// Applies all ingredient keys to a shaped recipe builder.
    fn keys(self, keys: Vec<(char, Ingredient)>) -> Self;
}

impl ShapedKeys for ShapedRecipeBuilder {
    fn keys(mut self, keys: Vec<(char, Ingredient)>) -> Self {
        for (symbol, ingredient) in keys {
            self = self.key(symbol, ingredient.as_ingredient());
        }
        self
    }
}
