use crate::module::Module;
use serde::{Deserialize, Serialize};

const MIN_TPS: f64 = 0.0;
const MAX_TPS: f64 = 20.0;
const TPS_DECIMAL_FORMAT: &str = "%.1f";
const MAX_COLOR_VALUE: u8 = 255;
const COLOR_FORMAT: &str = "#%02X%02X%02X";

/// Handles tab-list mechanics, including custom messages.
#[derive(Default)]
pub struct Tablist {
    /// Configuration for this module.
    config: Config,
}

impl Module for Tablist {
    fn enabled(&self) -> bool {
        self.config.enabled
    }
}

/// Configuration for the player mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { enabled: false }
    }
}
