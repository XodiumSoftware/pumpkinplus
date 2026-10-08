//! Bridge system for `PumpkinPlus`.
//!
//! A *bridge* is a server-side handler for a client mod's plugin channel
//! protocol (e.g., `AppleSkin`, Xaero's Map, Jade). Bridges are conceptually
//! distinct from mechanics: they don't modify gameplay, they translate
//! server state into a format client mods understand.
//!
//! Each bridge implements the [`Bridge`] trait and registers event handlers
//! on [`Context`] via [`Bridge::events`].
//!
//! ## Lifecycle
//!
//! 1. [`Bridge::enabled`] is consulted to decide whether to register the
//!    bridge at all. Bridges are only useful if their corresponding client
//!    mod is expected on players.
//! 2. [`Bridge::events`] registers event handlers (e.g.,
//!    `PlayerRegisterChannelEvent`, `PlayerCustomPayloadEvent`,
//!    `PlayerChangedWorldEvent`) that drive the channel handshake and
//!    payload exchange.
//!
//! ## Channel pattern
//!
//! Most client mods use this pattern:
//!
//! 1. Client sends a `minecraft:register` payload listing its channels;
//!    server sees a `PlayerRegisterChannelEvent` per channel.
//! 2. Server notes that the client supports the mod and starts syncing.
//! 3. Bidirectional payloads flow via `JavaPlayer::send_custom_payload`
//!    (server → client) and `PlayerCustomPayloadEvent` (client → server).

pub use crate::modules::bridges::appleskin::AppleSkinConfig;
pub use crate::modules::bridges::xaeromap::XaeroMapConfig;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use serde::{Deserialize, Serialize};

/// A trait representing a plugin bridge that can be enabled or disabled.
///
/// Bridges register event handlers via [`Bridge::events`]. They have no
/// commands or permission nodes — those are mechanics' concern.
pub trait Bridge {
    /// Returns `true` if the bridge is enabled, `false` otherwise.
    fn enabled(&self) -> bool;

    /// Registers event handlers for this bridge.
    ///
    /// Override this to call [`Bridge::register_event`] for each event this
    /// bridge handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers `Self` as the handler for event `T` and panics on failure.
    ///
    /// This is a thin wrapper around [`Context::register_event_handler`] that
    /// supplies a bridge-specific error message so individual bridges don't
    /// have to repeat it.
    fn register_event<T>(&self, context: &Context, priority: EventPriority, ignore_cancelled: bool)
    where
        T: FromIntoEvent + Send + Sync + 'static,
        Self: EventHandler<T> + Default + Send + Sync + 'static,
    {
        context
            .register_event_handler::<T, _>(Self::default(), priority, ignore_cancelled)
            .expect("failed to register event handler");
    }

    /// Registers this bridge's event handlers with the server if enabled.
    fn register(&self, context: &Context) {
        if self.enabled() {
            self.events(context);
        }
    }
}

/// Top-level configuration for all bridges.
///
/// Each field toggles one bridge module. All bridges are disabled by default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct BridgesConfig {
    /// `AppleSkin` saturation/exhaustion/gamerule sync.
    pub appleskin: AppleSkinConfig,
    /// Xaero's World Map / Minimap world id sync.
    pub xaeromap: XaeroMapConfig,
}
