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
pub use mechanics::*;
pub use modules::*;

use crate::module::Module;
use crate::motd::Motd;
use crate::player::Player;
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
            name: "Pumpkin+".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Xodium".into()],
            description: "Minecraft plugin that enhances the base gameplay in Rust".into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        ConfigManager::new(&context).map_err(|e| e.to_string())?;
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
