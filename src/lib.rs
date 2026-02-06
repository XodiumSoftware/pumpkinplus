mod config;

mod modules {
    pub mod module;
    pub mod motd;
    pub mod player;
}

use pumpkin_api_macros::{plugin_impl, plugin_method};

#[plugin_method]
async fn on_load(&mut self, ctx: Arc<Context>) -> Result<(), String> {
    ctx.init_log();

    let config = ConfigManager::new(ctx);

    ctx.register_event(
        Arc::new(PlayerModule { config }),
        EventPriority::Lowest,
        true,
    )
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
