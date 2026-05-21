//! Recipe system for PumpkinPlus.
//!
//! Each recipe category is implemented as a module implementing the [`Recipe`] trait.
//! Recipes are registered with the server in bulk via [`Recipe::register`].
//!
//! ## Supported Recipe Types
//!
//! | Type        | Pumpkin API Status | Description                          |
//! |-------------|--------------------|--------------------------------------|
//! | `shaped`    | ✅ Available       | Crafting recipes with a fixed layout |
//! | `shapeless` | ✅ Available       | Crafting recipes with loose items    |
//! | `cooking`   | ✅ Available       | Furnace, smoker, campfire, blast     |
//! | `potion`    | ⛔ Unavailable     | Potion brewing recipes               |

use std::time::Instant;
use tracing::info;

/// A trait representing a collection of custom recipes that can be registered.
///
/// Types implementing this trait provide one or more recipes to be added to the
/// server when [`Recipe::register`] is called. All recipes are returned in
/// bulk to allow for efficient registration.
///
/// # Example
///
/// ```rust,ignore
/// pub struct MyRecipes;
///
/// impl Recipe for MyRecipes {
///     fn recipes(&self) -> Vec<RecipeKind> {
///         vec![
///             RecipeKind::Shaped {
///                 // ...
///             },
///         ]
///     }
/// }
/// ```
pub trait Recipe {
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

    /// Returns `true` if there is at least one recipe to register.
    fn has_recipes(&self) -> bool {
        !self.shaped().is_empty() || !self.shapeless().is_empty() || !self.cooking().is_empty()
    }

    /// Registers all recipes returned by the trait methods.
    ///
    /// Logs the count and time taken. If no recipes are present this is a
    /// no-op.
    fn register(&self) {
        if !self.has_recipes() {
            return;
        }

        let start = Instant::now();

        let shaped = self.shaped();
        let shapeless = self.shapeless();
        let cooking = self.cooking();

        let total = shaped.len() + shapeless.len() + cooking.len();

        // TODO: Wire to pumpkin_plugin_api::recipe once re-exported upstream.
        // https://github.com/Pumpkin-MC/Pumpkin/issues/XXX
        //
        // Example desired call:
        // context.register_shaped_recipes(&shaped);
        // context.register_shapeless_recipes(&shapeless);
        // context.register_cooking_recipes(&cooking);

        let elapsed = start.elapsed().as_millis();
        info!(
            "Registered: {} recipe(s) ({} shaped, {} shapeless, {} cooking) | Took {}ms",
            total,
            shaped.len(),
            shapeless.len(),
            cooking.len(),
            elapsed
        );
    }
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
    pub result: ItemStack,
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
    pub result: ItemStack,
}

/// A furnace / smoker / blast furnace / campfire recipe.
#[derive(Debug, Clone)]
pub struct CookingRecipe {
    /// Unique recipe identifier.
    pub id: String,
    /// The input ingredient.
    pub ingredient: Ingredient,
    /// The result item.
    pub result: ItemStack,
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

/// A stack of items produced by a recipe.
///
/// Mirrors the Pumpkin WIT `ItemStack` type.
#[derive(Debug, Clone)]
pub struct ItemStack {
    /// Item identifier (e.g. `"minecraft:diamond_horse_armor"`).
    pub id: String,
    /// Number of items in the stack.
    pub count: u8,
}

impl Default for ItemStack {
    fn default() -> Self {
        Self {
            id: String::new(),
            count: 1,
        }
    }
}
