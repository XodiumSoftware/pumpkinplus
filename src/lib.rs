//! `PumpkinPlus` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in Rust
//! that enhances vanilla gameplay without replacing it.
//!
//! Every feature is modular and toggled via the plugin's JSON config file.
//! For installation, configuration, usage, and troubleshooting, see [`GUIDE.md`](../GUIDE.md).
#![allow(clippy::must_use_candidate)]

mod config;
mod utils {
    pub mod block;
    pub mod command;
    pub mod entity;
    pub mod placeholders;
    pub mod player;
    pub mod text;
}

mod mirror_types {
    pub mod entity_type;
    pub mod gamemode;
    pub mod interaction;
    mod macros;
}

pub use mirror_types::entity_type::EntityType;
pub use mirror_types::gamemode::GameMode;
pub use mirror_types::interaction::InteractAction;

mod modules {
    pub mod recipes {
        pub mod chainmail;
        pub mod diamond_recycle;
        pub mod painting;
        pub mod recipe;
        pub mod rotten_flesh;
        pub mod wood_log;
    }
    pub mod mechanics {
        pub mod mechanic;
        pub mod entity {
            pub mod griefing;
        }
        pub mod world {
            pub mod openable;
        }
        pub mod player {
            pub mod enderchest;
            pub mod messages;
            pub mod nickname;
        }
        pub mod server {
            pub mod chat;
            pub mod tablist;
        }
    }
}

pub use config::*;
pub use modules::*;

pub use modules::mechanics::entity::griefing::GriefingConfig;
pub use modules::mechanics::player::enderchest::EnderchestConfig;
pub use modules::mechanics::player::messages::MessagesConfig;
pub use modules::mechanics::player::nickname::NicknameConfig;
pub use modules::mechanics::server::chat::ChatConfig;
pub use modules::mechanics::server::tablist::TablistConfig;
pub use modules::mechanics::world::openable::OpenableConfig;

use crate::mechanics::entity::griefing::Griefing;
use crate::mechanics::mechanic::Mechanic;
use crate::mechanics::player::enderchest::Enderchest;
use crate::mechanics::player::messages::Messages;
use crate::mechanics::player::nickname::Nickname;
use crate::mechanics::server::chat::Chat;
use crate::mechanics::server::tablist::Tablist;
use crate::mechanics::world::openable::Openable;
use crate::modules::recipes::chainmail::Chainmail;
use crate::modules::recipes::diamond_recycle::DiamondRecycle;
use crate::modules::recipes::painting::Painting;
use crate::modules::recipes::recipe::Recipe;
use crate::modules::recipes::rotten_flesh::RottenFlesh;
use crate::modules::recipes::wood_log::WoodLog;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use std::time::Instant;
use tracing::{error, info};

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
                .split(',')
                .map(std::string::ToString::to_string)
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
        ConfigManager::load(&context);

        let griefing = Griefing;
        let enderchest = Enderchest;
        let nickname = Nickname;
        let messages = Messages;
        let chat = Chat;
        let tablist = Tablist;
        let openable = Openable;
        let modules: Vec<&dyn Mechanic> = vec![
            &griefing,
            &enderchest,
            &nickname,
            &messages,
            &chat,
            &tablist,
            &openable,
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

        // Recipe registration (no config toggles yet — always on)
        let recipes: Vec<&dyn Recipe> = vec![
            &Chainmail,
            &DiamondRecycle,
            &Painting,
            &RottenFlesh,
            &WoodLog,
        ];

        let mut registered_count = 0u32;
        let mut recipe_total_ms = 0u128;
        for recipe in recipes {
            let start = Instant::now();
            match recipe.register(&context) {
                Ok(()) => registered_count += recipe.count(),
                Err(e) => error!("Failed to register recipes: {e}"),
            }
            recipe_total_ms += start.elapsed().as_millis();
        }
        info!("Registered: {registered_count} recipe(s) | Took {recipe_total_ms}ms");
        info!("Pumpkin+ loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Pumpkin+ unloaded. CYA NEXT TIME!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PumpkinPlus);
