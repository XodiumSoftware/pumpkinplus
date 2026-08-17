//! Command registration helpers.

use pumpkin_plugin_api::permission::{Permission, PermissionDefault};

/// Builds a permission node for a command in the standard
/// `{plugin_id}:command.{name}` format with the default value set to `Allow`.
#[must_use]
pub fn default_permission(plugin_id: &str, name: &str, description: &str) -> Permission {
    Permission {
        node: format!("{plugin_id}:command.{name}"),
        description: description.to_string(),
        default: PermissionDefault::Allow,
        children: vec![],
    }
}
