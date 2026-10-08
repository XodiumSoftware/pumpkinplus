//! Xaero's Map bridge — tells Xaero's World Map / Minimap clients which
//! server-level "world id" they're connected to so map data persists across
//! multiplayer sessions on this server.
//!
//! Without this bridge, Xaero clients fall back to a per-session id and
//! their minimap/worldmap data is effectively tied to a single server
//! process lifetime. With this bridge, the server assigns a stable id on
//! first startup, which the config system then persists to `bridges.toml`.
//!
//! ## Protocol
//!
//! | Channel                | Direction       | Payload                |
//! |------------------------|-----------------|------------------------|
//! | `xaeroworldmap:main`   | server → client | `u8` (0) + `i32` (id)  |
//! | `xaerominimap:main`    | server → client | `u8` (0) + `i32` (id)  |
//!
//! ## Configuration
//!
//! | Field     | Default      | Description                                      |
//! |-----------|--------------|--------------------------------------------------|
//! | `enabled` | `false`      | Whether this bridge is active                    |
//! | `id`      | *(random)*   | Stable server id, auto-generated on first run    |

use crate::config::ConfigManager;
use crate::modules::bridges::bridge::Bridge;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerChangedWorldEvent, PlayerRegisterChannelEvent,
};
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

/// Channel for Xaero's World Map mod.
const WORLDMAP_CHANNEL: &str = "xaeroworldmap:main";
/// Channel for Xaero's Minimap mod.
const MINIMAP_CHANNEL: &str = "xaerominimap:main";

/// Handles Xaero's World Map / Minimap plugin channel synchronization.
#[derive(Default)]
pub struct XaeroMap;

impl Bridge for XaeroMap {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.bridges.xaeromap.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerRegisterChannelEvent>(context, EventPriority::Normal, true);
        self.register_event::<PlayerChangedWorldEvent>(context, EventPriority::Normal, true);
    }
}

impl XaeroMap {
    /// Sends the world id to the player on the requested channel.
    fn send_world_id(player: &Player, channel: &str, world_id: i32) {
        let Some(java) = player.as_java() else {
            return;
        };

        let mut payload = Vec::with_capacity(5);
        payload.push(0u8);
        payload.extend_from_slice(&world_id.to_be_bytes());

        java.send_custom_payload(channel, &payload);
    }

    /// Convenience: read the world id from the loaded config.
    fn config_id() -> Option<i32> {
        ConfigManager::get().map(|cm| cm.bridges.xaeromap.id)
    }
}

impl EventHandler<PlayerRegisterChannelEvent> for XaeroMap {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerRegisterChannelEvent>,
    ) -> EventData<PlayerRegisterChannelEvent> {
        if !matches!(event.channel.as_str(), WORLDMAP_CHANNEL | MINIMAP_CHANNEL) {
            return event;
        }

        if let Some(id) = Self::config_id() {
            Self::send_world_id(&event.player, &event.channel, id);
        }

        event.cancelled = false;
        event
    }
}

impl EventHandler<PlayerChangedWorldEvent> for XaeroMap {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerChangedWorldEvent>,
    ) -> EventData<PlayerChangedWorldEvent> {
        if let Some(id) = Self::config_id() {
            Self::send_world_id(&event.player, WORLDMAP_CHANNEL, id);
            Self::send_world_id(&event.player, MINIMAP_CHANNEL, id);
        }
        event
    }
}

/// Configuration for the Xaero Map bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XaeroMapConfig {
    /// Whether this bridge is active.
    pub enabled: bool,
    /// Stable server-level world id shared with Xaero clients.
    ///
    /// Auto-generated on first run by [`Default`], then persisted by the
    /// config loader so the same id is reused across restarts. Operators
    /// can also set this manually if migrating from another server.
    pub id: i32,
}

impl Default for XaeroMapConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            id: generate_id(),
        }
    }
}

/// Generates a new random-ish `i32` id for first-run defaults.
///
/// No CSPRNG is needed here — the id only has to be unique across servers,
/// not unpredictable. Hashing the current system time (plus process id for
/// extra entropy) gives a stable value for this process that's effectively
/// impossible to collide across real deployments.
fn generate_id() -> i32 {
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    {
        (hasher.finish() & u64::from(u32::MAX)) as u32 as i32
    }
}
