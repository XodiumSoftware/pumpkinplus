//! Mirror of the API gamemode enum.
//!
//! Matches Minecraft's four gamemodes exactly.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Mirror of the API gamemode enum.
///
/// Matches Minecraft's four gamemodes exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    /// Returns true if the given list is empty (allow-all) or contains this GameMode.
    pub fn matches_config(&self, allowed: &[Self]) -> bool {
        allowed.is_empty() || allowed.contains(self)
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Convert from the upstream API gamemode to our mirror type.
///
/// Uses the debug representation as the canonical name, falling back
/// to `Survival` if the upstream type emits something unexpected.
impl From<pumpkin_plugin_api::player::GameMode> for GameMode {
    fn from(value: pumpkin_plugin_api::player::GameMode) -> Self {
        #[allow(unreachable_patterns)]
        match value {
            pumpkin_plugin_api::player::GameMode::Survival => Self::Survival,
            pumpkin_plugin_api::player::GameMode::Creative => Self::Creative,
            pumpkin_plugin_api::player::GameMode::Adventure => Self::Adventure,
            pumpkin_plugin_api::player::GameMode::Spectator => Self::Spectator,
            _ => Self::Survival,
        }
    }
}
