//! Mechanic system for `PumpkinPlus`.
//!
//! Each gameplay feature is implemented as a module implementing the [`Mechanic`] trait.
//! Mechanics can register event handlers, commands, and permission nodes.

pub use crate::modules::mechanics::entity::griefing::GriefingConfig;
pub use crate::modules::mechanics::entity::silence::SilenceConfig;
pub use crate::modules::mechanics::entity::spawn_egg::SpawnEggConfig;
pub use crate::modules::mechanics::entity::tameable::TameableConfig;
pub use crate::modules::mechanics::player::anvil::AnvilConfig;
pub use crate::modules::mechanics::player::condense::CondenseConfig;
pub use crate::modules::mechanics::player::enderchest::EnderchestConfig;
pub use crate::modules::mechanics::player::head::HeadConfig;
pub use crate::modules::mechanics::player::locator::LocatorConfig;
pub use crate::modules::mechanics::player::nickname::NicknameConfig;
pub use crate::modules::mechanics::player::xp::XpConfig;
pub use crate::modules::mechanics::server::chat::ChatConfig;
pub use crate::modules::mechanics::server::messages::MessagesConfig;
pub use crate::modules::mechanics::server::rules::RulesConfig;
pub use crate::modules::mechanics::server::tablist::TablistConfig;
pub use crate::modules::mechanics::world::openable::OpenableConfig;
pub use crate::modules::mechanics::world::spawnprotection::SpawnProtectionConfig;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use pumpkin_plugin_api::permission::Permission;
use serde::{Deserialize, Serialize};
use tracing::error;

/// A trait representing a plugin mechanic that can be enabled or disabled.
///
/// Mechanics may optionally expose commands, permission nodes, and event handlers,
/// all registered with the server via [`Mechanic::register`].
pub trait Mechanic {
    /// Returns `true` if the module is enabled, `false` otherwise.
    fn enabled(&self) -> bool;

    /// Returns the commands provided by this mechanic.
    ///
    /// Each [`Command`] returned here will be registered with the server when
    /// [`Mechanic::register`] is called. Returns an empty vec by default.
    fn cmds(&self) -> Vec<Command> {
        vec![]
    }

    /// Returns the permission nodes required by this mechanic.
    ///
    /// All permissions returned here are registered with the server so permission
    /// plugins can grant/deny them. Enforcement happens inside each command's
    /// [`CommandHandler`] via `sender.has_permission(server, node)`.
    /// Returns an empty set by default.
    fn perms(&self) -> Vec<Permission> {
        Vec::new()
    }

    /// Registers event handlers for this mechanic.
    ///
    /// Override this to call [`Mechanic::register_event`] for each event this
    /// mechanic handles. No-op by default.
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

    /// Registers this mechanic's event handlers and commands with the server.
    ///
    /// Calls [`Mechanic::events`](Mechanic::events), registers each permission node
    /// from [`Mechanic::perms`] with the server, then registers each command from
    /// [`Mechanic::cmds`]. Commands are registered without a server-side permission
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

/// Top-level configuration for all mechanics.
///
/// Each field toggles one mechanic module. All mechanics are disabled by default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct MechanicsConfig {
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
    /// Server rules display in book format.
    pub rules: RulesConfig,
    /// Tab list header/footer.
    pub tablist: TablistConfig,
    /// Double-door synchronization.
    pub openable: OpenableConfig,
}
