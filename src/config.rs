use crate::modules::player::Config;
use pumpkin::plugin::Context;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io};

/// The `ConfigManager` struct is responsible for managing configuration settings
/// related to the application's player module. It acts as a container for
/// the `PlayerModuleConfig` configuration.
///
/// # Attributes
/// * `player_module` - A public field of type `PlayerModuleConfig` that holds
///   the configuration details for the player module.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager {
    pub player_module: Config,
}

impl ConfigManager {
    /// Creates a new instance of the configuration.
    ///
    /// # Arguments
    /// * `ctx` - An `Arc<Context>` instance, which provides the necessary context
    ///           to determine the file path for the configuration.
    ///
    /// # Returns
    /// * `Ok(Self)` - If the configuration is successfully loaded or created as default.
    /// * `Err(io::Error)` - If an I/O error occurs during the loading or saving process,
    ///                      other than a `NotFound` error.
    pub fn new(ctx: Arc<Context>) -> Result<Self, io::Error> {
        let path = Self::path(ctx.clone());

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

    /// Generates the file path for the configuration file.
    ///
    /// # Arguments
    /// * `ctx` - An `Arc<Context>` object that provides access to the application's
    ///           data folder through its `get_data_folder` method.
    fn path(ctx: Arc<Context>) -> PathBuf {
        ctx.get_data_folder().join("config.json")
    }

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

    /// Saves the current object to the specified file path in a JSON format.
    ///
    /// # Parameters
    /// - `path`: A reference to a [`Path`] where the object should be saved.
    ///
    /// # Returns
    /// - `Result<(), io::Error>`:
    ///     - `Ok(())` if the object is successfully saved.
    ///     - An `io::Error` if an error occurs during the serialization or file operations.
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
