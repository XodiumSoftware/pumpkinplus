---
name: add-module
description: Scaffolds a new PumpkinPlus module with config wiring and lib.rs registration.
---

# Add a Module

Use this skill when the user wants to add a new feature to PumpkinPlus. Every feature is a module implementing the `Module` trait:

| Kind     | Trait    | Location                                    | Config style              | Registered with server?             |
| -------- | -------- | ------------------------------------------- | ------------------------- | ----------------------------------- |
| `module` | `Module` | `src/modules/{entity,player,server,world}/` | Own `{Name}Config` struct | Yes (events, commands, permissions) |

## Before Writing Code

1. Ask for:
    - The module name (e.g. `Greatsword`, `Tether`).
    - Which category folder (`entity`, `player`, `server`, `world`) and whether it needs commands, permissions, or events.
2. If unsure about values, mirror a similar existing module:
    - `src/modules/server/tablist.rs`

## Naming

- File/module name: `snake_case` (e.g. `nether_wart_block.rs`).
- Struct: `PascalCase` matching the file (e.g. `NetherWartBlock`).
- Config section: same `snake_case` as the file.

## Common Skeleton

Every module file follows this item order (per `AGENTS.md` code style):

1. `//!` module docs with a config table
2. `use` imports
3. `const` values
4. `#[derive(Default)] pub struct {Name};`
5. `impl Module for {Name}` — `enabled()`, then `cmds()`/`perms()`/`events()`
6. `impl EventHandler<T> for {Name}` blocks
7. Helper fns (private, at bottom, before the local config struct)
8. `{Name}Config` struct at bottom

Event handlers use `impl EventHandler<T> for {Name}` + `fn handle(&self, server: Server, event: EventData<T>) -> EventData<T>` and are registered in `events()` via `self.register_event::<T>(context, EventPriority::Normal, true);`.

Never log with `println!` — use `tracing::{info, debug, error}`.

## Module

1. Create `src/modules/{category}/{name}.rs`.
2. `enabled()` must be:
    ```rust
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.{snake_name}.enabled)
    }
    ```
3. Override `cmds()`/`perms()` only if there are commands. Permission node pattern: `{PLUGIN_ID}:command.{name}` via `PLUGIN_ID` or `namespaced_id!`. `perms()` returns `Vec<Permission>` paired **by index** with `cmds()`.
4. Define at the bottom of the same file:
    ```rust
    /// Configuration for the {name} module.
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct {Name}Config {
        /// Whether this module is active.
        pub enabled: bool,
        // module fields...
    }
    ```
5. Wire the config into `src/modules/module.rs`:
    - Add `pub(crate) use crate::modules::{category}::{name}::{Name}Config;` with the other `pub(crate) use` lines.
    - Add `pub {snake_name}: {Name}Config,` with a `///` doc comment to `MechanicsConfig` (keep alphabetical with existing fields).
6. Wire into `src/lib.rs`:
    - Add `pub(crate) mod {name};` inside `mod modules { mod {category} { ... } }`, alphabetical.
    - Add `use crate::modules::{category}::{name}::{Name};` with the other imports.
    - Add `&{Name},` to the `modules` vec inside `register_modules`, keeping existing ordering style.
