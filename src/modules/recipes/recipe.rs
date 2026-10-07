//! Recipe system for `PumpkinPlus`.
//!
//! Each recipe pack is implemented as a module implementing the [`Recipe`] trait.
//! Recipe packs return a list of [`RecipeEntry`] variants wrapping the upstream
//! Pumpkin recipe builders.
//!
//! Recipe packs can be toggled individually in `recipes.toml`; each pack is
//! disabled by default.
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

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::recipe::{
    CookingRecipeBuilder, RecipeError, ShapedRecipeBuilder, ShapelessRecipeBuilder,
};
use serde::{Deserialize, Serialize};
use tracing::error;

/// Recipe counts grouped by kind.
#[derive(Debug, Clone, Copy, Default)]
pub struct RecipeCounts {
    /// Number of shaped crafting recipes.
    pub shaped: u32,
    /// Number of shapeless crafting recipes.
    pub shapeless: u32,
    /// Number of cooking recipes (furnace, smoker, campfire, blast furnace).
    pub cooking: u32,
}

impl RecipeCounts {
    /// Returns the total number of recipes across all kinds.
    #[must_use]
    pub fn total(self) -> u32 {
        self.shaped + self.shapeless + self.cooking
    }
}

impl std::ops::AddAssign for RecipeCounts {
    fn add_assign(&mut self, other: Self) {
        self.shaped += other.shaped;
        self.shapeless += other.shapeless;
        self.cooking += other.cooking;
    }
}

/// A single recipe entry produced by a recipe pack.
///
/// Wraps one of the upstream Pumpkin recipe builders so a pack can return a
/// single heterogeneous list of recipes.
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
    ///
    /// # Errors
    ///
    /// Returns [`RecipeError`] if upstream validation or registration fails.
    fn register(self, context: &Context) -> Result<(), RecipeError> {
        match self {
            Self::Shaped(builder) => context.register_recipe(builder),
            Self::Shapeless(builder) => context.register_recipe(builder),
            Self::Cooking(builder) => context.register_recipe(builder),
        }
    }
}

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
///     fn recipes(&self) -> Vec<RecipeEntry> {
///         vec![RecipeEntry::Shaped(
///             ShapedRecipeBuilder::new("my_plugin:example", ItemStack::new("minecraft:diamond_block", 1))
///                 .pattern(["AAA", "A A", "AAA"])
///                 .key('A', "minecraft:diamond")
///                 .category(RecipeCategory::Misc),
///         )]
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

    /// Returns all recipes in this pack.
    ///
    /// Each [`RecipeEntry`] is registered with the server when [`Recipe::register`]
    /// is called. Defaults to an empty vector.
    fn recipes(&self) -> Vec<RecipeEntry> {
        vec![]
    }

    /// Returns the total number of recipes provided by this module grouped by
    /// kind (shaped, shapeless, cooking).
    fn counts(&self) -> RecipeCounts {
        let mut counts = RecipeCounts::default();
        for entry in self.recipes() {
            match entry {
                RecipeEntry::Shaped(_) => counts.shaped += 1,
                RecipeEntry::Shapeless(_) => counts.shapeless += 1,
                RecipeEntry::Cooking(_) => counts.cooking += 1,
            }
        }
        counts
    }

    /// Returns the total number of recipes provided by this module.
    fn count(&self) -> u32 {
        let counts = self.counts();
        counts.shaped + counts.shapeless + counts.cooking
    }

    /// Returns `true` if there is at least one recipe to register.
    fn has_recipes(&self) -> bool {
        self.count() > 0
    }

    /// Registers all recipes returned by [`Recipe::recipes`] with the server.
    ///
    /// If the recipe pack is disabled via [`Recipe::enabled`], this is a no-op.
    ///
    /// Any upstream registration error is logged here.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }

        if !self.has_recipes() {
            return;
        }

        let recipes = self.recipes();
        for recipe in recipes {
            if let Err(e) = recipe.register(context) {
                error!("Failed to register recipe: {e}");
            }
        }
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
