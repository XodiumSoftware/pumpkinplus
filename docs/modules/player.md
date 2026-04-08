# Player

Handles custom messages shown to the server when a player joins, leaves, or is kicked during login.

## Configuration

| Field         | Default | Description                                                        |
|---------------|---------|--------------------------------------------------------------------|
| `enabled`     | `false` | Whether this module is active                                      |
| `join_msg`    | `""`    | Message broadcast to the server when a player joins                |
| `leave_msg`   | `""`    | Message broadcast to the server when a player leaves               |
| `kick_msg`    | `""`    | Message shown to the player when they are kicked during login      |
| `chat_format` | `""`    | Custom chat format. Use `{player}` and `{message}` as placeholders |
| `chat_filter` | `[]`    | List of blocked words/phrases (case-insensitive match)             |

If a message field is left empty, the corresponding default server behaviour is preserved.

## Placeholders

| Placeholder | Available in                                       |
|-------------|----------------------------------------------------|
| `{player}`  | `join_msg`, `leave_msg`, `kick_msg`, `chat_format` |
| `{message}` | `chat_format`                                      |

## Behaviour

- Listens on `PlayerJoinEvent`, `PlayerLeaveEvent`, `PlayerLoginEvent`, and `PlayerChatEvent` at `Highest` priority.
- Replaces the respective event message with the configured string (after placeholder substitution).
- If the configured string is empty, the event message is left unchanged.
- Chat messages containing any `chat_filter` entry (case-insensitive) are cancelled.
- Chat messages are reformatted according to `chat_format` if configured.
