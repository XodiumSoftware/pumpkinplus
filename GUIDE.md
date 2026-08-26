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

The plugin uses a JSON configuration file (`config.json`) that is automatically created on first run.

> **Note:** All gameplay mechanics and recipe packs are disabled by default. Enable the modules you want in `config.json` and restart the server.

### Default Config Structure

```json
{
  "chat": {
    "enabled": false,
    "chat_format": "",
    "chat_filter": []
  },
  "enderchest": {
    "enabled": false,
    "gamemodes": ["Survival", "Adventure"],
    "actions": ["RightClickAir"]
  },
  "griefing": {
    "enabled": false,
    "cancelled_entities": [
      "Blaze",
      "Creeper",
      "EnderDragon",
      "Enderman",
      "Fireball",
      "SmallFireball",
      "Wither"
    ]
  },
  "head": {
    "enabled": false,
    "skull_drop_chance": 0.01
  },
  "messages": {
    "enabled": false,
    "join_msg": "",
    "leave_msg": "",
    "kick_msg": ""
  },
  "nickname": {
    "enabled": false
  },
  "openable": {
    "enabled": false,
    "gamemodes": ["Survival", "Adventure"],
    "actions": ["RightClickBlock"],
    "knock_enabled": false,
    "knock_gamemodes": ["Survival", "Adventure"],
    "knock_sneaking_required": true
  },
  "recipes": {
    "chainmail": false,
    "diamond_recycle": false,
    "ice_breakdown": false,
    "nether_wart_block": false,
    "painting": false,
    "rotten_flesh": false,
    "wood_log": false,
    "wool_to_string": false
  },
  "tablist": {
    "enabled": false,
    "header": "",
    "footer": ""
  }
}
```

### Configuration Options

| Module | Description | Default |
|--------|-------------|---------|
| `chat` | Chat format and word filtering | Disabled |
| `enderchest` | Open personal enderchest by right-clicking air with an ender chest | Disabled |
| `griefing` | Cancel block-change and explosion events from configured mobs | Disabled |
| `messages` | Custom join/leave/kick messages | Disabled |
| `nickname` | Set a persistent nickname via `/nickname` or `/nick` | Disabled |
| `openable` | Synchronize double doors and sneaky door-knocking | Disabled |
| `tablist` | Custom tab-list header/footer with live placeholders | Disabled |

### Recipe Packs

| Config Field | Description | Default |
|--------------|-------------|---------|
| `recipes.chainmail` | Craft chainmail armor pieces using iron bars | Disabled |
| `recipes.diamond_recycle` | Smelt diamond tools/armor back into diamonds | Disabled |
| `recipes.ice_breakdown` | Break blue ice into packed ice and packed ice into ice | Disabled |
| `recipes.nether_wart_block` | Break nether wart blocks back into nether warts | Disabled |
| `recipes.painting` | Placeholder shapeless recipes for painting variants (requires upstream stonecutter / data-component support to match vanilla behavior) | Disabled |
| `recipes.rotten_flesh` | Cook rotten flesh into leather via furnace, smoker, and campfire | Disabled |
| `recipes.wood_log` | Convert wood/hyphae blocks back into 4 logs/stems | Disabled |
| `recipes.wool_to_string` | Convert any wool block into 4 string | Disabled |

### Placeholders

| Placeholder | Available in | Description |
|-------------|--------------|-------------|
| `{player}` | `messages.*`, `chat_format` | Player's display name |
| `{message}` | `chat_format` | Original chat message |
| `{online}` | `tablist.header`, `tablist.footer` | Number of online players |
| `{tps}` | `tablist.header`, `tablist.footer` | Server TPS (ticks per second) |
| `{mspt}` | `tablist.header`, `tablist.footer` | Milliseconds per tick |

## Installation

1. Place `pumpkinplus.wasm` in your Pumpkin server's `plugins/` directory
2. Start the server
3. The plugin will load and create `config.json` in the plugin data folder
4. Stop the server and edit `config.json` as needed
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

When enabled, overrides the default join/leave/kick messages. Leave any message empty to keep the vanilla message for that event.

### Nickname Module

When enabled, players can use `/nickname <name>` or `/nick <name>` to set a nickname, or `/nickname` / `/nick` with no argument (or `/nickname clear`) to remove it. Nicknames are persisted across logins.

### Openable Module

When enabled:

- Right-clicking one door of a double-door pair toggles both doors together.
- Sneaking and left-clicking a door with an empty main hand cancels the interaction so the door is not damaged. (A knock sound is planned but disabled until the Pumpkin API exports the `Sound` enum.)

### Tablist Module

When enabled, sets a custom header and footer for every player's tab list. The header/footer are refreshed on player join/leave and periodically so `{tps}` and `{mspt}` stay current.

### Recipes

Each recipe pack can be enabled independently in the `recipes` section of `config.json`:

| Recipe Pack | Config Field | Description |
|-------------|--------------|-------------|
| Chainmail | `chainmail` | Craft chainmail armor pieces using iron bars |
| DiamondRecycle | `diamond_recycle` | Smelt diamond tools/armor back into diamonds |
| IceBreakdown | `ice_breakdown` | Break blue ice into packed ice and packed ice into ice |
| NetherWartBlock | `nether_wart_block` | Break nether wart blocks back into nether warts |
| Painting | `painting` | Placeholder shapeless recipes for painting variants (requires upstream stonecutter / data-component support to match vanilla behavior) |
| RottenFlesh | `rotten_flesh` | Cook rotten flesh into leather via furnace, smoker, and campfire |
| WoodLog | `wood_log` | Convert wood/hyphae blocks back into 4 logs/stems |
| WoolToString | `wool_to_string` | Convert any wool block into 4 string |

## Commands & Permissions

| Command | Alias | Permission | Description |
|---------|-------|------------|-------------|
| `/nickname [name]` | `/nick` | `pumpkinplus:command.nickname` | Set or remove your nickname |

All command permissions default to `Allow`.

### Notes on Experimental Modules

There are currently no experimental modules. All shipped mechanics are fully wired and can be enabled in `config.json`.

## Troubleshooting

### "Plugin failed to load"

- Verify your Pumpkin server supports WASM plugins
- Check server logs for detailed error messages
- Ensure the `.wasm` file is not corrupted (re-download if needed)

### "Config not loading"

- Check that `config.json` is valid JSON
- The plugin will regenerate the config if it's invalid
- Stop the server before editing the config file

### Commands not working

- Ensure the module is enabled in `config.json`
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
