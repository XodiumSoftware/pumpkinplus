//! `AppleSkin` bridge — syncs saturation, exhaustion, and natural regeneration
//! state to AppleSkin-enabled clients.
//!
//! `AppleSkin` is a client mod that displays food/hunger information. Without a
//! server bridge it only shows the client's best guess; with this bridge the
//! server pushes authoritative values so the HUD always matches reality.
//!
//! ## Protocol
//!
//! | Channel                         | Direction       | Payload         |
//! |---------------------------------|-----------------|-----------------|
//! | `appleskin:saturation`          | server → client | 1 `f32`         |
//! | `appleskin:exhaustion`          | server → client | 1 `f32`         |
//! | `appleskin:natural_regeneration`| server → client | 1 `u8` (0 or 1) |
//!
//! ## Lifecycle
//!
//! 1. Client sends `minecraft:register` listing its channels; we see
//!    `PlayerRegisterChannelEvent` per channel and mark the player as
//!    supporting `AppleSkin`.
//! 2. Server pushes saturation/exhaustion every tick while values change
//!    (per-player state tracked in a `HashMap<Uuid, …>`).
//! 3. Server pushes `natural_health_regeneration` whenever the world
//!    gamerule changes or the player changes worlds.
//! 4. Player state is dropped on `PlayerLeaveEvent`.
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this bridge is active   |

use crate::config::ConfigManager;
use crate::modules::bridges::bridge::Bridge;
use pumpkin_plugin_api::events::{
    EventHandler, EventPriority, PlayerChangedWorldEvent, PlayerJoinEvent, PlayerLeaveEvent,
    PlayerRegisterChannelEvent,
};
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::wit::pumpkin::plugin::game_rules::GameRule;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Channel for server → client saturation sync.
const SATURATION_CHANNEL: &str = "appleskin:saturation";
/// Channel for server → client exhaustion sync.
const EXHAUSTION_CHANNEL: &str = "appleskin:exhaustion";
/// Channel for server → client natural-regeneration gamerule sync.
const NATURAL_REGENERATION_CHANNEL: &str = "appleskin:natural_regeneration";

/// Minimum delta in exhaustion between ticks before we bother sending an
/// update; mirrors the original Bukkit implementation's threshold.
const MINIMUM_EXHAUSTION_CHANGE_THRESHOLD: f32 = 0.01;

/// Per-player last-sent values so we only send packets when something
/// actually changed.
#[derive(Debug, Clone, Copy, Default)]
struct SyncState {
    /// Whether the client has registered `appleskin:saturation`/`appleskin:exhaustion`.
    syncs_hunger_channels: bool,
    /// Whether the client has registered `appleskin:natural_regeneration`.
    syncs_natural_regen_channel: bool,
    /// Last saturation we sent. `-1.0` means "haven't sent yet".
    last_saturation: f32,
    /// Last exhaustion we sent. `-1.0` means "haven't sent yet".
    last_exhaustion: f32,
}

impl SyncState {
    /// Freshly-connected state: no channels registered, nothing sent yet.
    const fn new() -> Self {
        Self {
            syncs_hunger_channels: false,
            syncs_natural_regen_channel: false,
            last_saturation: -1.0,
            last_exhaustion: -1.0,
        }
    }
}

/// Process-wide map of per-player sync state, keyed by player UUID halves.
///
/// The WIT `Uuid` doesn't implement `Eq`/`Hash`, so we key by its raw
/// `(high, low)` halves instead. `OnceLock` because the bridge registers
/// itself once on plugin load.
static SYNC_STATES: OnceLock<Mutex<HashMap<(u64, u64), SyncState>>> = OnceLock::new();

/// Extracts a hashable key from a player's WIT UUID.
fn player_key(player: &Player) -> (u64, u64) {
    let id = player.get_id();
    (id.high, id.low)
}

/// Shared accessor so handlers and the ticking task use the same map.
fn sync_states() -> &'static Mutex<HashMap<(u64, u64), SyncState>> {
    SYNC_STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Handles `AppleSkin` plugin channel synchronization.
#[derive(Default)]
pub struct AppleSkin;

impl Bridge for AppleSkin {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.bridges.appleskin.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerRegisterChannelEvent>(context, EventPriority::Normal, true);
        self.register_event::<PlayerChangedWorldEvent>(context, EventPriority::Normal, true);
        self.register_event::<PlayerJoinEvent>(context, EventPriority::Normal, true);
        self.register_event::<PlayerLeaveEvent>(context, EventPriority::Normal, true);

        context.schedule_repeating_task(1, 1, |server| {
            Self::tick(&server);
        });
    }
}

