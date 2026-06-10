//! Mirror of the API gamemode enum.
//!
//! Matches Minecraft's four gamemodes exactly.

use crate::mirror_enum;

mirror_enum! {
    /// Mirror of the API gamemode enum.
    ///
    /// Matches Minecraft's four gamemodes exactly.
    pub enum GameMode from pumpkin_plugin_api::player::GameMode {
        Survival,
        Creative,
        Adventure,
        Spectator,
    }
}
