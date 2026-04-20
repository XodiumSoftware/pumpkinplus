//! Module system for PumpkinPlus.
//!
//! Each feature is implemented as a module implementing the [`Module`] trait.
//! Modules can register event handlers, commands, and permission nodes.

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::Command;
use std::collections::HashSet;

/// A trait representing a plugin module that can be enabled or disabled.
///
/// Modules may optionally expose commands, permission nodes, and event handlers,
/// all registered with the server via [`Module::register`].
pub trait Module {
    /// Returns `true` if the module is enabled, `false` otherwise.
    fn enabled(&self) -> bool;

    /// Returns the commands provided by this module.
    ///
    /// Each [`Command`] returned here will be registered with the server when
    /// [`Module::register`] is called. Returns an empty vec by default.
    fn cmds(&self) -> Vec<Command> {
        vec![]
    }

    /// Returns the permission nodes required by this module.
    ///
    /// Permissions are paired with commands by index when registering. If there
    /// are fewer permissions than commands, remaining commands are registered
    /// without a permission requirement. Returns an empty set by default.
    fn perms(&self) -> HashSet<String> {
        HashSet::new()
    }

    /// Registers event handlers for this module.
    ///
    /// Override this to call [`Context::register_event_handler`] for each event
    /// this module handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers this module's event handlers and commands with the server.
    ///
    /// Calls [`Module::events`](Module::events), then registers each command from
    /// [`Module::cmds`] paired with its corresponding permission from [`Module::perms`]
    /// by index. Commands without a paired permission use an empty permission string.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        self.events(context);
        let perms: Vec<String> = self.perms().into_iter().collect();
        for (i, cmd) in self.cmds().into_iter().enumerate() {
            let perm = perms.get(i).cloned().unwrap_or_default();
            context.register_command(cmd, &perm);
        }
    }
}
