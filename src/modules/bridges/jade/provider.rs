//! Provider contracts for Jade.
//!
//! A *provider* answers one specific question about a block or entity —
//! e.g. "how many bees are in this beehive?" or "what's the brewing
//! progress of this brewing stand?" — and writes a small NBT payload under
//! its key into the response compound. The provider's key is also the
//! string advertised to the client in the `jade:server_handshake`.
//!
//! Providers are intentionally *single-purpose*, mirroring the original
//! Paper module layout. The `Jade` bridge routes incoming requests to the
//! providers registered in its two static lists.
//!
//! Each provider's config toggle lives under `[jade.providers]` in
//! `bridges.toml`. By default the toggle key is derived from the provider's
//! Rust struct name converted to `snake_case` (e.g. `Beehive` → `beehive`).
//! Providers whose advertised Jade key does not match the struct name can
//! override `enabled_key()` manually.
//!
//! Disabled providers are not advertised during the Jade handshake and are
//! skipped when processing client requests.

use std::any::type_name;
use std::sync::Mutex;

use pumpkin_plugin_api::Entity;
use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::World;

use crate::config::ConfigManager;

use crate::modules::bridges::jade::nbt::NbtCompound;

/// Cache mapping a provider's `std::any::type_name` to its snake-cased config
/// key. Populated lazily because converting to `snake_case` requires an
/// allocation; the result is leaked once per provider type.
static ENABLED_KEYS: Mutex<Vec<(&'static str, &'static str)>> = Mutex::new(Vec::new());

/// Converts an `UpperCamelCase` identifier (e.g. `Beehive`) to `snake_case`
/// (e.g. `beehive`).
fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
}

/// Derives the config key for a provider type from its `type_name`.
fn enabled_key_for<T: ?Sized>() -> &'static str {
    let full_name = type_name::<T>();
    let mut cache = ENABLED_KEYS.lock().expect("enabled key cache poisoned");
    if let Some((_, key)) = cache.iter().find(|(name, _)| *name == full_name) {
        return key;
    }
    let struct_name = full_name.rsplit("::").next().unwrap_or(full_name);
    let key: &'static str = Box::leak(to_snake_case(struct_name).into_boxed_str());
    cache.push((full_name, key));
    key
}

/// Provider for block-targeted Jade queries.
pub(crate) trait JadeBlockProvider: Send + Sync {
    /// Provider identifier advertised in the handshake, e.g. `"minecraft:beehive"`.
    fn key(&self) -> &'static str;

    /// Config key under `[jade.providers]` that enables this provider.
    ///
    /// Defaults to the provider's Rust struct name in `snake_case`.
    fn enabled_key(&self) -> &'static str {
        enabled_key_for::<Self>()
    }

    /// Returns `true` if this provider is enabled in the config.
    ///
    /// Falls back to `true` when the config is unavailable so a missing
    /// config file does not silently disable every provider.
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .and_then(|cm| cm.bridges.jade.providers.get(self.enabled_key()).copied())
            .unwrap_or(true)
    }

    /// Writes NBT for this block into `tag`.
    ///
    /// Returns `true` if any data was written; `false` to skip this provider
    /// for the given block (e.g., block isn't the right type).
    fn write(&self, world: &World, pos: BlockPos, tag: &mut NbtCompound) -> bool;
}

/// Provider for entity-targeted Jade queries.
pub(crate) trait JadeEntityProvider: Send + Sync {
    /// Provider identifier advertised in the handshake, e.g. `"minecraft:entity_health"`.
    fn key(&self) -> &'static str;

    /// Config key under `[jade.providers]` that enables this provider.
    ///
    /// Defaults to the provider's Rust struct name in `snake_case`.
    fn enabled_key(&self) -> &'static str {
        enabled_key_for::<Self>()
    }

    /// Returns `true` if this provider is enabled in the config.
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .and_then(|cm| cm.bridges.jade.providers.get(self.enabled_key()).copied())
            .unwrap_or(true)
    }

    /// Writes NBT for this entity into `tag`.
    ///
    /// Returns `true` if any data was written; `false` to skip this provider
    /// for the given entity (e.g., not living, not the right kind).
    fn write(&self, entity: &Entity, tag: &mut NbtCompound) -> bool;
}
