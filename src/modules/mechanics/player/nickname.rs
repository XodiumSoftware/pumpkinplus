//! Nickname module — set or remove player nicknames.
//!
//! ## Commands
//!
//! | Command          | Aliases | Permission                      | Description              |
//! |------------------|---------|---------------------------------|--------------------------|
//! | `/nickname [name]` | `nick`  | `pumpkinplus:command.nickname` | Set or remove nickname   |
//!
//! ## Configuration
//!
//! | Field       | Default | Description                                         |
//! |-------------|---------|-----------------------------------------------------|
//! | `enabled`   | `false` | Whether this module is active                       |
//!
//! ## Mechanics
//!
//! - `/nickname` — clears the player's nickname.
//! - `/nickname <name>` — sets the player's nickname.
//! - On join, the stored nickname is applied to the player's display name and tab list name.
//! - A confirmation message is sent via the action bar.
//!
//! ## Notes
//!
//! This module is currently a stub. The Pumpkin plugin API exposes `set-display-name`,
//! `set-tab-list-name`, and `show-actionbar`, but it does **not** expose a persistent
//! data container (PDC) API for storing nicknames across sessions. Until persistent
//! player data or a config-based store is available, nicknames cannot survive rejoins
//! or server restarts.

use crate::config::ConfigManager;
use crate::module::Module;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};

/// Handles player nicknames.
#[derive(Default)]
pub struct Nickname;

impl Module for Nickname {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<NicknameConfig>().enabled)
            .unwrap_or(false)
    }

    fn events(&self, _context: &Context) {
        // TODO: Implement when a persistent player data API (or equivalent) is
        // available in the Pumpkin plugin API.
        //
        // The intended logic is:
        //
        // 1. Register `/nickname` command with optional greedy string argument.
        //    - No argument: clear nickname.
        //    - With argument: set nickname.
        // 2. On command execution:
        //    - Update persistent storage with the new nickname.
        //    - Apply `set-display-name` with the nickname.
        //    - Apply `set-tab-list-name` with the nickname.
        //    - Send confirmation via `show-actionbar`.
        // 3. On `PlayerJoinEvent`:
        //    - Read stored nickname from persistent storage.
        //    - Apply display name and tab list name.
        //    - Trigger tab list refresh.
    }
}

/// Configuration for the nickname mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NicknameConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
