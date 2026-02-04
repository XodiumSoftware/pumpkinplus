mod config;
mod utils;

mod modules {
    pub mod player;
}

use pumpkin_api_macros::{plugin_impl, plugin_method};

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    server.init_log();

    log::info!("Hello, Pumpkin!");

    server
        .register_event(Arc::new(PlayerModule), EventPriority::Lowest, true)
        .await;

    Ok(())
}

/// IllyriaPlus plugin implementation.
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
