//! Mirror types for safe conversions between Pumpkin plugin API enums and our own types.
//!
//! These enums decouple our configuration from upstream API debug representations,
//! giving us compile-time guarantees and clean serde deserialization.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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
    /// Returns true if the given list is empty (allow-all) or contains this gamemode.
    pub fn matches_config(&self, allowed: &[Self]) -> bool {
        allowed.is_empty() || allowed.contains(self)
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for GameMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Survival" => Ok(Self::Survival),
            "Creative" => Ok(Self::Creative),
            "Adventure" => Ok(Self::Adventure),
            "Spectator" => Ok(Self::Spectator),
            other => Err(format!("unknown GameMode variant: {}", other)),
        }
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

impl FromStr for InteractionAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //TODO: when API available use that, like with GameMode.
        match s {
            "RightClickBlock" => Ok(Self::RightClickBlock),
            "RightClickAir" => Ok(Self::RightClickAir),
            "LeftClickBlock" => Ok(Self::LeftClickBlock),
            "LeftClickAir" => Ok(Self::LeftClickAir),
            other => Err(format!("unknown InteractionAction variant: {}", other)),
        }
    }
}
