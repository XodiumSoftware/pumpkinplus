//! Configuration management system.
//!
//! Uses a registry pattern where modules register their configs by name,
//! and ConfigManager handles loading from disk with merge semantics.
//!
//! ## Config Location
//!
//! The config file is stored at `{data_folder}/config.toml`.
//! It is created automatically on first load with all registered defaults.

use figment::Figment;
use figment::providers::{Format, Toml};
use pumpkin_plugin_api::Context;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::error;

/// Extracts a config key from a type's full name.
/// For example:
/// - `crate::modules::mechanics::player::Config` -> "player"
/// - `crate::modules::mechanics::player::PlayerConfig` -> "player"
pub fn config_key<T>() -> String {
    use std::any::type_name;

    let full_name = type_name::<T>();
    let parts: Vec<&str> = full_name.split("::").collect();

    if parts.len() >= 2 {
        parts[parts.len() - 2].to_string()
    } else if let Some(&last) = parts.last() {
        last.strip_suffix("Config")
            .map_or_else(|| last.to_string(), |s| s.to_string())
    } else {
        full_name.to_string()
    }
}

thread_local! {
    static CONFIG: RefCell<Option<ConfigManager>> = const { RefCell::new(None) };
}

/// Manages plugin configuration using a registry pattern.
/// Modules register their configs by name, and ConfigManager handles
/// loading from disk with merge semantics for missing fields.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager {
    #[serde(flatten)]
    configs: HashMap<String, Value>,
}

impl ConfigManager {
    /// Creates an empty ConfigManager ready for registration.
    pub fn empty() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Returns the global config manager instance.
    pub fn get() -> Option<Self> {
        CONFIG.with(|c| c.borrow().clone())
    }

    /// Gets a config by type, deriving the key from the type name.
    /// Returns defaults if not found or parse fails.
    pub fn get_config<T: DeserializeOwned + Default + 'static>(&self) -> T {
        let key = config_key::<T>();
        self.configs
            .get(&key)
            .and_then(|v| {
                serde_json::from_value(v.clone())
                    .inspect_err(|e| error!("Failed to parse config for key '{}': {}", key, e))
                    .ok()
            })
            .unwrap_or_default()
    }

    /// Registers a config with default values for a module.
    /// The key is derived automatically from the type name.
    pub fn register<T: Serialize + Default + 'static>(&mut self) {
        let key = config_key::<T>();
        let config = T::default();
        match serde_json::to_value(config) {
            Ok(value) => {
                self.configs.insert(key, value);
            }
            Err(e) => error!("Failed to serialize config for key: {}", e),
        }
    }

    /// Loads config from disk, merging with registered defaults.
    /// Call this after all modules have registered their configs.
    pub fn finalize(&mut self, context: &Context) {
        let path =
            PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.toml");

        if path.exists() {
            let file_config: HashMap<String, Value> = Figment::new()
                .merge(Toml::file(&path))
                .extract()
                .inspect_err(|e| error!("Failed to load config file: {:?}", e))
                .unwrap_or_default();

            for (key, value) in file_config {
                if self.configs.contains_key(&key) {
                    if let Some(existing) = self.configs.get(&key) {
                        let merged = merge_json(existing, &value);
                        self.configs.insert(key, merged);
                    }
                } else {
                    self.configs.insert(key, value);
                }
            }
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }

        fs::write(&path, toml::to_string_pretty(self).unwrap_or_default())
            .inspect_err(|e| error!("Failed to write config: {}", e))
            .ok();

        CONFIG.set(Some(self.clone()));
    }
}

/// Merge two JSON values, preferring values from `b` when both exist.
/// For objects, recursively merges fields.
fn merge_json(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            let mut result = a_map.clone();
            for (key, b_val) in b_map {
                let a_val = result.get(key);
                let merged = match a_val {
                    Some(a_val) => merge_json(a_val, b_val),
                    None => b_val.clone(),
                };
                result.insert(key.clone(), merged);
            }
            Value::Object(result)
        }
        _ => b.clone(),
    }
}
