//! Player-related helper functions.

use pumpkin_plugin_api::player::Player;

/// Returns a stable string identifier for a player from their UUID.
///
/// The UUID is represented as `{high}-{low}` using the two 64-bit halves
/// exposed by the plugin API.
#[must_use]
#[expect(dead_code)]
pub fn uuid_string(player: &Player) -> String {
    format!("{}-{}", player.get_id().high, player.get_id().low)
}
