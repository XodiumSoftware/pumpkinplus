---
name: add-command
description: Adds a slash command (with aliases, args, and permission wiring) to an existing PumpkinPlus mechanic module.
---

# Add a Command

Use this skill when the user wants to add a chat command to an existing PumpkinPlus mechanic. Commands live on **mechanics only** — enchantments, items, and recipes don't expose commands.

Reference implementation: `src/modules/mechanics/player/nickname.rs`.

## Before Writing Code

1. Ask the user which mechanic owns the command (or use the `add-module` skill first if the mechanic doesn't exist yet).
2. Ask for:
   - Command name and aliases (e.g. `["nickname", "nick"]`).
   - Argument shape: literals (`/cmd clear`) vs. arguments (`/cmd <name>`), and argument types (`ArgumentType::String(StringType::Greedy|Word|Phrase)`, integer, entity, etc.).
   - Whether an executor struct can be shared across subcommands or each needs its own.
   - Permission description text.

## Implementation

1. In the mechanic file, add to `cmds()`:
   ```rust
   fn cmds(&self) -> Vec<Command> {
       let command = Command::new(
           &["{name}".to_string(), "{alias}".to_string()],
           "{description}",
       )
       .then(
           CommandNode::argument("{arg}", &ArgumentType::String(StringType::Greedy))
               .execute({Name}Executor),
       )
       .then(CommandNode::literal("clear").execute({Name}Executor));
       vec![command]
   }
   ```
2. Implement a `struct {Name}Executor;` + `impl CommandHandler for {Name}Executor` below the `impl Mechanic` block:
   ```rust
   impl CommandHandler for {Name}Executor {
       fn handle(
           &self,
           sender: CommandSender,
           _server: Server,
           args: ConsumedArgs,
       ) -> Result<i32, CommandError> {
           let player = sender.as_player().ok_or(CommandError::PermissionDenied)?;
           let (Arg::Simple(value) | Arg::Msg(value)) = args.get_value("{arg}") else {
               // no-arg branch
               return Ok(1);
           };
           sender.send_message(TextComponent::text(&format!("...")));
           Ok(1)
       }
   }
   ```
3. Add the permission in `perms()` using the helper — never hand-format the node:
   ```rust
   fn perms(&self) -> Vec<Permission> {
       vec![default_permission(
           PLUGIN_ID,
           "{name}",
           "Allows using the /{name} command.",
       )]
   }
   ```
   Import `crate::utils::command::default_permission` and `crate::PLUGIN_ID`.

## Important Constraints

- `cmds()` and `perms()` are paired **by index** in `Mechanic::register` — the Nth permission belongs to the Nth command. Keep both vecs in the same order, and keep them aligned when adding/removing entries.
- Reading an argument: `args.get_value("{arg_name}")` returns an enum; destructure with the `(Arg::Simple(x) | Arg::Msg(x))` or-pattern to treat both shapes the same.
- `sender.as_player()` returns `Option<Player>` — commands that require a player should return `CommandError::PermissionDenied` for console senders (existing convention).
- Handler return is `Ok(i32)` — existing modules return `Ok(1)`.
- `use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};`, `pumpkin_plugin_api::command_wit::{Arg, ArgumentType, StringType}`, `pumpkin_plugin_api::commands::CommandHandler`, and `pumpkin_plugin_api::permission::Permission` as needed.

## Docs & Verification

1. Update the module's `//!` header with a `## Commands` table (command, aliases, permission node, description) — copy the table style from `nickname.rs`.
2. Verify:
   - `cargo build --target wasm32-wasip2`
   - `cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings` (warnings are errors)

After finishing, summarize the changes and ask the user if they want to commit.
