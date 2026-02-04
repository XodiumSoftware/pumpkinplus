use serde::{Deserialize, Serialize};

/// Represents a module handling motd mechanics within the system.
pub struct MotdModule {
    pub config: MotdModuleConfig,
}

impl MotdModule {
    pub fn new(config: MotdModuleConfig) -> Self {
        Self { config }
    }
}

/// Represents the config of the module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotdModuleConfig {
    pub join_msg: String,
    pub leave_msg: String,
}

impl Default for MotdModuleConfig {
    fn default() -> Self {
        Self {
            join_msg: "Welcome, {player}!".to_string(),
            leave_msg: "Goodbye, {player}!".to_string(),
        }
    }
}
