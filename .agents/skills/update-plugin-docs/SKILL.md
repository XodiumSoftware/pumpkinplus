---
name: update-plugin-docs
description: Updates PumpkinPlus rustdoc comments, module doc tables, GUIDE.md, and regenerates docs after code changes.
---

# Update Plugin Docs

Use this skill after code changes in PumpkinPlus, or when the user asks to sync or regenerate documentation.

## Rules (from `AGENTS.md`)

If a code change would confuse someone reading the docs, update the docs. Trigger points:

- New modules, commands, config fields, or placeholders → rustdoc module headers (`//!` tables)
- Changed config fields or behavior → the owning module's `## Configuration` table
- New items → the "Available Items" table in `src/modules/items/item.rs`
- New/changed build steps or features → `GUIDE.md` and `README.md` if relevant

## Conventions

- Module-level docs use `//!` with Markdown tables for config: `| Field | Default | Description |`.
- Document all `pub` items with `///`. Config struct fields get inline `///` comments.
- User-facing strings document their placeholders (`{player}`, `{online}`, `{tps}`, `{mspt}`, `{message}`) in a `## Placeholders` table.
- Commands are documented in a `## Commands` table: command, aliases, permission node, description.
- The `{PLUGIN_ID}` in docs is the crate name (`pumpkinplus`) — don't hardcode differently.

## Steps

1. Identify changed modules (diff or ask the user).
2. Update `//!` headers and `///` comments in the changed files to match current behavior, config fields, commands, and placeholders.
3. Check `src/lib.rs` crate docs still describe the plugin accurately.
4. If `GUIDE.md` mentions affected features, commands, or config keys, update those sections.
5. Regenerate docs and confirm they build:
   - `cargo doc --no-deps --target wasm32-wasip2`
   - Fix any broken doc links or rustdoc warnings it reports.
6. Also confirm lints still pass if doc examples or imports moved:
   - `cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings`

After finishing, list the files updated and ask the user if they want to commit.
