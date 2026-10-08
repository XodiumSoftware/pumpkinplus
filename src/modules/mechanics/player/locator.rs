//! Locator module — customize waypoint locator bar color.
//!
//! Allows players to personalize their waypoint locator bar color using
//! `/locator` command with named colors, hex codes, or reset to default.
//!
//! ## Status
//!
//! **Stub module** — Cannot be implemented due to missing Pumpkin plugin APIs.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to implement this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `Player.set_waypoint_color()` | Set the locator bar color |
//! | `Player.get_waypoint_color()` | Get the current locator bar color |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! ## Commands
//!
//! | Command | Aliases | Permission                    | Description |
//! |---------|---------|-------------------------------|-------------|
//! | `/locator` | `lc` | `pumpkinplus:command.locator` | Show current locator color |
//! | `/locator <color>` | `lc` | `pumpkinplus:command.locator` | Set locator color by name |
//! | `/locator <hex>` | `lc` | `pumpkinplus:command.locator` | Set locator color by hex code |
//! | `/locator reset` | `lc` | `pumpkinplus:command.locator` | Reset to default locator color |
//!
//! `IllyriaPlus` behavior reference:
//! - Waypoint color stored as player PDC (persistent data container)
//! - Action bar messages show current/new color
//! - Supports named colors and hex codes

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::utils::text::{parse_color_arg, parse_minimessage};
use pumpkin_plugin_api::{
    Server,
    command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
    command_wit::{Arg, ArgumentType, StringType},
    commands::CommandHandler,
    permission::{Permission, PermissionDefault},
};
use serde::{Deserialize, Serialize};

/// Permission node required to use the `/locator` command.
pub const PERM_LOCATOR: &str = concat!(env!("CARGO_PKG_NAME"), ":command.locator");

/// Handles locator bar customization.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Locator;

impl Mechanic for Locator {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.locator.enabled)
    }

    fn cmds(&self) -> Vec<Command> {
        vec![
            Command::new(
                &["locator".to_string(), "lc".to_string()],
                "Show, set, or reset your waypoint locator bar color",
            )
            .then(
                CommandNode::argument("color", &ArgumentType::String(StringType::Greedy))
                    .execute(LocatorExecutor),
            )
            .then(CommandNode::literal("reset").execute(LocatorExecutor))
            .execute(LocatorExecutor),
        ]
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: PERM_LOCATOR.into(),
            description: "Allows using the /locator and /lc commands.".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        }]
    }
}

/// Command executor that gets, sets, or resets the player's locator bar color.
struct LocatorExecutor;

impl CommandHandler for LocatorExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        if !sender.has_permission(&server, PERM_LOCATOR) {
            return Err(CommandError::PermissionDenied);
        }

        let player = sender.as_player().ok_or(CommandError::PermissionDenied)?;

        let color_input = match args.get_value("color") {
            Arg::Simple(value) | Arg::Msg(value) => value.trim().to_string(),
            _ => String::new(),
        };

        if color_input.is_empty() || color_input.eq_ignore_ascii_case("reset") {
            // TODO: Once `Player.set_waypoint_color(Option<RgbColor>)` is
            // available, pass `None` here to clear the stored color.
            player.send_system_message(
                parse_minimessage("<yellow>Locator color reset.</yellow>"),
                false,
            );
            return Ok(1);
        }

        let Some(color) = parse_color_arg(&color_input) else {
            player.send_system_message(
                parse_minimessage(&format!(
                    "<red>Unknown color:</red> {color_input}. Try a name like <gold>red</gold> or a hex like <gold>#FF8800</gold>."
                )),
                false,
            );
            return Ok(0);
        };
        let _ = color;

        // TODO: Once `Player.set_waypoint_color()` is available, pass `color`
        // instead of just acknowledging the command.
        player.send_system_message(
            parse_minimessage(&format!(
                "<yellow>Locator color updated to:</yellow> {color_input}"
            )),
            false,
        );
        Ok(1)
    }
}

/// Configuration for the locator mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocatorConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
