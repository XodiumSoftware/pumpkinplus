mod config;

mod modules {
    pub mod module;
    pub mod mechanics {
        pub mod locator;
        pub mod motd;
        pub mod player;
        pub mod tablist;
    }
}

pub use config::*;
pub use modules::*;

pub use modules::mechanics::locator::LocatorConfig;
pub use modules::mechanics::motd::MotdConfig;
pub use modules::mechanics::player::PlayerConfig;
pub use modules::mechanics::tablist::TablistConfig;

use crate::mechanics::player::Player;
use crate::module::Module;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::info;

pub const PLUGIN_ID: &str = "pumpkinplus";

/// PumpkinPlus plugin implementation.
pub struct PumpkinPlus {}

impl Plugin for PumpkinPlus {
    fn new() -> Self {
        PumpkinPlus {}
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "pumpkinplus".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Xodium".into()],
            description: "Minecraft Pumpkin plugin that enhances the base gameplay".into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        ConfigManager::new(&context);
        Player::default().register(&context);
        info!("Pumpkin+ loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Pumpkin+ unloaded. CYA NEXT TIME!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PumpkinPlus);
