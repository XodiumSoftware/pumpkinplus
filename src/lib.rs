mod config;

mod modules {
    pub mod module;
    pub mod enchantments {
        pub mod sample;
    }
    pub mod mechanics {
        pub mod locator;
        pub mod motd;
        pub mod player;
    }
    pub mod recipes {
        pub mod sample;
    }
}

pub use config::*;
pub use enchantments::*;
pub use mechanics::*;
pub use modules::*;
pub use recipes::*;

use crate::player::Player;
use pumpkin_plugin_api::events::EventPriority;
use pumpkin_plugin_api::{Context, PluginMetadata};
use tracing::info;

/// IllyriaPlus plugin implementation.
pub struct IllyriaPlus {}

impl IllyriaPlus {
    fn new() -> Self {
        IllyriaPlus {}
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "IllyriaPlus".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Xodium".into()],
            description: " Minecraft plugin that enhances the base gameplay in Rust".into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        context.register_event_handler(Player::default(), EventPriority::Highest, true)?;
        info!("IllyriaPlus loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("IllyriaPlus unloaded. CYA NEXT TIME!");
        Ok(())
    }
}
