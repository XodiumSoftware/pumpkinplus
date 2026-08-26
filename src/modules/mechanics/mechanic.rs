//! Mechanic system for `PumpkinPlus`.
//!
//! Each gameplay feature is implemented as a module implementing the [`Mechanic`] trait.
//! Mechanics can register event handlers, commands, and permission nodes.

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::permission::Permission;
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
    /// Override this to call [`Context::register_event_handler`] for each event
    /// this mechanic handles. No-op by default.
    fn events(&self, _context: &Context) {}

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
