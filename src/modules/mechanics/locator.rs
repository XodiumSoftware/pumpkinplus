use crate::module::Module;
use pumpkin::command::tree::CommandTree;
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
        )])
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
