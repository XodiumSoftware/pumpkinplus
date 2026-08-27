//! Mechanic system for `PumpkinPlus`.
//!
//! Each gameplay feature is implemented as a module implementing the [`Mechanic`] trait.
//! Mechanics can register event handlers, commands, and permission nodes.

pub use crate::modules::mechanics::entity::griefing::GriefingConfig;
pub use crate::modules::mechanics::player::enderchest::EnderchestConfig;
pub use crate::modules::mechanics::player::messages::MessagesConfig;
pub use crate::modules::mechanics::player::nickname::NicknameConfig;
pub use crate::modules::mechanics::server::chat::ChatConfig;
pub use crate::modules::mechanics::server::tablist::TablistConfig;
pub use crate::modules::mechanics::world::openable::OpenableConfig;
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
    /// Permissions are paired with commands by index when registering. If there
    /// are fewer permissions than commands, remaining commands are registered
    /// without a permission requirement. Returns an empty set by default.
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
    /// Calls [`Mechanic::events`](Mechanic::events), then registers each command from
    /// [`Mechanic::cmds`] paired with its corresponding permission from [`Mechanic::perms`]
    /// by index. Commands without a paired permission use an empty permission string.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        self.events(context);
        let perms = self.perms();
        for perm in &perms {
            if let Err(e) = context.register_permission(perm) {
                error!("Failed to register permission '{}': {e}", perm.node);
            }
        }

        let perm_nodes: Vec<String> = perms.into_iter().map(|p| p.node).collect();
        for (i, cmd) in self.cmds().into_iter().enumerate() {
            let perm = perm_nodes.get(i).cloned().unwrap_or_default();
            context.register_command(cmd, &perm);
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
    /// Mob griefing prevention.
    pub griefing: GriefingConfig,
    /// Shared enderchest mechanics.
    pub enderchest: EnderchestConfig,
    /// Custom join/leave/kick messages.
    pub messages: MessagesConfig,
    /// Player nickname commands.
    pub nickname: NicknameConfig,
    /// Chat formatting and filtering.
    pub chat: ChatConfig,
    /// Tab list header/footer.
    pub tablist: TablistConfig,
    /// Double-door synchronization.
    pub openable: OpenableConfig,
}
