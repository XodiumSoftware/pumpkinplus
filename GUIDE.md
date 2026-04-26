# Installation

## Table of Contents

- [Prerequisites](#prerequisites)
- [Download Nightly Build](#download-nightly-build)
- [Build from Source](#build-from-source)
- [Configuration](#configuration)
- [Installation](#installation-1)
- [Usage](#usage)
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

The plugin uses a TOML configuration file (`config.toml`) that is automatically created on first run.

### Default Config Structure

```toml
[player]
enabled = true
join_message = "Welcome {player}!"
leave_message = "Goodbye {player}!"
kick_message = "{player} was kicked"

[motd]
enabled = false

[tablist]
enabled = false

[locator]
enabled = false
```

### Configuration Options

| Module    | Description                     | Default  |
|-----------|---------------------------------|----------|
| `player`  | Custom join/leave/kick messages | Enabled  |
| `motd`    | Custom server MOTD              | Disabled |
| `tablist` | Custom tablist header/footer    | Disabled |
| `locator` | Locator bar personalization     | Disabled |

### Placeholders

- `{player}` — Replaced with the player's name in messages

## Installation

1. Place `pumpkinplus.wasm` in your Pumpkin server's `plugins/` directory
2. Start the server
3. The plugin will load and create `config.toml` in the plugin data folder
4. Stop the server and edit `config.toml` as needed
5. Restart the server

## Usage

Once installed, the plugin runs automatically. Available features depend on enabled modules:

### Player Module

Active by default. Provides custom join/leave/kick messages.

### Locator Module

When enabled, provides the `/locator` (or `/lc`) command:

- `/locator color <color>` — Set locator bar color
- `/locator hex <hex>` — Set custom hex color
- `/locator reset` — Reset to default

### Permission Nodes

- `pumpkinplus:command.locator` — Access to `/locator` command

## Troubleshooting

### "Plugin failed to load"

- Verify your Pumpkin server supports WASM plugins
- Check server logs for detailed error messages
- Ensure the `.wasm` file is not corrupted (re-download if needed)

### "Config not loading"

- Check that `config.toml` is valid TOML
- The plugin will regenerate the config if it's invalid
- Stop the server before editing the config file

### Commands not working

- Ensure the module is enabled in `config.toml`
- Check that you have the required permission node
- Verify the plugin loaded successfully in server logs

### Build errors

- Make sure you have the `wasm32-wasip2` target installed:
  ```bash
  rustup target add wasm32-wasip2
  ```
- Ensure you're using the latest stable Rust version

---

<p align="right"><a href="#readme-top">▲</a></p>
