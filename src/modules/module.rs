//! Module system for `PumpkinPlus`.
//!
//! Each gameplay feature lives under `modules/` as a module implementing the
//! [`Module`] trait. Modules can register event handlers, commands, and
//! permission nodes.

pub(crate) use crate::modules::entity::griefing::GriefingConfig;
pub(crate) use crate::modules::entity::silence::SilenceConfig;
pub(crate) use crate::modules::entity::spawn_egg::SpawnEggConfig;
pub(crate) use crate::modules::entity::tameable::TameableConfig;
pub(crate) use crate::modules::player::anvil::AnvilConfig;
pub(crate) use crate::modules::player::condense::CondenseConfig;
pub(crate) use crate::modules::player::enderchest::EnderchestConfig;
pub(crate) use crate::modules::player::head::HeadConfig;
pub(crate) use crate::modules::player::locator::LocatorConfig;
pub(crate) use crate::modules::player::nickname::NicknameConfig;
pub(crate) use crate::modules::player::xp::XpConfig;
pub(crate) use crate::modules::server::chat::ChatConfig;
pub(crate) use crate::modules::server::messages::MessagesConfig;
pub(crate) use crate::modules::server::tablist::TablistConfig;
pub(crate) use crate::modules::world::openable::OpenableConfig;
pub(crate) use crate::modules::world::spawnprotection::SpawnProtectionConfig;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use pumpkin_plugin_api::permission::Permission;
use serde::{Deserialize, Serialize};
use tracing::error;

/// A trait representing a plugin module that can be enabled or disabled.
///
/// Modules may optionally expose commands, permission nodes, and event handlers,
/// all registered with the server via [`Module::register`].
pub(crate) trait Module {
    /// Returns `true` if the module is enabled, `false` otherwise.
    fn enabled(&self) -> bool;

    /// Returns the commands provided by this module.
    ///
    /// Each [`Command`] returned here will be registered with the server when
    /// [`Module::register`] is called. Returns an empty vec by default.
    fn cmds(&self) -> Vec<Command> {
        Vec::new()
    }

    /// Returns the permission nodes required by this module.
    ///
    /// All permissions returned here are registered with the server so permission
    /// plugins can grant/deny them. Enforcement happens inside each command's
    /// [`CommandHandler`] via `sender.has_permission(server, node)`.
    /// Returns an empty vec by default.
    fn perms(&self) -> Vec<Permission> {
        Vec::new()
    }

    /// Registers event handlers for this module.
    ///
    /// Override this to call [`Module::register_event`] for each event this
    /// module handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers `Self` as the handler for event `T` and panics on failure.
    ///
    /// This is a thin wrapper around [`Context::register_event_handler`] that
    /// supplies the module-specific error message so individual modules don't
    /// have to repeat it.
    fn register_event<T>(&self, context: &Context, priority: EventPriority, ignore_cancelled: bool)
    where
        T: FromIntoEvent + Send + Sync + 'static,
        Self: EventHandler<T> + Default + Send + Sync + 'static,
    {
        context
            .register_event_handler::<T, _>(Self::default(), priority, ignore_cancelled)
            .expect("failed to register event handler");
    }

    /// Registers this module's event handlers and commands with the server.
    ///
    /// Calls [`Module::events`](Module::events), registers each permission node
    /// from [`Module::perms`] with the server, then registers each command from
    /// [`Module::cmds`]. Commands are registered without a server-side permission
    /// gate; they self-enforce their permissions inside [`CommandHandler::handle`]
    /// via `sender.has_permission(server, node)`.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        self.events(context);
        for perm in self.perms() {
            if let Err(e) = context.register_permission(&perm) {
                error!("Failed to register permission '{}': {e}", perm.node);
            }
        }
        for cmd in self.cmds() {
            context.register_command(cmd, "");
        }
    }
}

/// Top-level configuration for all modules.
///
/// Each field toggles one module. All modules are disabled by default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct MechanicsConfig {
    /// Custom anvil operations (disenchant, cost bypass).
    pub anvil: AnvilConfig,
    /// Item condensing commands.
    pub condense: CondenseConfig,
    /// Mob griefing prevention.
    pub griefing: GriefingConfig,
    /// Mob silencing with amethyst shards.
    pub silence: SilenceConfig,
    /// Spawn protection: keeps monsters out of the spawn area.
    pub spawnprotection: SpawnProtectionConfig,
    /// Spawn egg drops from mob deaths.
    pub spawn_egg: SpawnEggConfig,
    /// Pet ownership transfer between players.
    pub tameable: TameableConfig,
    /// Shared enderchest mechanics.
    pub enderchest: EnderchestConfig,
    /// Player head drops on death.
    pub head: HeadConfig,
    /// Locator bar color customization.
    pub locator: LocatorConfig,
    /// Custom join/leave/kick messages.
    pub messages: MessagesConfig,
    /// Player nickname commands.
    pub nickname: NicknameConfig,
    /// XP to bottle conversion.
    pub xp: XpConfig,
    /// Chat formatting and filtering.
    pub chat: ChatConfig,
    /// Tab list header/footer.
    pub tablist: TablistConfig,
    /// Double-door synchronization.
    pub openable: OpenableConfig,
}
