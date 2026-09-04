---
name: bootstrap-audit
description: Audits PumpkinPlus registration wiring so every module file is declared in lib.rs, has a config toggle, and is actually registered.
---

# Bootstrap Audit

Use this skill when the user asks to verify the plugin wiring, when modules appear to be missing at runtime, or after adding/renaming several modules. This project historically has files that exist but are not registered (e.g. utility enchantments declared in `lib.rs` modules but absent from `register_enchantments`).

## What To Check

Run this read-only audit and report a table of findings. Only fix things if the user asks.

### 1. Module declaration vs. file existence

For each of these folders, list the `.rs` files (excluding the trait file) and check each has a matching `pub mod {name};` in `src/lib.rs` under the right `mod modules { ... }` branch:

- `src/modules/mechanics/entity|player|server|world/` → `mod mechanics { mod {category} { ... } }`
- `src/modules/enchantments/utility|vanilla/` → `mod enchantments { mod {utility|vanilla} { ... } }`
- `src/modules/items/{category}/` → `mod items { mod {category} }`
- `src/modules/recipes/vanilla/` → `mod recipes { mod vanilla { ... } }`

### 2. Registration vs. declaration

Each declared module whose trait has server-side registration must appear in the right vec in `src/lib.rs`:

- Mechanics → `PumpkinPlus::register_mechanics` `mechanics: Vec<&dyn Mechanic>`
- Recipes → `register_recipes` `recipes: Vec<&dyn Recipe>`
- Enchantments → `register_enchantments` `enchantments: Vec<&dyn Enchantment>`
- Items are **not** registered (pure builders) — exclude them from this check.

Flag any declared-but-unregistered module, and any registered-but-nonexistent import (stale wiring).

### 3. Config toggle coverage

- Mechanics: every mechanic must have a `{Name}Config` field in `MechanicsConfig` (`src/modules/mechanics/mechanic.rs`), a matching `pub use` at the top of that file, and its `enabled()` must read `cm.mechanics.{snake}.enabled`.
- Enchantments: every enchantment must have a `bool` field in `EnchantmentsConfig` (`src/modules/enchantments/enchantment.rs`), and `enabled()` must read `cm.enchantments.{snake}`.
- Recipes: every pack must have a `bool` field in `RecipesConfig` (`src/modules/recipes/recipe.rs`), and `enabled()` must read `cm.recipes.{snake}`.
- Cross-check that no *extra* config fields exist without a corresponding registered module (orphaned toggles), and no registered module is missing its toggle.

Also check `lib.rs` re-exports (`pub use modules::mechanics::...::{Name}Config;`) stay in sync for mechanics.

### 4. Command/permission pairing

For mechanics with commands: `cmds()` and `perms()` are paired by index in `Mechanic::register`. Flag mechanics where the vec lengths or intended orderings look mismatched.

### 5. Item table docs

If `src/modules/items/` changed, check the "Available Items" doc table in `src/modules/items/item.rs` lists every item module.

## How To Report

Present a table:

| Check | Module | Status | Issue |
|---|---|---|---|
| Declared in lib.rs | `Nimbus` | ❌ | File exists, registered or not — be precise |
| Config toggle | `chainmail` | ✅ | |
| Registered | `Vinemine` | ❌ | Missing from `register_enchantments` vec |

End with a short summary: N modules found, N warnings, N errors. Ask whether the user wants the mismatches fixed before editing anything.