impl AppleSkin {
    /// Pushes saturation/exhaustion to any player that has registered those channels.
    ///
    /// Runs every tick; only sends a packet when the value actually changed
    /// (or for saturation, when it differs from the last-sent value).
    fn tick(server: &Server) {
        let Ok(mut states) = sync_states().lock() else {
            return;
        };

        for player in server.get_all_players() {
            let id = player_key(&player);
            let Some(state) = states.get_mut(&id) else {
                continue;
            };

            if state.syncs_hunger_channels {
                let Some(java) = player.as_java() else {
                    continue;
                };

                let saturation = player.get_saturation();
                if (saturation - state.last_saturation).abs() > f32::EPSILON {
                    java.send_custom_payload(SATURATION_CHANNEL, &saturation.to_be_bytes());
                    state.last_saturation = saturation;
                }

                let exhaustion = player.get_exhaustion();
                if (exhaustion - state.last_exhaustion).abs() >= MINIMUM_EXHAUSTION_CHANGE_THRESHOLD
                {
                    java.send_custom_payload(EXHAUSTION_CHANNEL, &exhaustion.to_be_bytes());
                    state.last_exhaustion = exhaustion;
                }
            }
        }
    }

    /// Sends the current `natural_health_regeneration` gamerule of the
    /// player's current world to that player, if they've registered the
    /// channel.
    fn send_natural_regen_state(player: &Player) {
        let Ok(states) = sync_states().lock() else {
            return;
        };
        let Some(state) = states.get(&player_key(player)) else {
            return;
        };
        if !state.syncs_natural_regen_channel {
            return;
        }

        Self::send_natural_regen_value(player);
    }

    /// Sends the current value of the `natural_health_regeneration` gamerule
    /// in the player's current world, without consulting tracked state.
    fn send_natural_regen_value(player: &Player) {
        let Some(java) = player.as_java() else {
            return;
        };
        let world = player.get_world();
        let Some(enabled) = world.get_game_rule_bool(GameRule::NaturalHealthRegeneration) else {
            return;
        };
        java.send_custom_payload(NATURAL_REGENERATION_CHANNEL, &[u8::from(enabled)]);
    }
}

impl EventHandler<PlayerRegisterChannelEvent> for AppleSkin {
    fn handle(
        &self,
        _server: Server,
        mut event: pumpkin_plugin_api::events::EventData<PlayerRegisterChannelEvent>,
    ) -> pumpkin_plugin_api::events::EventData<PlayerRegisterChannelEvent> {
        let id = player_key(&event.player);

        {
            let Ok(mut states) = sync_states().lock() else {
                return event;
            };
            let entry = states.entry(id).or_insert_with(SyncState::new);

            match event.channel.as_str() {
                SATURATION_CHANNEL | EXHAUSTION_CHANNEL => {
                    entry.syncs_hunger_channels = true;
                }
                NATURAL_REGENERATION_CHANNEL => {
                    entry.syncs_natural_regen_channel = true;
                }
                _ => return event,
            }
        }

        if event.channel.as_str() == NATURAL_REGENERATION_CHANNEL {
            Self::send_natural_regen_value(&event.player);
        }

        event.cancelled = false;
        event
    }
}

impl EventHandler<PlayerChangedWorldEvent> for AppleSkin {
    fn handle(
        &self,
        _server: Server,
        event: pumpkin_plugin_api::events::EventData<PlayerChangedWorldEvent>,
    ) -> pumpkin_plugin_api::events::EventData<PlayerChangedWorldEvent> {
        // The new world may have a different natural_health_regeneration
        // value; resend on behalf of the new world.
        Self::send_natural_regen_state(&event.player);
        event
    }
}

// TODO(pumpkin-api): The Pumpkin plugin API does not currently expose an event
// for world gamerule changes (no `WorldGameRuleChangeEvent` analogue in
// `event.wit`), so we cannot push `natural_health_regeneration` updates when an
// operator toggles the rule mid-game like the Bukkit implementation can.
// Workaround would be polling in `tick()`, or upstream a feature request to
// Pumpkin. Either way, the client will eventually self-correct on world change
// or relog.

impl EventHandler<PlayerJoinEvent> for AppleSkin {
    fn handle(
        &self,
        _server: Server,
        event: pumpkin_plugin_api::events::EventData<PlayerJoinEvent>,
    ) -> pumpkin_plugin_api::events::EventData<PlayerJoinEvent> {
        if let Ok(mut states) = sync_states().lock() {
            states
                .entry(player_key(&event.player))
                .and_modify(Self::reset_state)
                .or_insert_with(SyncState::new);
        }
        event
    }
}

impl EventHandler<PlayerLeaveEvent> for AppleSkin {
    fn handle(
        &self,
        _server: Server,
        event: pumpkin_plugin_api::events::EventData<PlayerLeaveEvent>,
    ) -> pumpkin_plugin_api::events::EventData<PlayerLeaveEvent> {
        if let Ok(mut states) = sync_states().lock() {
            states.remove(&player_key(&event.player));
        }
        event
    }
}

impl AppleSkin {
    /// Reset per-player tracked values; channel-registration flags stay cleared.
    fn reset_state(state: &mut SyncState) {
        *state = SyncState::new();
    }
}

/// Configuration for the `AppleSkin` bridge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppleSkinConfig {
    /// Whether this bridge is active.
    pub enabled: bool,
}
