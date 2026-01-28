use pumpkin_api_macros::plugin_impl;

#[plugin_impl]
pub struct IllyriaPlus {}

impl IllyriaPlus {
    pub fn new() -> Self {
        IllyriaPlus {}
    }
}

impl Default for IllyriaPlus {
    fn default() -> Self {
        Self::new()
    }
}
