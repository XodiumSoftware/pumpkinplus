# Locator

Allows players to personalize the colour of their locator bar.

> **Status: Stub** — The command is registered but the implementation is pending the Pumpkin API exposing locator bar controls.

## Commands & Permissions

| Command                        | Aliases | Permission                    | Default  | Description                                                   |
|--------------------------------|---------|-------------------------------|----------|---------------------------------------------------------------|
| `/locator <color\|hex\|reset>` | `lc`    | `pumpkinplus:command.locator` | Everyone | Set locator bar colour by name, hex code, or reset to default |

## Configuration

| Field     | Default | Description                   |
|-----------|---------|-------------------------------|
| `enabled` | `false` | Whether this module is active |

## Behaviour

- Accepts a named colour (e.g. `red`, `aqua`), a hex code (e.g. `#FF5500`), or `reset` to restore the default.
- Currently responds with `"Not yet implemented."` until the API is available.
