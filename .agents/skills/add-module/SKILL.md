---
name: add-module
description: Scaffolds a new PumpkinPlus module (mechanic, enchantment, item, or recipe pack) with config wiring and lib.rs registration.
---

# Add a Module

Use this skill when the user wants to add a new feature to PumpkinPlus. PumpkinPlus has four module kinds:

| Kind | Trait | Location | Config style | Registered with server? |
|---|---|---|---|---|
| `mechanic` | `Mechanic` | `src/modules/mechanics/{entity,player,server,world}/` | Own `{Name}Config` struct | Yes (events, commands, permissions) |
| `enchantment` | `Enchantment` | `src/modules/enchantments/{utility,vanilla}/` | `bool` field in `EnchantmentsConfig` | Yes (custom definition and/or events) |
| `item` | `Item` (`items::item::Item`) | `src/modules/items/{category}/` (currently only `weapons/`) | None | No — pure builder used by other modules |
| `recipe` | `Recipe` | `src/modules/recipes/vanilla/` | `bool` field in `RecipesConfig` | Yes (recipe entries) |

## Before Writing Code

1. Ask the user which kind: **mechanic**, **enchantment**, **item**, or **recipe**.
2. Ask for:
   - The module name (e.g. `Greatsword`, `silk_touch`, `wood_log`).
   - For mechanics: which category folder (`entity`, `player`, `server`, `world`) and whether it needs commands, permissions, or events.
   - For enchantments: `utility` (custom registry enchantment) or `vanilla` (behavior override of a vanilla enchantment), max level, anvil cost, weight, slots, supported items tag, and which events to listen to.
   - For items: the item category subfolder, the vanilla base item, custom name, and any data components/NBT markers.
   - For recipes: shaped/shapeless/cooking recipes, patterns, ingredient keys, and output items.
3. If unsure about values, mirror a similar existing module:
   - Mechanic: `src/modules/mechanics/server/tablist.rs`
   - Utility enchantment: `src/modules/enchantments/utility/embertread.rs`
   - Vanilla enchantment: `src/modules/enchantments/vanilla/fortune.rs`
   - Item: `src/modules/items/weapons/longsword.rs`
   - Recipe: `src/modules/recipes/vanilla/chainmail.rs`

## Naming

- File/module name: `snake_case` (e.g. `nether_wart_block.rs`).
- Struct name: `PascalCase` (e.g. `NetherWartBlock`).
- Plugin-namespaced ids: use the crate macro `use crate::namespaced_id;` and `namespaced_id!("suffix")` — never hardcode `pumpkinplus:`.
- Snake-case config field name matches the file name.

## Common Skeleton

Every module file follows this item order (per `AGENTS.md` code style):

1. `//!` module docs with a config table
2. `use` imports
3. `const` values (e.g. `EMBERTREAD_ID`)
4. `#[derive(Default)] pub struct {Name};`
5. `impl {Trait} for {Name}` — `enabled()`, then `events()`/`enchantment()`/`recipes()`/`build()`
6. `impl EventHandler<T> for {Name}` blocks (mechanics/enchantments only)
7. Helper fns (private, at bottom, before any local config struct)
8. `{Name}Config` struct at bottom (mechanics only)

Event handlers use `impl EventHandler<T> for {Name}` + `fn handle(&self, server: Server, event: EventData<T>) -> EventData<T>` and are registered in `events()` via `self.register_event::<T>(context, EventPriority::Normal, true);`.

Never log with `println!` — use `tracing::{info, debug, error}`.

## Mechanic

1. Create `src/modules/mechanics/{category}/{name}.rs`.
2. `enabled()` must be:
   ```rust
   fn enabled(&self) -> bool {
       ConfigManager::get().is_some_and(|cm| cm.mechanics.{snake_name}.enabled)
   }
   ```
3. Override `cmds()`/`perms()` only if there are commands. Permission node pattern: `{PLUGIN_ID}:command.{name}` via `PLUGIN_ID` or `namespaced_id!`. `perms()` returns `Vec<Permission>` paired **by index** with `cmds()`.
4. Define at the bottom of the same file:
   ```rust
   /// Configuration for the {name} mechanics module.
   #[derive(Debug, Default, Clone, Serialize, Deserialize)]
   pub struct {Name}Config {
       /// Whether this module is active.
       pub enabled: bool,
       // module fields...
   }
   ```
5. Wire the config into `src/modules/mechanics/mechanic.rs`:
   - Add `pub use crate::modules::mechanics::{category}::{name}::{Name}Config;` with the other `pub use` lines.
   - Add `pub {snake_name}: {Name}Config,` with a `///` doc comment to `MechanicsConfig` (keep alphabetical with existing fields).
6. Wire into `src/lib.rs`:
   - Add `pub mod {name};` inside `mod modules { mod mechanics { mod {category} { ... } } }`, alphabetical.
   - Add `pub use modules::mechanics::{category}::{name}::{Name}Config;` with the other re-exports.
   - Add `use crate::mechanics::{category}::{name}::{Name};` with the other imports.
   - Add `&{Name},` to the `mechanics` vec inside `register_mechanics`, keeping existing ordering style.

## Enchantment

1. Ask whether it is `utility` (registers a brand-new custom enchantment via `EnchantmentBuilder`, e.g. Embertread) or `vanilla` (event-driven behavior override gated on a vanilla enchantment, e.g. Fortune).
2. Create `src/modules/enchantments/{utility|vanilla}/{name}.rs`.
3. `enabled()` must be:
   ```rust
   fn enabled(&self) -> bool {
       ConfigManager::get().is_some_and(|cm| cm.enchantments.{snake_name})
   }
   ```
   Note: `EnchantmentsConfig` fields are plain `bool`, not structs.
