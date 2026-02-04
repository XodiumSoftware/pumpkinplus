use std::env;
use std::path::PathBuf;

/// A utility struct for common helper functions.
pub struct Utils;

impl Utils {
    /// Returns a path to a file in the same directory as the current executable.
    ///
    /// # Arguments
    /// * `filename` - The name of the file to locate (e.g., "config.toml")
    ///
    /// # Returns
    /// A `PathBuf` pointing to the specified file in the executable's directory
    pub fn get_exec_path(filename: &str) -> PathBuf {
        env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Failed to get executable directory")
            .join(filename)
    }
}
