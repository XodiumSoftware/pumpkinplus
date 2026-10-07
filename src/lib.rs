//! `PumpkinPlus` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in Rust
//! that enhances vanilla gameplay without replacing it.
//!
//! Every feature is modular and toggled via the plugin's JSON config file.
//! For installation, configuration, usage, and troubleshooting, see [`GUIDE.md`](../GUIDE.md).
#![allow(clippy::must_use_candidate)]

mod config;
pub mod guis {
    pub mod admin;
    pub mod gui;
    pub mod registry;
}
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

    pub mod recipes;
    pub mod mechanics {
        pub mod mechanic;
        pub mod entity {
            pub mod griefing;
            pub mod silence;
            pub mod spawn_egg;
            pub mod tameable;
        }
        pub mod world {
            pub mod openable;
        }
        pub mod player {
            pub mod anvil;
            pub mod condense;
            pub mod enderchest;
            pub mod head;
            pub mod locator;
            pub mod messages;
            pub mod nickname;
            pub mod xp;
        }
        pub mod server {
            pub mod chat;
            pub mod rules;
            pub mod tablist;
        }
    }
}

pub use config::*;
pub use guis::gui::GuiBuilder;
pub use modules::*;

pub use modules::enchantments::enchantment::EnchantmentsConfig;

pub use modules::mechanics::entity::griefing::GriefingConfig;
pub use modules::mechanics::entity::silence::SilenceConfig;
pub use modules::mechanics::entity::spawn_egg::SpawnEggConfig;
pub use modules::mechanics::entity::tameable::TameableConfig;
pub use modules::mechanics::player::anvil::AnvilConfig;
pub use modules::mechanics::player::condense::CondenseConfig;
pub use modules::mechanics::player::enderchest::EnderchestConfig;
pub use modules::mechanics::player::head::HeadConfig;
pub use modules::mechanics::player::locator::LocatorConfig;
pub use modules::mechanics::player::messages::MessagesConfig;
pub use modules::mechanics::player::nickname::NicknameConfig;
pub use modules::mechanics::player::xp::XpConfig;
pub use modules::mechanics::server::chat::ChatConfig;
pub use modules::mechanics::server::rules::RulesConfig;
pub use modules::mechanics::server::tablist::TablistConfig;
pub use modules::mechanics::world::openable::OpenableConfig;
pub use modules::recipes::RecipesConfig;

use crate::mechanics::entity::griefing::Griefing;
use crate::mechanics::entity::silence::Silence;
use crate::mechanics::entity::spawn_egg::SpawnEgg;
use crate::mechanics::entity::tameable::Tameable;
use crate::mechanics::mechanic::Mechanic;
use crate::mechanics::player::anvil::Anvil;
use crate::mechanics::player::condense::Condense;
use crate::mechanics::player::enderchest::Enderchest;
use crate::mechanics::player::head::Head;
use crate::mechanics::player::locator::Locator;
use crate::mechanics::player::messages::Messages;
use crate::mechanics::player::nickname::Nickname;
use crate::mechanics::player::xp::Xp;
use crate::mechanics::server::chat::Chat;
use crate::mechanics::server::rules::Rules;
use crate::mechanics::server::tablist::Tablist;
use crate::mechanics::world::openable::Openable;
use crate::modules::enchantments::enchantment::Enchantment;
use crate::modules::enchantments::utility::embertread::Embertread;
use crate::modules::enchantments::vanilla::fortune::Fortune;
use pumpkin_plugin_api::command::{Command, CommandError, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, InventoryClickEvent, InventoryCloseEvent,
};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, Server};
use std::time::Instant;
use tracing::{error, info};

pub const PLUGIN_ID: &str = env!("CARGO_PKG_NAME");

pub struct PumpkinPlus {}

impl PumpkinPlus {
    /// Registers all mechanics and their commands/permissions.
    fn register_mechanics(context: &Context) {
        let mechanics: Vec<&dyn Mechanic> = vec![
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
            &Rules,
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

    /// Registers all config-defined recipes.
    fn register_recipes(context: &Context) {
        let start = Instant::now();
        crate::modules::recipes::register_all(context);
        let total_ms = start.elapsed().as_millis();
        info!("Recipe registration took {total_ms}ms");
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
            "Registered: {} enchantment(s) | Took {}ms",
            enabled_enchantments, enchantment_total_ms
        );
    }

    /// Registers the admin `/pumpkinplus` command, which opens the main menu GUI.
    fn register_admin(context: &Context) {
        let permission = Permission {
            node: format!("{PLUGIN_ID}:command.{PLUGIN_ID}"),
            description: format!("Opens the {PLUGIN_ID} admin menu."),
            default: PermissionDefault::Op(PermissionLevel::Four),
            children: vec![],
        };
        if let Err(e) = context.register_permission(&permission) {
            error!("Failed to register permission '{}': {e}", permission.node);
        }

        let command = Command::new(
            &[PLUGIN_ID.to_string(), "pp".to_string()],
            "Open the admin menu",
        )
        .execute(AdminExecutor);
        context.register_command(command, &permission.node);
    }

    /// Registers global GUI event handlers for click dispatch and session cleanup.
    fn register_gui_handlers(context: &Context) {
        context
            .register_event_handler::<InventoryClickEvent, _>(
                GuiClickDispatcher,
                EventPriority::Highest,
                true,
            )
            .expect("failed to register InventoryClickEvent handler");
        context
            .register_event_handler::<InventoryCloseEvent, _>(
                GuiCloseDispatcher,
                EventPriority::Highest,
                true,
            )
            .expect("failed to register InventoryCloseEvent handler");
    }
}

/// Command executor that opens the main admin menu GUI.
struct AdminExecutor;

impl CommandHandler for AdminExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.as_player().ok_or(CommandError::PermissionDenied)?;
        let gui = crate::guis::admin::build_admin_menu();
        // Track the session so clicks route back to the right handlers.
        crate::guis::registry::open(&player, crate::guis::admin::ADMIN_MENU_ID);
        player.open_gui(gui);
        Ok(1)
    }
}

/// Dispatches `InventoryClickEvent`s to slot handlers in the GUI registry.
struct GuiClickDispatcher;

impl EventHandler<InventoryClickEvent> for GuiClickDispatcher {
    fn handle(
        &self,
        _server: Server,
        event: EventData<InventoryClickEvent>,
    ) -> EventData<InventoryClickEvent> {
        crate::guis::registry::dispatch_click(event)
    }
}

/// Clears GUI sessions when a player closes an inventory.
struct GuiCloseDispatcher;

impl EventHandler<InventoryCloseEvent> for GuiCloseDispatcher {
    fn handle(
        &self,
        _server: Server,
        event: EventData<InventoryCloseEvent>,
    ) -> EventData<InventoryCloseEvent> {
        crate::guis::registry::dispatch_close(event)
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

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let config = ConfigManager::load(&context);

        Self::register_admin(&context);
        Self::register_gui_handlers(&context);

        if config.modules.mechanics {
            Self::register_mechanics(&context);
        }
        if config.modules.recipes {
            Self::register_recipes(&context);
        }
        if config.modules.enchantments {
            Self::register_enchantments(&context);
        }

        info!("Pumpkin+ loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Pumpkin+ unloaded. CYA NEXT TIME!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PumpkinPlus);
