use crate::modules::player::PlayerModuleConfig;
use crate::utils::Utils;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ConfigManager {
    //TODO: collect all configs in here and write/load them to a json file in plugin root (use pumpkin api).
    pub player_module: PlayerModuleConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self::load_or_create(&Utils::get_exec_path("config.toml"))
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
            .then(|| Figment::from(Toml::file(path)).extract::<Self>().ok())
            .flatten()
            .unwrap_or_else(|| {
                let config = Self::default();
                config.save_to_file(path).unwrap();
                config
            })
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
        fs::write(
            path,
            toml::to_string_pretty(self).expect("Failed to serialize config to TOML"),
        )
    }
}
