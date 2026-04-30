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
| **Serialization** | serde + toml                                                | Config serialization             |
| **Config**        | figment                                                     | TOML config with merge semantics |
| **Logging**       | tracing                                                     | Structured logging               |
| **Docs**          | rustdoc (via `cargo doc`)                                   | API documentation                |

### Pumpkin API Resources

- **Repository**: https://github.com/Pumpkin-MC/Pumpkin
- **Plugin API**: https://github.com/Pumpkin-MC/Pumpkin/tree/master/pumpkin-plugin-api

### Pumpkin API Notes

- Plugin entry point implements `Plugin` trait
- Events use `EventHandler<T>` trait with `EventPriority`
- Commands use `CommandHandler` trait with `CommandNode` builder pattern
- Plugin registered via `register_plugin!(PluginName)` macro
- WASM target requires `wasm32-wasip2` toolchain

## Quick Commands

```bash
# Build the WASM plugin (debug)
cargo build --target wasm32-wasip2

# Build the WASM plugin (release, optimized)
cargo build --release --target wasm32-wasip2

# Generate documentation
cargo doc --no-deps --target wasm32-wasip2
```

## Architecture Overview

### Entry Point

**`PumpkinPlus`** — implements `Plugin` from `pumpkin_plugin_api`:

1. **Registration**: Via `register_plugin!(PumpkinPlus)` macro
2. **`on_load`**:
    - Initializes `ConfigManager` (loads/creates `config.toml`)
    - Registers all module configs
    - Calls `Module::register` for each enabled module
3. **`on_unload`**: Logs farewell message

### Module System

Every feature implements the **`Module`** trait (`src/modules/module.rs`):

| Method       | Purpose                                          | Default     |
|--------------|--------------------------------------------------|-------------|
| `enabled()`  | Returns whether module is active                 | Required    |
| `cmds()`     | Returns `Vec<Command>` to register               | Empty vec   |
| `perms()`    | Returns `HashSet<String>` permission nodes       | Empty set   |
| `events()`   | Registers event handlers via `Context`           | No-op       |
| `register()` | Calls `events()`, registers commands/permissions | Implemented |

Modules are plain structs (not singletons) instantiated with `Default::default()` and passed to `register()` in `on_load`.

### Configuration

**`ConfigManager`** (`src/config.rs`) — JSON-backed config with merge semantics:

- Config located at `{data_folder}/config.toml`
- On first load: creates file with all registered defaults
- On subsequent loads: merges user values with defaults (preserves extra fields)
- Each module owns a nested `Config` struct with `enabled: bool` field

Config key derived from type name automatically (e.g., `PlayerConfig` → `"player"`).

### Active Modules

| Module    | File                               | Description                                                                   |
|-----------|------------------------------------|-------------------------------------------------------------------------------|
| `Player`  | `src/modules/mechanics/player.rs`  | Custom join/leave/kick messages, chat format/filter                           |
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
        ├── player.rs         # Join/leave/kick messages, chat
        ├── tablist.rs        # Tab list header/footer
        └── locator.rs        # Locator bar commands
```

### Key Conventions

- `unsafe_code` forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`)
- All Clippy warnings enabled (`[lints.clippy] all = "warn"`)
- Config structs: `Debug`, `Clone`, `Default`, `Serialize`, `Deserialize`
- Config fields documented with `///` comments
- Permission pattern: `{PLUGIN_ID}:command.{name}` (e.g., `pumpkinplus:command.locator`)
- Module configs accessed via `ConfigManager::get().unwrap_or_default()`
- Event handlers return `EventData<T>` (may modify event)
- Release profile: LTO + strip for minimal WASM size

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

1. **ARCHITECTURE.md** — Update if you:
    - Add/remove modules
    - Change the module system
    - Modify configuration patterns
    - Change project structure

2. **rustdoc comments** — Add/update if you:
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
6. Implement `Module` trait:
    - `enabled()` — check config
    - `events()` — register event handlers (if needed)
    - `cmds()` — return commands (if needed)
    - `perms()` — return permission nodes (if commands)
7. Implement `EventHandler<T>` for each event (if needed)
8. In `src/lib.rs`:
    - Add `pub mod {module}` in `modules::mechanics`
    - Add `pub use` for config type
    - Register config in `on_load`: `manager.register::<{Module}Config>();`
    - Instantiate and register module in modules vec
9. Update `ARCHITECTURE.md` module table
10. Run `cargo build --target wasm32-wasip2` to verify

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
