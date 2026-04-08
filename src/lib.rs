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

pub use modules::mechanics::locator::Config as LocatorConfig;
pub use modules::mechanics::motd::Config as MotdConfig;
pub use modules::mechanics::player::Config as PlayerConfig;
pub use modules::mechanics::tablist::Config as TablistConfig;

use crate::mechanics::player::Player;
use crate::module::Module;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::info;

pub const PLUGIN_ID: &str = env!("CARGO_PKG_NAME");

/// Module names for config lookup
pub mod module_names {
    pub const PLAYER: &str = "player";
    pub const TABLIST: &str = "tablist";
    pub const MOTD: &str = "motd";
    pub const LOCATOR: &str = "locator";
}

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
        let mut manager = ConfigManager::empty();

        manager.register::<PlayerConfig>(module_names::PLAYER);
        manager.register::<TablistConfig>(module_names::TABLIST);
        manager.register::<MotdConfig>(module_names::MOTD);
        manager.register::<LocatorConfig>(module_names::LOCATOR);

        manager.finalize(&context);

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
