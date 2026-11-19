//! Block state manipulation helpers.

use pumpkin_plugin_api::world::{Block, BlockStateInfo, resolve_block_state};

/// Checks whether the block described by `info` is an openable structure
/// (door, trapdoor, or fence gate).
///
/// Uses name-based suffix matching instead of the typed [`BlockType`] registry
/// so that custom/modded openables are detected without hardcoding.
#[must_use]
pub(crate) fn is_openable(info: &BlockStateInfo) -> bool {
    let name = &info.name;
    name.ends_with("_door") || name.ends_with("_trapdoor") || name.ends_with("_fence_gate")
}

/// Checks whether the block described by `info` is a door specifically.
///
/// Doors support double-block sync; trapdoors and fence gates do not.
#[must_use]
pub(crate) fn is_door(info: &BlockStateInfo) -> bool {
    info.name.ends_with("_door")
}

/// Returns the `(closed, open)` state ID pair of the door described by
/// `info`, keyed by the `open` property.
///
/// Resolves `open=true` through the API once and derives `open=false` from the
/// door's state list, so a double door needs a single API call instead of two.
/// Returns `None` if the block has no `open` property, the door does not have
/// exactly two states, or the open state cannot be resolved.
#[must_use]
pub(crate) fn door_open_pair(info: &BlockStateInfo) -> Option<(u16, u16)> {
    if !info.properties.iter().any(|(key, _)| key == "open") {
        return None;
    }

    let mut properties = info.properties.clone();
    let (_, value) = properties.iter_mut().find(|(key, _)| key == "open")?;
    value.clear();
    value.push_str("true");

    let open_id = resolve_block_state(&info.name, &properties)?;
    let block = Block::from_state_id(open_id)?;
    if block.state_ids.len() != 2 {
        return None;
    }
    let closed_id = block.state_ids.into_iter().find(|&id| id != open_id)?;

    Some((closed_id, open_id))
}
