//! Block state manipulation helpers.

use pumpkin_plugin_api::world::{BlockStateInfo, resolve_block_state};

/// Checks whether the block described by `info` is an openable structure
/// (door, trapdoor, or fence gate).
///
/// Uses name-based suffix matching instead of the typed [`BlockType`] registry
/// so that custom/modded openables are detected without hardcoding.
#[must_use]
pub fn is_openable(info: &BlockStateInfo) -> bool {
    let name = &info.name;
    name.ends_with("_door") || name.ends_with("_trapdoor") || name.ends_with("_fence_gate")
}

/// Checks whether the block described by `info` is a door specifically.
///
/// Doors support double-block sync; trapdoors and fence gates do not.
#[must_use]
pub fn is_door(info: &BlockStateInfo) -> bool {
    info.name.ends_with("_door")
}

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
