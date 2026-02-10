mod config;

mod modules {
    pub mod module;
    pub mod enchantments {
        pub mod sample;
    }
    pub mod mechanics {
        pub mod motd;
        pub mod player;
    }
    pub mod recipes {
        pub mod sample;
    }
}

pub use config::*;
pub use enchantments::*;
pub use mechanics::*;
pub use modules::*;
pub use recipes::*;

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
