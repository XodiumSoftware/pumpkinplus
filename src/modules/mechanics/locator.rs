use crate::{PLUGIN_ID, config::ConfigManager, module::Module};
use pumpkin_plugin_api::{
    Server,
    command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Handles locator bar mechanics.
#[derive(Default)]
pub struct Locator;

impl Module for Locator {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.locator_module.enabled)
            .unwrap_or(true)
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

    fn perms(&self) -> HashSet<String> {
        HashSet::from([format!("{}:command.locator", PLUGIN_ID)])
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
pub type LocatorConfig = Config;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
}
