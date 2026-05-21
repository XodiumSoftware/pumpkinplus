# ARCHITECTURE.md

This file provides guidance when working with code in this repository.

## Project Overview

PumpkinPlus is a Pumpkin Minecraft plugin that enhances base gameplay. Built with Rust + Cargo, targeting `wasm32-wasip2` as a `cdylib`. Uses the `pumpkin-plugin-api` for event handling, commands, and server interaction.

## Build & Run Commands

```bash
# Build the WASM plugin (debug) and copy to .server/plugins/
powershell -ExecutionPolicy Bypass -File build.ps1

# Build the WASM plugin (release)
powershell -ExecutionPolicy Bypass -File build.ps1 -Release
```

Testing is done via integration testing: build the WASM plugin and load it in a Pumpkin server.

## Architecture

### Entry Point

- **`PumpkinPlus`** — implements `Plugin` from `pumpkin_plugin_api`. Registered via `register_plugin!(PumpkinPlus)`.
    - `on_load`: initializes `ConfigManager` (loads or creates `config.json`), then registers all modules via `Module::register`.
    - `on_unload`: logs a farewell message.

### Module System

Every feature implements the **`Module`** trait (`src/modules/module.rs`). The trait provides:

- **`enabled()`** — returns whether the module is active (driven by each module's `Config.enabled` field).
- **`cmds()`** — returns `Vec<Command>` to register with the server. Empty by default.
- **`perms()`** — returns `HashSet<String>` of permission nodes paired with commands by index. Empty by default.
- **`events()`** — registers event handlers via `Context::register_event_handler`. No-op by default.
- **`register()`** — calls `events()`, then registers each command with its paired permission. Short-circuits if `enabled()` is false.

Modules are plain structs (not singletons) instantiated with `Default::default()` and passed to `register()` in `on_load`.

### Configuration

**`ConfigManager`** (`src/config.rs`) is a single JSON-backed struct that aggregates all module configs. On `on_load`:

- If `config.json` exists in the plugin data folder, it is deserialized and merged with defaults.
- If not found, the default config is written to disk and returned.
- Each module owns a nested **`Config`** struct (derived `Serialize`/`Deserialize`) with an `enabled: bool` field and any module-specific fields. `ConfigManager` holds one field per module config.

### Modules

| Module    | File                               | Status | Description                                                                                                                      |
|-----------|------------------------------------|--------|----------------------------------------------------------------------------------------------------------------------------------|
| `Messages` | `src/modules/mechanics/player/messages.rs` | Active | Custom join/leave/kick messages. Handles `PlayerJoinEvent`, `PlayerLeaveEvent`, `PlayerLoginEvent`. Uses `{player}` placeholder. |
| `Chat`     | `src/modules/mechanics/server/chat.rs`     | Active | Chat formatting and word filtering. Handles `PlayerChatEvent`. Uses `{player}` and `{message}` placeholders. |
| `Tablist` | `src/modules/mechanics/server/tablist.rs` | Active | Dynamic tab list header/footer with `{player}`, `{online}`, `{tps}`, `{mspt}` placeholders. |
| `Openable`| `src/modules/mechanics/world/openable.rs` | Active | Double door synchronization. Handles `PlayerInteractEvent`. |
| `Bat`     | `src/modules/mechanics/entity/bat.rs`       | Stub   | Custom bat drops (phantom membrane). Awaiting `EntityDeathEvent` in Pumpkin API. |
| `Griefing`| `src/modules/mechanics/entity/griefing.rs`  | Stub   | Mob griefing prevention (block changes and explosions). Awaiting `EntityChangeBlockEvent` / `EntityExplodeEvent` in Pumpkin API. |
| `Husk`    | `src/modules/mechanics/entity/husk.rs`      | Stub   | Custom husk sand drops with camel rider bonus. Awaiting `EntityDeathEvent` in Pumpkin API. |
| `SpawnEgg`| `src/modules/mechanics/entity/spawn_egg.rs` | Stub   | Rare spawn egg drops on entity death. Awaiting `EntityDeathEvent` in Pumpkin API. |

### Package Structure

| Path                     | Contents                                                |
|--------------------------|---------------------------------------------------------|
| `src/lib.rs`             | `PumpkinPlus` plugin struct, entry point                |
| `src/config.rs`          | `ConfigManager` — JSON config load/save/merge            |
| `src/modules/module.rs`  | `Module` trait definition                               |
| `src/modules/mechanics/player/` | Player features: messages, enderchest, locator   |
| `src/modules/mechanics/server/` | Server features: chat, tablist                   |
| `src/modules/mechanics/world/`  | World features: openable (double doors)          |

### Key Conventions

- `unsafe_code` is forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`).
- All Clippy warnings are enabled (`[lints.clippy] all = "warn"`).
- Config structs are defined as a nested `Config` inside the same file as their module, deriving `Debug`, `Clone`, `Default`, `Serialize`, `Deserialize`.
- Permission nodes follow the pattern `{PLUGIN_ID}:command.{name}` (e.g. `pumpkinplus:command.locator`).
- The `{player}` placeholder in message config fields is substituted at runtime via `.replace("{player}", &player.get_name())`.
- The release profile enables LTO and strips symbols for minimal WASM output size.
