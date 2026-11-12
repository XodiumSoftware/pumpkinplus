//! `PumpkinPlus` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in Rust
//! that enhances vanilla gameplay without replacing it.
//!
//! Every feature is modular and toggled via the plugin's JSON config file.
//! For installation, configuration, usage, and troubleshooting, see [`GUIDE.md`](../GUIDE.md).
#![allow(clippy::must_use_candidate)]

mod config;
/// Shared helpers used across modules (blocks, entities, text, placeholders).
mod utils {
    pub(crate) mod block;
    pub(crate) mod entity;
    pub(crate) mod macros;
    pub(crate) mod placeholders;
    pub(crate) mod player;
    pub(crate) mod text;
}

/// Config-friendly mirrors of API enums that cannot be deserialized directly.
mod mirror_types {
    pub(crate) mod entity_type;
    pub(crate) mod gamemode;
    pub(crate) mod hand;
    pub(crate) mod interaction;
    mod macros;
}

pub(crate) use mirror_types::gamemode::GameMode;
pub(crate) use mirror_types::hand::Hand as MirrorHand;
pub(crate) use mirror_types::interaction::InteractAction;

/// All feature modules, grouped by category.
mod modules {
    pub(crate) mod module;
    /// Entity-related mechanics.
    pub(crate) mod entity {
        pub(crate) mod griefing;
        pub(crate) mod silence;
        pub(crate) mod spawn_egg;
        pub(crate) mod tameable;
    }
    /// World/block-related mechanics.
    pub(crate) mod world {
        pub(crate) mod openable;
    }
    /// Player-related mechanics.
    pub(crate) mod player {
        pub(crate) mod anvil;
        pub(crate) mod condense;
        pub(crate) mod enderchest;
        pub(crate) mod head;
        pub(crate) mod locator;
        pub(crate) mod nickname;
        pub(crate) mod xp;
    }
    /// Server-wide mechanics.
    pub(crate) mod server {
        pub(crate) mod chat;
        pub(crate) mod messages;
        pub(crate) mod tablist;
    }
}

pub(crate) use config::ConfigManager;

use crate::modules::entity::griefing::Griefing;
use crate::modules::entity::silence::Silence;
use crate::modules::entity::spawn_egg::SpawnEgg;
use crate::modules::entity::tameable::Tameable;
use crate::modules::module::Module;
use crate::modules::player::anvil::Anvil;
use crate::modules::player::condense::Condense;
use crate::modules::player::enderchest::Enderchest;
use crate::modules::player::head::Head;
use crate::modules::player::locator::Locator;
use crate::modules::player::nickname::Nickname;
use crate::modules::player::xp::Xp;
use crate::modules::server::chat::Chat;
use crate::modules::server::messages::Messages;
use crate::modules::server::tablist::Tablist;
use crate::modules::world::openable::Openable;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::{Plugin, PluginMetadata};
use std::time::Instant;
use tracing::info;

/// The plugin identifier, derived from the crate name.
pub const PLUGIN_ID: &str = env!("CARGO_PKG_NAME");

/// The `PumpkinPlus` plugin entry point.
pub struct PumpkinPlus {}

impl PumpkinPlus {
    /// Registers all modules and their commands/permissions.
    fn register_modules(context: &Context) {
        let modules: Vec<&dyn Module> = vec![
            &Griefing,
            &Silence,
            &SpawnEgg,
            &Tameable,
            &Anvil,
            &Condense,
            &Enderchest,
            &Head,
            &Locator,
            &Nickname,
            &Messages,
            &Xp,
            &Chat,
            &Tablist,
            &Openable,
        ];
        let enabled_modules = modules.iter().filter(|m| m.enabled()).count();

        let mut module_total_ms = 0u128;
        for module in modules {
            let start = Instant::now();
            module.register(context);
            module_total_ms += start.elapsed().as_millis();
        }

        info!(
            "Registered: {} module(s) | Took {}ms",
            enabled_modules, module_total_ms
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
                .map(ToString::to_string)
                .collect(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            dependencies: vec![],
            permissions: vec![
                pumpkin_plugin_api::permissions::FS_READ_DATA.into(),
                pumpkin_plugin_api::permissions::FS_WRITE_DATA.into(),
            ],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        ConfigManager::load(&context);

        Self::register_modules(&context);

        info!("Pumpkin+ loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Pumpkin+ unloaded. CYA NEXT TIME!");
        Ok(())
    }
}

/// The plugin entry point the host calls by export name.
///
/// The body is a macro expansion, so the generated `__init_plugin` cannot carry
/// its own rustdoc. The expectation is scoped to this module rather than the
/// invocation, because an attribute on a macro call does not reach what it
/// expands to.
///
/// The module is `pub` because the expansion defines `pub extern "C" fn
/// __init_plugin`, and the host calls it by export name.
#[expect(
    missing_docs,
    reason = "the generated __init_plugin is documented by the macro, not here"
)]
pub mod entry {
    use super::PumpkinPlus;

    pumpkin_plugin_api::register_plugin!(PumpkinPlus);
}
