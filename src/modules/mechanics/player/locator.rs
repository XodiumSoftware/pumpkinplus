//! Locator module — customize waypoint locator bar color.
//!
//! Allows players to personalize their waypoint locator bar color using
//! `/locator` command with named colors, hex codes, or reset to default.
//!
//! ## Status
//!
//! **Stub module** — Cannot be implemented due to missing Pumpkin plugin APIs.
//!
//! ## Missing APIs
//!
//! The following Pumpkin plugin APIs are required to implement this module:
//!
//! | API | Purpose |
//! |-----|---------|
//! | `Player.set_waypoint_color()` | Set the locator bar color |
//! | `Player.get_waypoint_color()` | Get the current locator bar color |
//! | Command arguments | Parse named colors and hex codes in `/locator` command |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! ## Commands
//!
//! | Command | Aliases | Description |
//! |---------|---------|-------------|
//! | `/locator` | `lc` | Show current locator color |
//! | `/locator <color>` | | Set locator color by name |
//! | `/locator <hex>` | | Set locator color by hex code |
//! | `/locator reset` | | Reset to default locator color |
//!
//! `IllyriaPlus` behavior reference:
//! - Waypoint color stored as player PDC (persistent data container)
//! - Action bar messages show current/new color
//! - Supports named colors and hex codes

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles locator bar customization.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Locator;

impl Mechanic for Locator {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.locator.enabled)
    }

    // No commands registered — stub module due to missing APIs.
    // Waypoint color APIs don't exist in the Pumpkin plugin API yet.
}

/// Configuration for the locator mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocatorConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
