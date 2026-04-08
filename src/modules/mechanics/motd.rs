use crate::config::ConfigManager;
use crate::modules::module::Module;
use pumpkin_plugin_api::Server;
use serde::{Deserialize, Serialize};

/// Handles MOTD (Message of the Day) mechanics.
#[derive(Default)]
pub struct Motd;

impl Module for Motd {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.motd_module.enabled)
            .unwrap_or(true)
    }
}

impl Motd {
    pub fn motd(&self, _server: &mut Server) {
        let config = ConfigManager::get()
            .map(|cm| cm.motd_module)
            .unwrap_or_default();
        if !self.enabled() || config.motd.is_empty() {
            return;
        }
        todo!("api is not ready yet")
        //server.set_motd(config.motd.join("\n"));
    }
}

/// Configuration for the MOTD mechanics module.
pub type MotdConfig = Config;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
    /// Lines of the MOTD displayed to players on the server list.
    pub motd: Vec<String>,
}
