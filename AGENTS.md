# pumpkinplus — Claude Code Context

## Project at a Glance

- **Name:** pumpkinplus
- **Type:** Pumpkin Minecraft plugin (WASM)
- **MC Version:** Latest Pumpkin (tracks nightly)
- **Language:** Rust (Edition 2024)
- **Build Tool:** Cargo
- **Output:** `pumpkinplus.wasm` (WASI Preview 2)
- **Target:** `wasm32-wasip2`

## APIs & Tools

| Category          | Technology                                                  | Purpose                          |
|-------------------|-------------------------------------------------------------|----------------------------------|
| **Core API**      | [pumpkin-plugin-api](https://github.com/Pumpkin-MC/Pumpkin) | Minecraft server plugin API      |
| **Language**      | Rust 2024                                                   | Systems language                 |
| **Build Tool**    | Cargo                                                       | Build automation                 |
| **Serialization** | serde + json                                                | Config serialization             |
| **Config**        | config                                                     | JSON config with merge semantics |
| **Logging**       | tracing                                                     | Structured logging               |
| **Docs**          | rustdoc (via `cargo doc`)                                   | API documentation                |

### Pumpkin API Resources

- **Repository**: https://github.com/Pumpkin-MC/Pumpkin
- **Plugin API**: https://github.com/Pumpkin-MC/Pumpkin/tree/master/pumpkin-plugin-api
- **Plugin Knowledge Base**: https://book.vypal.me/

### Pumpkin API Notes

- Plugin entry point implements `Plugin` trait
- Events use `EventHandler<T>` trait with `EventPriority`
- Commands use `CommandHandler` trait with `CommandNode` builder pattern
- Plugin registered via `register_plugin!(PluginName)` macro
- WASM target requires `wasm32-wasip2` toolchain

## Quick Commands

```bash
# Build the WASM plugin (debug) and copy to .server/plugins/
powershell -ExecutionPolicy Bypass -File build.ps1

# Build the WASM plugin (release)
powershell -ExecutionPolicy Bypass -File build.ps1 -Release

# Generate documentation
cargo doc --no-deps --target wasm32-wasip2

# Lint with pedantic lints enabled; every warning is treated as an error.
cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings
```

## Architecture Overview

### Entry Point

**`PumpkinPlus`** — implements `Plugin` from `pumpkin_plugin_api`:

1. **Registration**: Via `register_plugin!(PumpkinPlus)` macro
2. **`on_load`**:
    - Initializes `ConfigManager` (loads/creates `config.json`)
    - Registers all module configs
    - Calls `Module::register` for each enabled module
3. **`on_unload`**: Logs farewell message

### Module System

Every feature implements the **`Mechanic`** trait (`src/modules/mechanics/mechanic.rs`):

| Method       | Purpose                                          | Default     |
|--------------|--------------------------------------------------|-------------|
| `enabled()`  | Returns whether module is active                 | Required    |
| `cmds()`     | Returns `Vec<Command>` to register               | Empty vec   |
| `perms()`    | Returns `HashSet<String>` permission nodes       | Empty set   |
| `events()`   | Registers event handlers via `Context`           | No-op       |
| `register()` | Calls `events()`, registers commands/permissions | Implemented |

Modules are plain structs (not singletons) instantiated with `Default::default()` and passed to `register()` in `on_load`.

### Configuration

**`ConfigManager`** (`src/config.rs`) — JSON-backed config using the [`config`](https://crates.io/crates/config) crate:

- Config located at `{data_folder}/config.json`
- On first load: creates file with all module defaults
- On subsequent loads: merges user values with defaults and preserves extra fields
- All module configs live in a single `PluginConfig` struct; modules access their section via `ConfigManager::get().map(|cm| cm.section)`

### Active Modules

| Module    | File                               | Description                                                                   |
|-----------|------------------------------------|-------------------------------------------------------------------------------|
| `Messages`| `src/modules/mechanics/player/messages.rs` | Custom join/leave/kick messages                                    |
| `Chat`    | `src/modules/mechanics/server/chat.rs`   | Chat format/filter                                                 |
| `Tablist` | `src/modules/mechanics/tablist.rs` | Dynamic tab list header/footer with `{player}`, `{online}`, `{tps}`, `{mspt}` |
| `Locator` | `src/modules/mechanics/locator.rs` | Locator bar personalization (`/locator` command, stub)                        |

### Placeholders

| Placeholder | Available in       | Description                   |
|-------------|--------------------|-------------------------------|
| `{player}`  | All message fields | Player's display name         |
| `{message}` | `chat_format`      | Original chat message         |
| `{online}`  | `header`, `footer` | Number of online players      |
| `{tps}`     | `header`, `footer` | Server TPS (ticks per second) |
| `{mspt}`    | `header`, `footer` | Milliseconds per tick         |

### Project Structure

```
src/
├── lib.rs                    # Plugin entry point, `PumpkinPlus` struct
├── config.rs                 # `ConfigManager` — JSON config load/save
└── modules/
    ├── module.rs             # `Module` trait definition
    └── mechanics/
        ├── player/
        │   ├── messages.rs   # Join/leave/kick messages
        │   ├── enderchest.rs # Enderchest sharing
        │   └── locator.rs    # Locator bar commands
        ├── server/
        │   ├── chat.rs       # Chat format/filter
        │   └── tablist.rs    # Tab list header/footer
        └── world/
            └── openable.rs   # Double door sync
```

### Key Conventions

- `unsafe_code` forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`)
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors. (`[lints.clippy] all = "warn"`)
- Config structs: `Debug`, `Clone`, `Default`, `Serialize`, `Deserialize`
- Config fields documented with `///` comments
- Permission pattern: `{PLUGIN_ID}:command.{name}` (e.g., `pumpkinplus:command.locator`)
- Module configs accessed via `ConfigManager::get().unwrap_or_default()`
- Event handlers return `EventData<T>` (may modify event)
- Release profile: LTO + strip for minimal WASM size
- **Before checking API availability, run `cargo update` to fetch the latest `pumpkin-plugin-api` revision.** The API is git-tracked and may have added new events or types since the last local checkout.

### Code Style

**Order of items in modules:**

1. **`//!`** — Module-level documentation
2. **`use`** — imports (external, crate, std)
3. **`const`** — module constants (e.g., `PLUGIN_ID`)
4. **`struct`**/`enum`** definitions (documented fields)
5. **`impl Module`** — module trait implementation
6. **`impl EventHandler<T>`** — event handler implementations
7. **`impl OtherTrait`** — other trait implementations
8. **Type alias** — `pub type ModuleConfig = Config;`
9. **`Config` struct** — module config (at bottom)

**Documentation:**

- Module-level docs with `//!` describing module purpose and config table
- All `pub` items have `///` rustdoc comments
- Config fields documented inline
- Placeholder tables for user-facing strings

**Error Handling:**

- Use `tracing::error!` for logging, not `println!`
- Event handler failures use `.expect()` for critical registrations
- Config parsing errors logged gracefully, fall back to defaults

## Testing

- Unit tests in `src/config.rs` under `#[cfg(test)]`
- Run with: `cargo test`
- Integration testing: build WASM and load in Pumpkin server

## Important Notes

- All modules disabled by default (`enabled: false`)
- Config file auto-created with defaults on first load
- WASM output must be copied to server's `plugins/` folder
- Pumpkin server must support WASM plugins (WASI Preview 2)
- Plugin API is unstable and may change

## Claude Code Workflow

### Task Management

**When creating tasks:**

- Number tasks in the name (e.g., "1. Add MOTD module", "2. Fix chat filter")

**After completing each task:**

- Ask the user if they want to git commit the changes or adjust before committing

**When all tasks in a worktree are complete:**

- Ask the user if they want to git publish (push) the changes or adjust before publishing

### After Making Edits

**Always update documentation when code changes:**

1. **rustdoc comments** — Add/update if you:
    - Add new modules or public APIs
    - Change config fields or placeholders
    - Add commands or events
    - **Run `cargo doc --no-deps --target wasm32-wasip2`** to verify

**Rule of thumb:** If a code change would confuse someone reading the docs, update the docs.

## CI/CD

GitHub Actions workflows in `.github/workflows/`:

- **plugin.yml** — Builds WASM plugin on push/PR, uploads artifact
- **docs.yml** — Generates rustdoc and deploys to GitHub Pages
- **enforce_pr_title.yml** — Validates PR titles follow conventional commits

## Adding a New Module

To add a new module, follow these steps:

1. Create new file in `src/modules/mechanics/{module}.rs`
2. Add module-level docs `//!` with description and config table
3. Define `{Module}Config` struct with `enabled: bool` + module fields
4. Implement `Default` for config with sensible defaults
5. Create `{Module}` struct deriving `Default`
6. Implement `Mechanic` trait:
    - `enabled()` — check config via `ConfigManager::get().is_some_and(|cm| cm.{section}.enabled)`
    - `events()` — register event handlers (if needed)
    - `cmds()` — return commands (if needed)
    - `perms()` — return permission nodes (if commands)
7. Implement `EventHandler<T>` for each event (if needed)
8. In `src/config.rs`:
    - Add `pub use crate::modules::mechanics::{module}::{Module}Config;`
    - Add a `pub {section}: {Module}Config` field to `PluginConfig`
9. In `src/lib.rs`:
    - Add `pub mod {module}` in `modules::mechanics`
    - Instantiate and register module in modules vec
10. Run `cargo build --target wasm32-wasip2` to verify

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
