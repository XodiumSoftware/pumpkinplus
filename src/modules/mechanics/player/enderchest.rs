//! Enderchest module - portable per-player enderchest access.
//!
//! ## Configuration
//!
//! | Field       | Default                     | Description                               |
//! |-------------|-----------------------------|-------------------------------------------|
//! | `enabled`   | `false`                     | Whether this module is active             |
//! | `gamemodes` | `["Survival", "Adventure"]` | Gamemodes allowed to use enderchests      |
//! | `actions`   | `["RightClickAir"]`         | Interaction actions that trigger the GUI  |
//! | `hand`      | `["Right"]`                 | Hands the ender chest item can be held in |
//!
//! ## Permissions
//!
//! | Node | Default | Description |
//! |------|---------|-------------|
//! | `pumpkinplus:enderchest.use` | allow | Allows opening the enderchest via right-click |
//!
//! Note: the permission node uses the `enderchest.use` subpath rather than
//! `command.*` because this mechanic is event-driven, not command-driven.
//!
//! ## Mechanics
//!
//! When a player right-clicks in the air while holding an ender chest item,
//! their personal ender chest inventory screen is opened. This makes the ender
//! chest portable.

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::{GameMode, InteractAction, MirrorHand};
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerInteractEvent};
use pumpkin_plugin_api::item::{Item, ItemStackExt};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Permission node required to open the enderchest via the item interaction.
pub const PERM_ENDERCHEST: &str = concat!(env!("CARGO_PKG_NAME"), ":enderchest.use");

/// Handles enderchest mechanics.
#[derive(Default)]
pub struct Enderchest;

impl Mechanic for Enderchest {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.enderchest.enabled)
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: PERM_ENDERCHEST.into(),
            description: "Allows opening the enderchest via the item interaction.".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        }]
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerInteractEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<PlayerInteractEvent> for Enderchest {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerInteractEvent>,
    ) -> EventData<PlayerInteractEvent> {
        if !self.enabled() {
            return event;
        }

        let config: EnderchestConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.enderchest)
            .unwrap_or_default();

        let action = InteractAction::from(event.action);
        if !action.matches_config(&config.actions) {
            return event;
        }

        let hand = [MirrorHand::Right, MirrorHand::Left].into_iter().find(|h| {
            event
                .player
                .get_item_in_hand((*h).into())
                .is_some_and(|item| item.is_item(Item::EnderChest))
        });

        let Some(hand) = hand else {
            return event;
        };

        if !hand.matches_config(&config.hand) {
            return event;
        }

        let gamemode = GameMode::from(event.player.get_gamemode());
        if !gamemode.matches_config(&config.gamemodes) {
            return event;
        }

        if !event.player.has_permission(PERM_ENDERCHEST) {
            return event;
        }

        event.player.open_ender_chest();
        event.cancelled = true;

        event
    }
}

/// Configuration for the enderchest mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnderchestConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// List of gamemodes allowed to use enderchests. Use variant names like "Survival", "Creative", etc. Leave empty to allow all.
    pub gamemodes: Vec<GameMode>,
    /// List of interaction actions that trigger opening the enderchest. Use variant names like `RightClickBlock`, `RightClickAir`, etc. Leave empty to allow all.
    pub actions: Vec<InteractAction>,
    /// Which hands the ender chest item can be held in to trigger the GUI.
    /// Use variant names like "Left" or "Right". Leave empty to allow either hand.
    pub hand: Vec<MirrorHand>,
}

impl Default for EnderchestConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            gamemodes: vec![GameMode::Survival, GameMode::Adventure],
            actions: vec![InteractAction::RightClickAir],
            hand: vec![MirrorHand::Right],
        }
    }
}
