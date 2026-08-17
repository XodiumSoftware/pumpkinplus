//! Locator module - personalize locator bar color.
//!
//! ## Commands
//!
//! | Command                        | Aliases | Permission                    | Description                |
//! |--------------------------------|---------|-------------------------------|----------------------------|
//! | `/locator <color\|hex\|reset>` | `lc`    | `pumpkinplus:command.locator` | Set locator bar color      |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                   |
//! |-----------|---------|-------------------------------|
//! | `enabled` | `false` | Whether this module is active |

use crate::{PLUGIN_ID, config::ConfigManager, mechanics::mechanic::Mechanic};
use pumpkin_plugin_api::{
    Server,
    command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    permission::{Permission, PermissionDefault},
    text::TextComponent,
};
use serde::{Deserialize, Serialize};

/// Handles locator bar mechanics.
#[derive(Default)]
pub struct Locator;

impl Mechanic for Locator {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.locator.enabled)
    }

    fn cmds(&self) -> Vec<Command> {
        let command = Command::new(
            &["locator".to_string(), "lc".to_string()],
            "Allows players to personalise their locator bar",
        );
        command.then(CommandNode::literal("color").execute(LocatorExecutor));
        command.then(CommandNode::literal("hex").execute(LocatorExecutor));
        command.then(CommandNode::literal("reset").execute(LocatorExecutor));
        vec![command]
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: format!("{PLUGIN_ID}:command.locator"),
            description: "Allows using the /locator and /lc commands.".to_string(),
            default: PermissionDefault::Allow,
            children: vec![],
        }]
    }
}

struct LocatorExecutor;

impl CommandHandler for LocatorExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        // TODO: figure out the api to adjust the locator bar.
        sender.send_message(TextComponent::text("Not yet implemented."));
        Ok(1)
    }
}

/// Configuration for the locator mechanics module.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocatorConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
