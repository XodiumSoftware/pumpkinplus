use crate::player::Config;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::PathBuf;
use tracing::error;

/// Manages plugin configuration, loading from and saving to disk.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager {
    /// Configuration for the player mechanics module.
    pub player_module: Config,
}

impl ConfigManager {
    /// Creates a new [`ConfigManager`], loading from disk if it exists,
    /// otherwise writing defaults first.
    pub fn new(context: &Context) -> Self {
        let mut manager = Self::default();
        manager.read_and_update(context);
        manager.write(context);
        manager
    }

    /// Reads and deserializes the configuration from disk.
    fn read(&self, context: &Context) -> Result<ConfigManager, String> {
        match fs::read_to_string(Self::path(context)) {
            Ok(content) => match serde_json::from_str::<ConfigManager>(&content) {
                Ok(data) => Ok(data),
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    /// Reads the configuration from disk and updates the current instance in place.
    fn read_and_update(&mut self, context: &Context) {
        match self.read(context) {
            Ok(data) => *self = data,
            Err(err) => error!("{}", err),
        }
    }

    /// Serializes and writes the configuration to disk, creating the directory if needed.
    fn write(&self, context: &Context) {
        let path = Self::path(context);

        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
        {
            Ok(mut file) => match serde_json::to_string_pretty(self) {
                Ok(json) => {
                    if let Err(err) = file.write_all(json.as_bytes()) {
                        error!("{}", err);
                    }
                }
                Err(err) => error!("{}", err),
            },
            Err(err) => error!("{}", err),
        }
    }

    /// Returns the path to the configuration file within the plugin's data folder.
    fn path(context: &Context) -> PathBuf {
        PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.json")
    }
}
