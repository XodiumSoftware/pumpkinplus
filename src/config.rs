use crate::modules::player::PlayerModuleConfig;
use crate::utils::Utils;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ConfigManager {
    //TODO: collect all configs in here and write/load them to a json file in plugin root (use pumpkin api).
    pub player_module: PlayerModuleConfig,
    plugin_name: String,
    server_root: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self::load_or_create(&Utils::get_exec_path("config.json"))
    }

    /// Loads the configuration from a file, creating a default one if it doesn't exist.
    ///
    /// # Arguments
    /// * `path` - The path to the configuration file.
    ///
    /// # Returns
    /// A `Self` instance containing the loaded or default configuration.
    pub fn load_or_create(path: &PathBuf) -> Self {
        path.exists()
            .then(|| Self::load_from_file(path).ok())
            .flatten()
            .unwrap_or_else(|| {
                let config = Self::default();
                config.save_to_file(path).unwrap();
                config
            })
    }

    /// Loads the configuration from a JSON file.
    ///
    /// # Arguments
    /// * `path` - Path to the JSON configuration file.
    ///
    /// # Returns
    /// A `std::io::Result<Self>` containing the loaded configuration.
    pub fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        serde_json::from_str::<Self>(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Saves the current configuration to a file.
    ///
    /// # Arguments
    /// * `path` - The path to the configuration file.
    ///
    /// # Returns
    /// A `std::io::Result<()>` indicating success or failure.
    pub fn save_to_file(&self, path: &PathBuf) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        fs::write(path, json)
    }
}
