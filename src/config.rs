//! Configuration management system.
//!
//! The plugin configuration is split across multiple TOML files inside the
//! plugin's data folder:
//!
//! | File                | Purpose                                            |
//! |---------------------|----------------------------------------------------|
//! | `config.toml`       | Master toggles for each module group               |
//! | `bridges.toml`      | Per-bridge toggles under `BridgesConfig`           |
//! | `mechanics.toml`    | Per-mechanic settings under `MechanicsConfig`      |
//! | `enchantments.toml` | Per-enchantment toggles under `EnchantmentsConfig` |
//! | `recipes.toml`      | Per-recipe pack toggles under `RecipesConfig`      |
//!
//! Each file is merged layered onto its typed defaults using the
//! [`config`](https://crates.io/crates/config) crate, so missing keys fall back
//! to defaults and extra keys in user files are preserved when re-saved.

use config::{File, FileFormat};
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;
use tracing::error;

pub use crate::modules::bridges::bridge::BridgesConfig;
pub use crate::modules::enchantments::enchantment::EnchantmentsConfig;
pub use crate::modules::mechanics::mechanic::MechanicsConfig;
pub use crate::modules::recipes::RecipesConfig;

thread_local! {
    static CONFIG: RefCell<Option<PluginConfig>> = const { RefCell::new(None) };
}

/// Master module group toggles stored in `config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct ModuleToggles {
    /// Master toggle for all bridge modules.
    pub bridges: bool,
    /// Master toggle for all enchantment modules.
    pub enchantments: bool,
    /// Master toggle for all mechanic modules.
    pub mechanics: bool,
    /// Master toggle for all recipe pack modules.
    pub recipes: bool,
}

impl Default for ModuleToggles {
    fn default() -> Self {
        Self {
            bridges: true,
            enchantments: true,
            mechanics: true,
            recipes: true,
        }
    }
}

/// Top-level plugin configuration.
///
/// The `modules` field is read from `config.toml` and controls whether each
/// module group's own file is even consulted. The remaining fields hold the
/// fully-merged per-group configurations loaded from their own TOML files.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PluginConfig {
    /// Master toggles for each module group (from `config.toml`).
    pub modules: ModuleToggles,
    /// All bridge module configs (from `bridges.toml`).
    #[serde(skip)]
    pub bridges: BridgesConfig,
    /// All mechanic module configs (from `mechanics.toml`).
    #[serde(skip)]
    pub mechanics: MechanicsConfig,
    /// Custom recipe pack toggles (from `recipes.toml`).
    #[serde(skip)]
    pub recipes: RecipesConfig,
    /// Vanilla enchantment behavior override toggles (from `enchantments.toml`).
    #[serde(skip)]
    pub enchantments: EnchantmentsConfig,
}

/// Loads and provides access to the plugin configuration.
#[derive(Debug, Clone, Copy)]
pub struct ConfigManager;

impl ConfigManager {
    /// Loads `config.toml` plus all enabled module group configs, stores the
    /// merged result globally, and persists merged files back to disk.
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
        let data_folder = PathBuf::from(context.get_data_folder());

        let modules: ModuleToggles = load_section(&data_folder, "config.toml");

        let mut config = Self {
            modules: modules.clone(),
            bridges: BridgesConfig::default(),
            mechanics: MechanicsConfig::default(),
            recipes: RecipesConfig::default(),
            enchantments: EnchantmentsConfig::default(),
        };

        if modules.bridges {
            config.bridges = load_section(&data_folder, "bridges.toml");
        }
        if modules.mechanics {
            config.mechanics = load_section(&data_folder, "mechanics.toml");
        }
        if modules.recipes {
            config.recipes = load_section(&data_folder, "recipes.toml");
        }
        if modules.enchantments {
            config.enchantments = load_section(&data_folder, "enchantments.toml");
        }

        config
    }
}

/// Loads a single TOML section file from `data_folder`, merges it over
/// `T::default()`, writes the merged result back to disk, and returns the
/// typed value. Falls back to `T::default()` on any error.
fn load_section<T>(data_folder: &Path, file_name: &str) -> T
where
    T: Default + Serialize + for<'de> Deserialize<'de>,
{
    let path = data_folder.join(file_name);

    let defaults_toml = toml::to_string(&T::default()).unwrap_or_else(|e| {
        error!("Failed to serialize defaults for {file_name}: {e}");
        String::new()
    });

    let builder = config::Config::builder()
        .add_source(File::from_str(&defaults_toml, FileFormat::Toml))
        .add_source(File::from(path.clone()).required(false));

    let cfg = match builder.build() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build config for {file_name}: {e}");
            return T::default();
        }
    };

    let merged: Value = match cfg.try_deserialize() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to deserialize merged config for {file_name}: {e}");
            return T::default();
        }
    };

    let value: T = match merged.clone().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse merged config for {file_name}: {e}");
            T::default()
        }
    };

    if let Some(parent) = path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        error!("Failed to create config directory for {file_name}: {e}");
    }

    if let Err(e) = fs::write(&path, toml::to_string_pretty(&merged).unwrap_or_default()) {
        error!("Failed to write config file {file_name}: {e}");
    }

    value
}
