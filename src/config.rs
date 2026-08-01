//! Configuration management system.
//!
//! Uses the [`config`](https://crates.io/crates/config) crate to load and merge
//! layered configuration from `{data_folder}/config.json`, falling back to typed
//! defaults when values are missing. Extra keys in the user's config are preserved
//! when the file is rewritten.

use config::{File, FileFormat};
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use tracing::error;

pub use crate::modules::mechanics::entity::bat::BatConfig;
pub use crate::modules::mechanics::entity::griefing::GriefingConfig;
pub use crate::modules::mechanics::entity::husk::HuskConfig;
pub use crate::modules::mechanics::entity::spawn_egg::SpawnEggConfig;
pub use crate::modules::mechanics::entity::tameable::TameableConfig;
pub use crate::modules::mechanics::player::enderchest::EnderchestConfig;
pub use crate::modules::mechanics::player::head::HeadConfig;
pub use crate::modules::mechanics::player::locator::LocatorConfig;
pub use crate::modules::mechanics::player::messages::MessagesConfig;
pub use crate::modules::mechanics::player::nickname::NicknameConfig;
pub use crate::modules::mechanics::server::chat::ChatConfig;
pub use crate::modules::mechanics::server::tablist::TablistConfig;
pub use crate::modules::mechanics::world::openable::OpenableConfig;

thread_local! {
    static CONFIG: RefCell<Option<PluginConfig>> = const { RefCell::new(None) };
}

/// Top-level plugin configuration.
///
/// Each field corresponds to one module's config section. Missing sections in the
/// JSON file fall back to the typed defaults for that module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PluginConfig {
    /// Bat drop mechanics.
    pub bat: BatConfig,
    /// Mob griefing prevention.
    pub griefing: GriefingConfig,
    /// Husk drop mechanics.
    pub husk: HuskConfig,
    /// Spawn egg drop mechanics.
    pub spawn_egg: SpawnEggConfig,
    /// Tameable ownership transfer.
    pub tameable: TameableConfig,
    /// Shared enderchest mechanics.
    pub enderchest: EnderchestConfig,
    /// Player head drop mechanics.
    pub head: HeadConfig,
    /// Locator bar personalization.
    pub locator: LocatorConfig,
    /// Custom join/leave/kick messages.
    pub messages: MessagesConfig,
    /// Player nickname commands.
    pub nickname: NicknameConfig,
    /// Chat formatting and filtering.
    pub chat: ChatConfig,
    /// Tab list header/footer.
    pub tablist: TablistConfig,
    /// Double-door synchronization.
    pub openable: OpenableConfig,
}

/// Loads and provides access to the plugin configuration.
#[derive(Debug, Clone, Copy)]
pub struct ConfigManager;

impl ConfigManager {
    /// Loads configuration from `{data_folder}/config.json`, merges it with
    /// defaults, writes the merged result back to disk, and stores it globally.
    ///
    /// Call this once in `Plugin::on_load` after all modules are ready.
    pub fn load(context: &Context) -> PluginConfig {
        let config = PluginConfig::load(context);
        CONFIG.with(|c| *c.borrow_mut() = Some(config.clone()));
        config
    }

    /// Returns the loaded configuration, if any.
    #[must_use]
    pub fn get() -> Option<PluginConfig> {
        CONFIG.with(|c| c.borrow().clone())
    }
}

impl PluginConfig {
    fn load(context: &Context) -> Self {
        let path =
            PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.json");

        let defaults_json = serde_json::to_string(&Self::default()).unwrap_or_else(|e| {
            error!("Failed to serialize default config: {e}");
            String::new()
        });

        let builder = config::Config::builder()
            .add_source(File::from_str(&defaults_json, FileFormat::Json))
            .add_source(File::from(path.clone()).required(false));

        let cfg = match builder.build() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to build config: {e}");
                return Self::default();
            }
        };

        let merged: Value = match cfg.try_deserialize() {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to deserialize merged config: {e}");
                return Self::default();
            }
        };

        let config: Self = match serde_json::from_value(merged.clone()) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to parse merged config into PluginConfig: {e}");
                Self::default()
            }
        };

        if let Some(parent) = path.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            error!("Failed to create config directory: {e}");
        }

        if let Err(e) = fs::write(
            &path,
            serde_json::to_string_pretty(&merged).unwrap_or_default(),
        ) {
            error!("Failed to write config: {e}");
        }

        config
    }
}
