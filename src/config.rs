//! Configuration management system.
//!
//! The plugin configuration lives in a single `config.toml` file inside the
//! plugin's data folder. It holds the per-module configs (see
//! [`MechanicsConfig`]), each with its own `enabled` flag.
//!
//! The file is merged onto its typed defaults using the
//! [`config`](https://crates.io/crates/config) crate, so missing keys fall back
//! to defaults and extra keys in the user's file are preserved when re-saved.

use config::{File, FileFormat};
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;
use tracing::error;

pub(crate) use crate::modules::module::MechanicsConfig;

thread_local! {
    static CONFIG: RefCell<Option<PluginConfig>> = const { RefCell::new(None) };
}

/// Top-level plugin configuration, loaded from `config.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PluginConfig {
    /// All module configs, flattened into the top level of `config.toml`.
    #[serde(flatten)]
    pub mechanics: MechanicsConfig,
}

/// Loads and provides access to the plugin configuration.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ConfigManager;

impl ConfigManager {
    /// Loads `config.toml`, stores the merged result globally, and persists the
    /// merged file back to disk.
    ///
    /// Call this once in `Plugin::on_load` after all modules are ready.
    pub(crate) fn load(context: &Context) -> PluginConfig {
        let config = PluginConfig::load(context);
        CONFIG.with(|c| *c.borrow_mut() = Some(config.clone()));
        config
    }

    /// Returns the loaded configuration, if any.
    #[must_use]
    pub(crate) fn get() -> Option<PluginConfig> {
        CONFIG.with(|c| c.borrow().clone())
    }
}

impl PluginConfig {
    fn load(context: &Context) -> Self {
        load_config_file(&PathBuf::from(context.get_data_folder()))
    }
}

/// Loads `config.toml` from `data_folder`, merges it over `T::default()`,
/// writes the merged result back to disk, and returns the typed value. Falls
/// back to `T::default()` on any error.
fn load_config_file<T>(data_folder: &Path) -> T
where
    T: Default + Serialize + for<'de> Deserialize<'de>,
{
    const FILE_NAME: &str = "config.toml";
    let path = data_folder.join(FILE_NAME);

    let defaults_toml = toml::to_string(&T::default()).unwrap_or_else(|e| {
        error!("Failed to serialize defaults for {FILE_NAME}: {e}");
        String::new()
    });

    let builder = config::Config::builder()
        .add_source(File::from_str(&defaults_toml, FileFormat::Toml))
        .add_source(File::from(path.clone()).required(false));

    let cfg = match builder.build() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build config for {FILE_NAME}: {e}");
            return T::default();
        }
    };

    let merged: Value = match cfg.try_deserialize() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to deserialize merged config for {FILE_NAME}: {e}");
            return T::default();
        }
    };

    let value: T = match merged.clone().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse merged config for {FILE_NAME}: {e}");
            T::default()
        }
    };

    if let Some(parent) = path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        error!("Failed to create config directory for {FILE_NAME}: {e}");
    }

    if let Err(e) = fs::write(&path, toml::to_string_pretty(&merged).unwrap_or_default()) {
        error!("Failed to write config file {FILE_NAME}: {e}");
    }

    value
}
