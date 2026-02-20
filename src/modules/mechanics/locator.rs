use crate::module::Module;
use pumpkin::command::args::simple::SimpleArgConsumer;
use pumpkin::command::args::{Arg, ConsumedArgs};
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::dispatcher::CommandError::CommandFailed;
use pumpkin::command::tree::CommandTree;
use pumpkin::command::tree::builder::{argument, literal};
use pumpkin::command::{CommandExecutor, CommandSender};
use pumpkin::server::Server;
use pumpkin_util::permission::{Permission, PermissionDefault};
use pumpkin_util::text::TextComponent;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::pin::Pin;

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
        .then(argument("color", SimpleArgConsumer).execute(LocatorExecutor))
        .then(argument("hex", SimpleArgConsumer).execute(LocatorExecutor))
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
        args: &ConsumedArgs<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + 'a>> {
        Box::pin(async move {
            let player = match sender.as_player() {
                Some(p) => p,
                None => {
                    return Err(CommandFailed(TextComponent::text(
                        "Only players can use this command",
                    )));
                }
            };

            // TODO: figure out the api to adjust the locator bar.

            if let Some(Arg::Simple(value)) = args.get("color") {
                let color: &str = value;
                player;
            }
            if let Some(Arg::Simple(value)) = args.get("hex") {
                let hex: &str = value;
                player;
            }
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
