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

use crate::mechanics::locator::Locator;
use crate::mechanics::motd::Motd;
use crate::mechanics::player::Player;
use crate::mechanics::tablist::Tablist;
use crate::module::Module;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use std::time::Instant;
use tracing::info;

pub const PLUGIN_ID: &str = env!("CARGO_PKG_NAME");

/// PumpkinPlus plugin implementation.
pub struct PumpkinPlus {}

impl Plugin for PumpkinPlus {
    fn new() -> Self {
        PumpkinPlus {}
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: PLUGIN_ID.into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: env!("CARGO_PKG_AUTHORS")
                .split(':')
                .map(Into::into)
                .collect(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let mut manager = ConfigManager::empty();

        manager.register::<PlayerConfig>();
        manager.register::<TablistConfig>();
        manager.register::<MotdConfig>();
        manager.register::<LocatorConfig>();

        manager.finalize(&context);

        let player = Player::default();
        let tablist = Tablist::default();
        let motd = Motd::default();
        let locator = Locator::default();
        let modules: Vec<&dyn Module> = vec![&player, &tablist, &motd, &locator];
        let enabled_count = modules.iter().filter(|m| m.enabled()).count();

        let mut total_ms = 0u128;
        for module in modules {
            let start = Instant::now();
            module.register(&context);
            total_ms += start.elapsed().as_millis();
        }

        info!(
            "Registered: {} module(s) | Took {}ms",
            enabled_count, total_ms
        );
        info!("Pumpkin+ loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Pumpkin+ unloaded. CYA NEXT TIME!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PumpkinPlus);
