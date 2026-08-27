//! Mirror of the API player interaction action enum.
//!
//! These are the actions that can trigger a player interact event.

use crate::mirror_enum;

mirror_enum! {
    /// Mirror of the API player interaction action enum.
    ///
    /// These are the actions that can trigger a player interact event.
    pub enum InteractAction from pumpkin_plugin_api::events::InteractAction {
        RightClickBlock,
        RightClickAir,
        LeftClickBlock,
        LeftClickAir,
    }
}
