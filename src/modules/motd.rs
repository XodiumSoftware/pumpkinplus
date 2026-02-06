use crate::modules::module::Module;
use pumpkin::server::Server;
use serde::{Deserialize, Serialize};

/// Represents a module handling motd mechanics within the system.
pub struct MotdModule {
    config: Config,
}

impl MotdModule {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn motd(&self, server: &mut Server) {
        if self.enabled() {
            server.basic_config.motd = self.config.motd.join("\n");
        }
    }
}

impl Module for MotdModule {
    fn enabled(&self) -> bool {
        self.config.enabled
    }
}

/// Represents the config of the module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub motd: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            motd: vec![
                "<gradient:#CB2D3E:#EF473A><b>Ultimate Private SMP</b></gradient>".to_string(),
                "<gradient:#FFE259:#FFA751><b>➤ WELCOME BACK LADS!</b></gradient>".to_string(),
            ],
        }
    }
}
