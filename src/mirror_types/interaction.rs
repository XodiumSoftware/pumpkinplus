//! Mirror of the API player interaction action enum.
//!
//! These are the actions that can trigger a [`PlayerInteractEvent`].

use serde::{Deserialize, Serialize};
use std::fmt;

/// Mirror of the API player interaction action enum.
///
/// These are the actions that can trigger a [`PlayerInteractEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum InteractAction {
    RightClickBlock,
    RightClickAir,
    LeftClickBlock,
    LeftClickAir,
}

impl InteractAction {
    /// Returns true if the given list is empty (allow-all) or contains this InteractAction.
    pub fn matches_config(&self, allowed: &[Self]) -> bool {
        allowed.is_empty() || allowed.contains(self)
    }
}

impl fmt::Display for InteractAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Convert from the upstream API InteractAction to our mirror type.
///
/// Uses the debug representation as the canonical name, falling back
/// to `RightClickBlock` if the upstream type emits something unexpected.
impl From<pumpkin_plugin_api::events::InteractAction> for InteractAction {
    fn from(value: pumpkin_plugin_api::events::InteractAction) -> Self {
        #[allow(unreachable_patterns)]
        match value {
            pumpkin_plugin_api::events::InteractAction::RightClickBlock => Self::RightClickBlock,
            pumpkin_plugin_api::events::InteractAction::RightClickAir => Self::RightClickAir,
            pumpkin_plugin_api::events::InteractAction::LeftClickBlock => Self::LeftClickBlock,
            pumpkin_plugin_api::events::InteractAction::LeftClickAir => Self::LeftClickAir,
            _ => Self::RightClickBlock,
        }
    }
}
