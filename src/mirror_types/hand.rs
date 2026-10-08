//! Mirror of the API player hand enum.
//!
//! These are the hands a player can hold an item in.

use crate::mirror_enum;

mirror_enum! {
    /// Mirror of the API player hand enum.
    ///
    /// Used in config to select which hand a mechanic reads from.
    pub enum Hand from pumpkin_plugin_api::common::Hand {
        Left,
        Right,
    }
}
