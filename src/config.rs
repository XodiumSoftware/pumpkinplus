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

pub use crate::modules::enchantments::enchantment::EnchantmentsConfig;
pub use crate::modules::mechanics::mechanic::MechanicsConfig;
pub use crate::modules::recipes::recipe::RecipesConfig;

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
    /// All mechanic module toggles.
    pub mechanics: MechanicsConfig,
    /// Custom recipe pack toggles.
    pub recipes: RecipesConfig,
    /// Vanilla enchantment behavior override toggles.
    pub enchantments: EnchantmentsConfig,
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
        let path = PathBuf::from(context.get_data_folder()).join("config.json");

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
