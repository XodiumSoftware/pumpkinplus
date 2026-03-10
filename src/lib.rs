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
use pumpkin_plugin_api::{Context, PluginMetadata};
use tracing::info;

pub const PLUGIN_ID: &str = "vanillaplus";

/// IllyriaPlus plugin implementation.
pub struct IllyriaPlus {}

impl IllyriaPlus {
    fn new() -> Self {
        IllyriaPlus {}
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Vanilla+".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Xodium".into()],
            description: "Minecraft plugin that enhances the base gameplay in Rust".into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        Motd::default().register(&context);
        Player::default().register(&context);
        info!("IllyriaPlus loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("IllyriaPlus unloaded. CYA NEXT TIME!");
        Ok(())
    }
}
