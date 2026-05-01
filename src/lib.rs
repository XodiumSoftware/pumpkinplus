//! PumpkinPlus is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in Rust
//! that enhances the vanilla gameplay without replacing it.
//!
//! Every feature is modular and toggled via a JSON config file.
//!
//! ## Features
//!
//! | Category    | What it adds                                    |
//! |-------------|-------------------------------------------------|
//! | **Player**  | Custom join, leave, and kick messages           |
//! | **Chat**    | Chat formatting and word filtering                |
//! | **Tablist** | Dynamic tab list header/footer with placeholders |
//! | **Locator** | Personalize locator bar color (`/locator`)        |
//!
//! ## Installation
//!
//! 1. Download the latest `pumpkinplus.wasm` from
//!    [GitHub Releases](https://github.com/XodiumSoftware/PumpkinPlus/releases).
//! 2. Drop it into your Pumpkin server's `plugins/` folder.
//! 3. Start (or restart) the server.
//!
//! On first start, a `config.json` file is created in the plugin's data folder with all defaults.
//! Edit it and restart to apply changes.
//!
//! ## Building
//!
//! ```bash
//! cargo build --release --target wasm32-wasip2
//! ```
//!
//! The output is at `target/wasm32-wasip2/release/pumpkinplus.wasm`.
//!
//! ## Viewing Documentation
//!
//! ```bash
//! cargo doc --open
//! ```
//!
//! # Configuration
//!
//! All settings live in `config.json` in the plugin's data folder.
//! Each top-level key corresponds to one module.
//!
//! ## Placeholders
//!
//! String fields that are displayed as in-game messages support placeholders:
//!
//! | Placeholder | Replaced with              |
//! |-------------|----------------------------|
//! | `{player}`  | The player's in-game name  |
//! | `{online}`  | Number of online players   |
//! | `{tps}`     | Server TPS                 |
//! | `{mspt}`    | Milliseconds per tick      |
//! | `{message}` | The original chat message  |

mod config;

mod modules {
    pub mod module;
    pub mod mechanics {
        pub mod double_doors;
        pub mod locator;
        pub mod player;
        pub mod tablist;
    }
}

pub use config::*;
pub use modules::*;

pub use modules::mechanics::double_doors::Config as DoubleDoorsConfig;
pub use modules::mechanics::locator::Config as LocatorConfig;
pub use modules::mechanics::player::Config as PlayerConfig;
pub use modules::mechanics::tablist::Config as TablistConfig;

use crate::mechanics::tablist::Tablist;
use crate::mechanics::{double_doors::DoubleDoors, player::Player};
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
            dependencies: vec![],
            permissions: vec![
                pumpkin_plugin_api::permissions::FS_READ_DATA.into(),
                pumpkin_plugin_api::permissions::FS_WRITE_DATA.into(),
            ],
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let mut manager = ConfigManager::empty();

        manager.register::<PlayerConfig>();
        manager.register::<TablistConfig>();
        manager.register::<DoubleDoorsConfig>();
        //manager.register::<LocatorConfig>();

        manager.finalize(&context);

        let player = Player {};
        let tablist = Tablist;
        //let locator = Locator;
        let double_doors = DoubleDoors;
        let modules: Vec<&dyn Module> = vec![&player, &tablist, &double_doors];
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
