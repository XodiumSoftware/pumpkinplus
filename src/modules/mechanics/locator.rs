use crate::module::Module;
use pumpkin::command::args::ConsumedArgs;
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::tree::builder::{argument, literal};
use pumpkin::command::tree::CommandTree;
use pumpkin::command::{CommandExecutor, CommandSender};
use pumpkin::server::Server;
use pumpkin_util::permission::{Permission, PermissionDefault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents handling locator mechanics within the system.
pub struct Locator {
    config: Config,
}

impl Locator {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
}

impl Module for Locator {
    fn enabled(&self) -> bool {
        self.config.enabled
    }

    fn cmds(&self) -> HashSet<CommandTree> {
        HashSet::from([CommandTree::new(
            ["locator", "lc"],
            "Allows players to personalise their locator bar",
        )
            .then(argument("color", ArgumentC).execute(LocatorExecutor))
            .then(argument("hex", ArgumentC).execute(LocatorExecutor))
            .then(literal("reset").execute(LocatorExecutor))])
    }

    fn perms(&self) -> HashSet<Permission> {
        HashSet::from([Permission::new(
            "locator",
            "Allows use of the locator command",
            PermissionDefault::Allow,
        )])
    }
}

struct LocatorExecutor;

impl CommandExecutor for LocatorExecutor {
    fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        Box::pin(async move {
            let arg_0 = self.0;
            let arg_1 = self.1;
            let arg_2 = self.2;
            let player = sender.as_player().unwrap();

            player //TODO

            Ok(())
        })
    }
}

/// Represents the config of the module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { enabled: true }
    }
}
