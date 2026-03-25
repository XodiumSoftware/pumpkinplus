use crate::player::Config;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Manages plugin configuration, loading from and saving to disk.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager {
    /// Configuration for the player mechanics module.
    pub player_module: Config,
}

impl ConfigManager {
    /// Loads the configuration from disk, or writes and returns the default if not found.
    ///
    /// # Errors
    /// Returns an [`io::Error`] if an I/O or deserialization error occurs.
    pub fn new(context: &Context) -> Result<Self, io::Error> {
        let path = Self::path(context);

        match Self::load(&path) {
            Ok(config) => Ok(config),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let default_config = ConfigManager::default();
                default_config.save(&path)?;
                Ok(default_config)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the path to the configuration file within the plugin's data folder.
    fn path(context: &Context) -> PathBuf {
        PathBuf::from(context.get_data_folder()).join("config.json")
    }

    /// Reads and deserializes the configuration from `path`.
    ///
    /// # Errors
    /// Returns [`io::ErrorKind::NotFound`] if the file does not exist, or
    /// [`io::ErrorKind::InvalidData`] if deserialization fails.
    fn load(path: &Path) -> Result<ConfigManager, io::Error> {
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Config file not found at: {}", path.display()),
            ));
        }

        let content = fs::read_to_string(path)?;
        let config: ConfigManager = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }

    /// Serializes and writes the configuration to `path`, creating parent directories if needed.
    ///
    /// # Errors
    /// Returns an [`io::Error`] if serialization or any file operation fails.
    fn save(&self, path: &Path) -> Result<(), io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        fs::write(path, content)?;
        Ok(())
    }
}
