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
    pub mod macros;
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

pub use mirror_types::gamemode::GameMode;
pub use mirror_types::interaction::InteractAction;

mod modules {
    pub mod enchantments {
        pub mod enchantment;
        pub mod utility {
            pub mod embertread;
            pub mod nimbus;
            pub mod tether;
            pub mod vinemine;
        }
        pub mod vanilla {
            pub mod feather_falling;
            pub mod fortune;
            pub mod silk_touch;
        }
    }
    pub mod recipes {
        pub mod recipe;
        pub mod vanilla {
            pub mod chainmail;
            pub mod diamond_recycle;
            pub mod ice_breakdown;
            pub mod nether_wart_block;
            pub mod painting;
            pub mod rotten_flesh;
            pub mod wood_log;
            pub mod wool_to_string;
        }
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

pub use modules::enchantments::enchantment::EnchantmentsConfig;
pub use modules::mechanics::entity::griefing::GriefingConfig;
pub use modules::mechanics::player::enderchest::EnderchestConfig;
pub use modules::mechanics::player::messages::MessagesConfig;
pub use modules::mechanics::player::nickname::NicknameConfig;
pub use modules::mechanics::server::chat::ChatConfig;
pub use modules::mechanics::server::tablist::TablistConfig;
pub use modules::mechanics::world::openable::OpenableConfig;
pub use modules::recipes::recipe::RecipesConfig;

use crate::mechanics::entity::griefing::Griefing;
use crate::mechanics::mechanic::Mechanic;
use crate::mechanics::player::enderchest::Enderchest;
use crate::mechanics::player::messages::Messages;
use crate::mechanics::player::nickname::Nickname;
use crate::mechanics::server::chat::Chat;
use crate::mechanics::server::tablist::Tablist;
use crate::mechanics::world::openable::Openable;
use crate::modules::enchantments::enchantment::Enchantment;
use crate::modules::enchantments::utility::embertread::Embertread;
use crate::modules::enchantments::vanilla::fortune::Fortune;
use crate::modules::recipes::recipe::Recipe;
use crate::modules::recipes::vanilla::chainmail::Chainmail;
use crate::modules::recipes::vanilla::diamond_recycle::DiamondRecycle;
use crate::modules::recipes::vanilla::ice_breakdown::IceBreakdown;
use crate::modules::recipes::vanilla::nether_wart_block::NetherWartBlock;
use crate::modules::recipes::vanilla::painting::Painting;
use crate::modules::recipes::vanilla::rotten_flesh::RottenFlesh;
use crate::modules::recipes::vanilla::wood_log::WoodLog;
use crate::modules::recipes::vanilla::wool_to_string::WoolToString;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use std::time::Instant;
use tracing::info;

pub const PLUGIN_ID: &str = env!("CARGO_PKG_NAME");

pub struct PumpkinPlus {}

impl PumpkinPlus {
    /// Registers all mechanics and their commands/permissions.
    fn register_mechanics(context: &Context) {
        let mechanics: Vec<&dyn Mechanic> = vec![
            &Griefing,
            &Enderchest,
            &Nickname,
            &Messages,
            &Chat,
            &Tablist,
            &Openable,
        ];
        let enabled_mechanics = mechanics.iter().filter(|m| m.enabled()).count();

        let mut mechanic_total_ms = 0u128;
        for mechanic in mechanics {
            let start = Instant::now();
            mechanic.register(context);
            mechanic_total_ms += start.elapsed().as_millis();
        }

        info!(
            "Registered: {} mechanic(s) | Took {}ms",
            enabled_mechanics, mechanic_total_ms
        );
    }

    /// Registers all recipe packs.
    fn register_recipes(context: &Context) {
        let recipes: Vec<&dyn Recipe> = vec![
            &Chainmail,
            &DiamondRecycle,
            &IceBreakdown,
            &NetherWartBlock,
            &Painting,
            &RottenFlesh,
            &WoodLog,
            &WoolToString,
        ];

        let enabled_recipes = recipes.iter().filter(|r| r.enabled()).count();
        let mut recipe_total_ms = 0u128;
        for recipe in recipes {
            let start = Instant::now();
            recipe.register(context);
            recipe_total_ms += start.elapsed().as_millis();
        }
        info!(
            "Registered: {} recipe pack(s) | Took {}ms",
            enabled_recipes, recipe_total_ms
        );
    }

    /// Registers all enchantments (custom definitions and behavior overrides).
    fn register_enchantments(context: &Context) {
        let enchantments: Vec<&dyn Enchantment> = vec![&Embertread, &Fortune];

        let enabled_enchantments = enchantments.iter().filter(|e| e.enabled()).count();
        let mut enchantment_total_ms = 0u128;
        for enchantment in enchantments {
            let start = Instant::now();
            enchantment.register(context);
            enchantment_total_ms += start.elapsed().as_millis();
        }
        info!(
            "Registered: {} enchantment pack(s) | Took {}ms",
            enabled_enchantments, enchantment_total_ms
        );
    }
}

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

        Self::register_mechanics(&context);
        Self::register_recipes(&context);
        Self::register_enchantments(&context);

        info!("Pumpkin+ loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Pumpkin+ unloaded. CYA NEXT TIME!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PumpkinPlus);
