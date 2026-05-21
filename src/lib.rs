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
mod mirror_types {
    pub mod entity_type;
    pub mod gamemode;
    pub mod interaction;
}

pub use mirror_types::entity_type::EntityType;
pub use mirror_types::gamemode::GameMode;
pub use mirror_types::interaction::InteractionAction;

mod modules {
    pub mod module;
    pub mod mechanics {
        pub mod entity {
            pub mod bat;
            pub mod griefing;
            pub mod husk;
        }
        pub mod world {
            pub mod openable;
        }
        pub mod player {
            pub mod enderchest;
            pub mod locator;
            pub mod messages;
        }
        pub mod server {
            pub mod chat;
            pub mod tablist;
        }
    }
}

pub use config::*;
pub use modules::*;

pub use modules::mechanics::entity::bat::BatConfig;
pub use modules::mechanics::entity::griefing::GriefingConfig;
pub use modules::mechanics::entity::husk::HuskConfig;
pub use modules::mechanics::player::enderchest::EnderchestConfig;
pub use modules::mechanics::player::messages::MessagesConfig;
pub use modules::mechanics::server::chat::ChatConfig;
pub use modules::mechanics::server::tablist::TablistConfig;
pub use modules::mechanics::world::openable::OpenableConfig;

// use crate::mechanics::entity::bat::Bat;
// use crate::mechanics::entity::griefing::Griefing;
// use crate::mechanics::player::enderchest::Enderchest;
// use crate::mechanics::player::locator::Locator;
use crate::mechanics::player::messages::Messages;
// use crate::mechanics::world::openable::Openable;
use crate::mechanics::server::chat::Chat;
use crate::mechanics::server::tablist::Tablist;
use crate::module::Module;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use std::time::Instant;
use tracing::info;

pub const PLUGIN_ID: &str = env!("CARGO_PKG_NAME");

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

        // manager.register::<BatConfig>();
        // manager.register::<GriefingConfig>();
        // manager.register::<HuskConfig>();
        // manager.register::<EnderchestConfig>();
        // manager.register::<LocatorConfig>();
        manager.register::<MessagesConfig>();
        manager.register::<ChatConfig>();
        manager.register::<TablistConfig>();
        // manager.register::<OpenableConfig>();

        manager.finalize(&context);

        // let bat = Bat;
        // let griefing = Griefing;
        // let husk = Husk;
        // let enderchest = Enderchest;
        // let locator = Locator;
        let messages = Messages;
        let chat = Chat;
        let tablist = Tablist;
        // let openable = Openable;
        let modules: Vec<&dyn Module> = vec![
            // &bat,
            // &griefing,
            // &husk,
            // &enderchest,
            // &locator,
            &messages, &chat, &tablist,
            // &openable,
        ];
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
