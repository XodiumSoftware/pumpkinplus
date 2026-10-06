//! Rules module — display server rules in a book interface.
//!
//! When a player runs `/rules`, a written book opens displaying the server
//! rules organized by category (player rules, mod/admin rules).
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
//! | `Book` builder | Create written book with pages programmatically |
//! | `Player.open_book()` | Open book UI without requiring item in hand |
//! | Book page components | MiniMessage-formatted pages in written book |
//!
//! Note: `Player.open_book(hand)` exists but requires a book item in the
//! player's hand — cannot create a virtual book from plugin.
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |
//!
//! ## Commands
//!
//! | Command | Description |
//! |---------|-------------|
//! | `/rules` | Open the server rules book |
//!
//! `IllyriaPlus` behavior reference:
//! - Rules displayed as written book with multiple pages
//! - Page 1: Player rules (1-7)
//! - Page 2: Player rules (8-13)
//! - Page 3: Mod/Admin rules

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use serde::{Deserialize, Serialize};

/// Handles server rules display in book format.
///
/// See module-level docs for the current implementation status and missing APIs.
#[derive(Default)]
pub struct Rules;

impl Mechanic for Rules {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.rules.enabled)
    }

    // No commands registered — stub module due to missing APIs.
    // Book creation and programmatic opening not available in plugin API.
}

/// Configuration for the rules mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RulesConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
