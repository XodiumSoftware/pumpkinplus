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
pub enum InteractionAction {
    RightClickBlock,
    RightClickAir,
    LeftClickBlock,
    LeftClickAir,
}

impl InteractionAction {
    /// Returns true if the given list is empty (allow-all) or contains this action.
    pub fn matches_config(&self, allowed: &[Self]) -> bool {
        allowed.is_empty() || allowed.contains(self)
    }

    /// Convert from any type that implements `Debug` (such as the upstream API action).
    ///
    /// Uses the `Debug` representation as the canonical name. This is still an
    /// improvement over raw strings because the conversion is centralised and the
    /// config is validated at deserialization time.
    pub fn from_debug<T: std::fmt::Debug>(value: &T) -> Self {
        let repr = format!("{:?}", value);
        match repr.as_str() {
            "RightClickBlock" => Self::RightClickBlock,
            "RightClickAir" => Self::RightClickAir,
            "LeftClickBlock" => Self::LeftClickBlock,
            "LeftClickAir" => Self::LeftClickAir,
            _ => Self::RightClickBlock,
        }
    }
}

impl fmt::Display for InteractionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
