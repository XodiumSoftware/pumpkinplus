//! Block state manipulation helpers.

use pumpkin_plugin_api::world::{BlockStateInfo, resolve_block_state};

/// Returns the state ID of a block identical to `info` but with its `open`
/// property toggled (`true` ↔ `false`).
///
/// Returns `None` if the block has no `open` property or if the resulting
/// property set cannot be resolved back to a valid state ID.
#[must_use]
pub fn toggle_open_property(info: &BlockStateInfo) -> Option<u16> {
    let mut properties = info.properties.clone();
    for (key, value) in &mut properties {
        if key == "open" {
            *value = if value == "true" {
                "false".into()
            } else {
                "true".into()
            };
            break;
        }
    }

    resolve_block_state(&info.name, &properties)
}
