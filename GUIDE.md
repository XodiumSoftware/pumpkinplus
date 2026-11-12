# Installation

## Table of Contents

- [Prerequisites](#prerequisites)
- [Download Nightly Build](#download-nightly-build)
- [Build from Source](#build-from-source)
- [Configuration](#configuration)
- [Installation](#installation-1)
- [Usage](#usage)
- [Commands & Permissions](#commands--permissions)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

- [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft server
- The server must support WASM plugins (via `wasm32-wasip2` target)

## Download Nightly Build

Download pre-built WASM binaries from GitHub releases.

### Setup

1. Download the latest nightly release:

    ```bash
    curl -L -o pumpkinplus.wasm https://github.com/XodiumSoftware/pumpkinplus/releases/download/nightly/pumpkinplus-wasm32-wasip2
    ```

2. Place the `.wasm` file in your Pumpkin server's `plugins/` directory

## Build from Source

Build the plugin yourself using Rust.

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- `wasm32-wasip2` target: `rustup target add wasm32-wasip2`

### Setup

1. Clone the repository:

    ```bash
    git clone https://github.com/XodiumSoftware/pumpkinplus.git
    cd pumpkinplus
    ```

2. Build the plugin:

    ```bash
    cargo build --release --target wasm32-wasip2
    ```

3. The output file is at:
    ```
    target/wasm32-wasip2/release/pumpkinplus.wasm
    ```

## Configuration

Configuration lives in a TOML file in the plugin's data folder:

| File             | Purpose                                                       |
| ---------------- | ------------------------------------------------------------- |
| `config.toml`    | Master on/off toggle for the mechanics module group           |
| `mechanics.toml` | Per-mechanic settings (`chat`, `enderchest`, `messages`, ...) |

Both files are created on first run with sensible defaults. Setting the master toggle in `config.toml` to `false` disables the entire group and skips loading its own file.

> **Note:** All gameplay mechanics are disabled by default. Enable the modules you want in `mechanics.toml` and restart the server.

### `config.toml`

```toml
[modules]
mechanics = true
```

### `mechanics.toml` (excerpt)

```toml
[chat]
enabled     = false
chat_format = ""
chat_filter = []

[messages]
enabled   = false
join_msg  = ""
leave_msg = ""
kick_msg  = ""

# ... one [section] per mechanic
```

### Configuration Options

| Module            | Description                                                        | Default  |
| ----------------- | ------------------------------------------------------------------ | -------- |
| `chat`            | Chat format and word filtering                                     | Disabled |
| `enderchest`      | Open personal enderchest by right-clicking air with an ender chest | Disabled |
| `griefing`        | Cancel block-change and explosion events from configured mobs      | Disabled |
| `messages`        | Custom join/leave/kick messages                                    | Disabled |
| `nickname`        | Set a persistent nickname via `/nickname` or `/nick`               | Disabled |
| `openable`        | Synchronize double doors and sneaky door-knocking                  | Disabled |
| `spawnprotection` | Keep hostile mobs from spawning in the spawn protection area       | Disabled |
| `tablist`         | Custom tab-list header/footer with live placeholders               | Disabled |

### Placeholders

| Placeholder | Available in                       | Description                   |
| ----------- | ---------------------------------- | ----------------------------- |
| `{player}`  | `messages.*`, `chat_format`        | Player's display name         |
| `{message}` | `chat_format`                      | Original chat message         |
| `{online}`  | `tablist.header`, `tablist.footer` | Number of online players      |
| `{tps}`     | `tablist.header`, `tablist.footer` | Server TPS (ticks per second) |
| `{mspt}`    | `tablist.header`, `tablist.footer` | Milliseconds per tick         |

Messages, nicknames, and the tab-list header/footer also support [MiniMessage](https://docs.advntr.dev/minimessage/format.html) formatting tags (e.g. `<gradient:red:blue>`, `<bold>`, `<hover:show_text:...>`). Placeholders are resolved first, then MiniMessage tags are parsed.

## Installation

1. Place `pumpkinplus.wasm` in your Pumpkin server's `plugins/` directory
2. Start the server
3. The plugin will load and create `config.toml` plus the per-group TOML files in the plugin data folder
4. Stop the server and edit the TOML files as needed
5. Restart the server

## Usage

Once installed, the plugin runs automatically. Available features depend on enabled modules.

### Chat Module

When enabled, formats chat messages and/or filters blocked words.

- `chat_format` — Message format. Use `{player}` and `{message}`.
- `chat_filter` — Case-insensitive list of substrings; messages containing any entry are cancelled.

### Enderchest Module

When enabled, right-clicking in the air while holding an ender chest opens the player's personal ender chest. Configurable by gamemode and interaction action.

### Griefing Module

When enabled, cancels `EntityChangeBlockEvent` and `EntityExplodeEvent` for the entity types listed in `cancelled_entities`. Add or remove vanilla entity type names as needed.

### Messages Module

When enabled, overrides the default join/leave/kick messages. Leave any message empty to keep the vanilla message for that event. Messages support MiniMessage formatting tags, e.g. `"<green><bold>{player}</bold> joined the game"`.

### Nickname Module

When enabled, players can use `/nickname <name>` or `/nick <name>` to set a nickname, or `/nickname` / `/nick` with no argument (or `/nickname clear`) to remove it. Nicknames are persisted across logins and support MiniMessage formatting tags, e.g. `/nick <rainbow>Illyrius`.

### Openable Module

When enabled:

- Right-clicking one door of a double-door pair toggles both doors together.
- Sneaking and left-clicking a door with an empty main hand cancels the interaction so the door is not damaged. (A knock sound is planned but disabled until the Pumpkin API exports the `Sound` enum.)

### SpawnProtection Module

When enabled, cancels hostile-mob spawns inside the spawn protection area. The protected area is a square of `(2 * radius + 1)` blocks centered on the world spawn point, matching vanilla Minecraft's Chebyshev distance check.

- `radius` — Spawn protection radius in blocks (default `16`, matching the vanilla default). Set to `0` or a negative value to disable protection at runtime without turning off the module.

> **Note:** The Pumpkin plugin API does not currently expose the server's `spawn-protection` setting, so this module uses its own `radius` value. Damage protection for players/villagers and chunk-load monster despawning are not yet implemented — those require API support for entity lookup in the relevant events.

### Tablist Module

When enabled, sets a custom header and footer for every player's tab list. The header/footer are refreshed on player join/leave and periodically so `{tps}` and `{mspt}` stay current. Header and footer support MiniMessage formatting tags.

## Commands & Permissions

| Command            | Alias   | Permission                     | Description                 |
| ------------------ | ------- | ------------------------------ | --------------------------- |
| `/nickname [name]` | `/nick` | `pumpkinplus:command.nickname` | Set or remove your nickname |

All command permissions default to `Allow`.

### Notes on Experimental Modules

There are currently no experimental modules. All shipped mechanics are fully wired and can be enabled via their TOML config file.

## Troubleshooting

### "Plugin failed to load"

- Verify your Pumpkin server supports WASM plugins
- Check server logs for detailed error messages
- Ensure the `.wasm` file is not corrupted (re-download if needed)

### "Config not loading"

- Check that the offending `.toml` file is valid TOML
- The plugin will regenerate a config file if it's invalid and fall back to defaults
- Stop the server before editing config files

### Commands not working

- Ensure the module is enabled in `mechanics.toml`
- Ensure the `mechanics` master toggle in `config.toml` is `true`
- Check that you have the required permission node
- Verify the plugin loaded successfully in server logs

### Build errors

- Make sure you have the `wasm32-wasip2` target installed:
    ```bash
    rustup target add wasm32-wasip2
    ```
- Ensure you're using the latest stable Rust version
- Run `cargo update` to pull the latest `pumpkin-plugin-api` revision

---

<p align="right"><a href="#readme-top">▲</a></p>