4. For a **utility** enchantment, also override `enchantment()`:
   ```rust
   const {SCREAMING}_ID: &str = namespaced_id!("{snake_name}");

   fn enchantment(&self) -> EnchantmentBuilder {
       EnchantmentBuilder::new({SCREAMING}_ID, TextComponent::text("{Display Name}"))
           .max_level(1)
           .anvil_cost(2)
           .supported_items("#minecraft:enchantable/foot_armor")
           .weight(2)
           .slots([AttributeModifierSlot::Feet])
   }
   ```
   Event code checks presence with `stack.has_custom_enchantment({SCREAMING}_ID)`.
5. For a **vanilla** override, leave `enchantment()` at the default and gate behavior on the vanilla enchantment, e.g. `item.get_enchantments().iter().find(|e| e.enchantment == VanillaEnchantment::Fortune)`.
6. Override `events()` to register each `EventHandler` impl.
7. Wire the toggle into `src/modules/enchantments/enchantment.rs`: add `/// doc` + `pub {snake_name}: bool,` to `EnchantmentsConfig` (alphabetical).
8. Wire into `src/lib.rs`:
   - Add `pub mod {name};` inside `mod modules { mod enchantments { mod {utility|vanilla} } }`, alphabetical.
   - Add `use crate::modules::enchantments::{utility|vanilla}::{name}::{Name};`.
   - Add `&{Name},` to the `enchantments` vec inside `register_enchantments`.

## Item

Items are **pure builders** — Pumpkin has no custom-item registry, so there is no config toggle and no registration. Other modules call `build()` to obtain a stack.

1. Create `src/modules/items/{category}/{name}.rs` (only `weapons/` exists today; create a new category folder if needed and add it to `lib.rs` under `mod items`).
2. Implement the trait (note the alias to avoid clashing with the API's `Item` enum):
   ```rust
   use crate::items::item::Item as ItemTrait;

   impl ItemTrait for {Name} {
       fn key(&self) -> &'static str {
           namespaced_id!("{snake_name}")
       }

       fn build(&self) -> ItemStack {
           let (namespace, key) = self.key_parts();
           let stack = ItemStack::of(Item::{BaseItem}, 1);
           stack.set_custom_name(Some(TextComponent::text("{Display Name}")));
           stack.set_custom_data(namespace, key, &NbtTree {
               root: 0,
               tags: vec![NbtTag::Byte(1)],
           });
           stack
       }
   }
   ```
3. Document components in `//!` docs as a table, mirroring `longsword.rs`. If setting a component with an unimplemented host codec (e.g. `attribute_modifiers`), say so in the docs and comment, and attach an empty payload.
4. Wire into `src/lib.rs`: add `pub mod {name};` inside `mod modules { mod items { mod {category} } }`, alphabetical.
5. Update the "Available Items" table in `src/modules/items/item.rs` doc header.

## Recipe Pack

1. Create `src/modules/recipes/vanilla/{name}.rs`.
2. `enabled()` must be:
   ```rust
   fn enabled(&self) -> bool {
       ConfigManager::get().is_some_and(|cm| cm.recipes.{snake_name})
   }
   ```
   Note: `RecipesConfig` fields are plain `bool`.
3. Build recipes in `recipes()` with `RecipeEntry` variants, e.g.:
   ```rust
   vec![
       RecipeEntry::Shaped(
           ShapedRecipeBuilder::new(
               namespaced_id!("{snake_name}_{item}"),
               ItemStack::new("minecraft:{item}", 1),
           )
           .pattern(["AAA", "A A"])
           .key('A', "minecraft:iron_bars")
           .category(RecipeCategory::Misc),
       ),
       RecipeEntry::Shapeless(
           ShapelessRecipeBuilder::new(
               namespaced_id!("{snake_name}_unshape"),
               ItemStack::new("minecraft:iron_bars", 8),
           )
           .ingredient("minecraft:chainmail_chestplate")
           .category(RecipeCategory::Misc),
       ),
       RecipeEntry::Cooking(
           CookingRecipeBuilder::smelting(
               namespaced_id!("{snake_name}_furnace"),
               "minecraft:input_item",
               ItemStack::new("minecraft:output_item", 1),
           ),
       ),
   ]
   ```
   Potion brewing recipes are **not** supported by the Pumpkin API (`recipe.rs` docs say so) — don't scaffold them.
4. Document recipes in `//!` docs as a table (output / pattern / ingredients), mirroring `chainmail.rs`.
5. Wire the toggle into `src/modules/recipes/recipe.rs`: add `/// doc` + `pub {snake_name}: bool,` to `RecipesConfig` (alphabetical).
6. Wire into `src/lib.rs`:
   - Add `pub mod {name};` inside `mod modules { mod recipes { mod vanilla } }`, alphabetical.
   - Add `use crate::modules::recipes::vanilla::{name}::{Name};`.
   - Add `&{Name},` to the `recipes` vec inside `register_recipes`.

## Verification

1. Build: `cargo build --target wasm32-wasip2`
2. Lint (must pass with zero warnings — warnings are errors here):
   `cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings`
3. All new modules default to `enabled: false`; the config file is auto-regenerated with defaults on next load, so no hand-editing of `config.json` is needed.
4. If the Pumpkin plugin API version matters (new events/builders), run `cargo update` first — the API is git-tracked.

After finishing, summarize the files changed and ask the user if they want to commit, per the project workflow in `AGENTS.md`.
