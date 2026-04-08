use figment::Figment;
use figment::providers::{Format, Json};
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
pub fn config_key<T>() -> &'static str {
    use std::any::type_name;
    use std::sync::OnceLock;

    static CACHE: OnceLock<std::sync::Mutex<HashMap<String, &'static str>>> = OnceLock::new();

    let cache = CACHE.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
    let full_name = type_name::<T>();

    if let Ok(map) = cache.lock() {
        if let Some(&key) = map.get(full_name) {
            return key;
        }
    }

    let parts: Vec<&str> = full_name.split("::").collect();
    let key = if parts.len() >= 2 {
        parts[parts.len() - 2]
    } else if let Some(&last) = parts.last() {
        if last.ends_with("Config") {
            &last[..last.len() - 6]
        } else {
            last
        }
    } else {
        full_name
    };

    let key: &'static str = Box::leak(key.to_string().into_boxed_str());

    if let Ok(mut map) = cache.lock() {
        map.insert(full_name.to_string(), key);
    }

    key
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
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    /// Registers a config with default values for a module.
    /// The key is derived automatically from the type name.
    pub fn register<T: Serialize + Default + 'static>(&mut self) {
        let key = config_key::<T>();
        let config = T::default();
        self.configs
            .insert(key.to_string(), serde_json::to_value(config).unwrap());
    }

    /// Loads config from disk, merging with registered defaults.
    /// Call this after all modules have registered their configs.
    pub fn finalize(&mut self, context: &Context) {
        let path =
            PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.json");

        if path.exists() {
            let file_config: HashMap<String, Value> = Figment::new()
                .merge(Json::file(&path))
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

        fs::write(
            &path,
            serde_json::to_string_pretty(self).unwrap_or_default(),
        )
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
