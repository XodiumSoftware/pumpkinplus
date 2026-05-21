//! Nickname module — set or remove player nicknames with JSON persistence.
//!
//! ## Commands
//!
//! | Command              | Aliases | Permission                      | Description              |
//! |----------------------|---------|---------------------------------|--------------------------|
//! | `/nickname [name]`   | `nick`  | `pumpkinplus:command.nickname` | Set or remove nickname   |
//!
//! ## Configuration
//!
//! | Field       | Default | Description                                         |
//! |-------------|---------|-----------------------------------------------------|
//! | `enabled`   | `false` | Whether this module is active                       |
//!
//! ## Mechanics
//!
//! - `/nickname` — clears the player's nickname.
//! - `/nickname <name>` — sets the player's nickname.
//! - Nicknames are persisted in `{data_folder}/nicknames.json`.
//! - On join, the stored nickname is applied to the player's display name and tab list name.
//! - A confirmation message is sent via the action bar.

use crate::{PLUGIN_ID, config::ConfigManager, module::Module};
use pumpkin_plugin_api::{
    Context, Server,
    command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
    command_wit::{ArgumentType, StringType},
    commands::CommandHandler,
    events::{EventData, EventHandler, EventPriority, PlayerJoinEvent},
    text::TextComponent,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use tracing::error;

use std::cell::RefCell;

// Thread-local storage for the data folder path used by NicknamesStore.
thread_local! {
    static DATA_FOLDER: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Stores and retrieves player nicknames from `{data_folder}/nicknames.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct NicknamesStore {
    nicknames: HashMap<String, String>,
}

impl NicknamesStore {
    fn path() -> PathBuf {
        let folder = DATA_FOLDER.with(|f| f.borrow().clone().unwrap_or_default());
        PathBuf::from(folder.trim_start_matches("./")).join("nicknames.json")
    }

    fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(
            &path,
            serde_json::to_string_pretty(self).unwrap_or_default(),
        )
        .inspect_err(|e| error!("Failed to write nicknames.json: {}", e))
        .ok();
    }

    fn get(&self, uuid: &str) -> Option<&String> {
        self.nicknames.get(uuid)
    }

    fn set(&mut self, uuid: &str, nickname: String) {
        if nickname.is_empty() {
            self.nicknames.remove(uuid);
        } else {
            self.nicknames.insert(uuid.to_string(), nickname);
        }
        self.save();
    }
}

/// Handles player nicknames.
#[derive(Default)]
pub struct Nickname;

impl Module for Nickname {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<NicknameConfig>().enabled)
            .unwrap_or(false)
    }

    fn cmds(&self) -> Vec<Command> {
        let command = Command::new(
            &["nickname".to_string(), "nick".to_string()],
            "Set or remove your nickname",
        );
        // /nickname <name> — sets nickname
        command.then(
            CommandNode::argument("name", &ArgumentType::String(StringType::Greedy))
                .execute(NicknameExecutor),
        );
        // /nickname clear — clears nickname
        command.then(CommandNode::literal("clear").execute(NicknameExecutor));
        vec![command]
    }

    fn perms(&self) -> HashSet<String> {
        HashSet::from([format!("{}:command.nickname", PLUGIN_ID)])
    }

    fn events(&self, context: &Context) {
        DATA_FOLDER.with(|f| *f.borrow_mut() = Some(context.get_data_folder().to_string()));

        context
            .register_event_handler::<PlayerJoinEvent, _>(Nickname, EventPriority::Normal, true)
            .expect("failed to register nickname join event handler");
    }
}

struct NicknameExecutor;

impl CommandHandler for NicknameExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.as_player().ok_or(CommandError::PermissionDenied)?;
        let uuid = format!("{}-{}", player.get_id().high, player.get_id().low);

        let mut store = NicknamesStore::load();

        // Try to get the "name" argument; if missing, treat as clear
        let arg = args.get_value("name");
        let nickname = match arg {
            pumpkin_plugin_api::command_wit::Arg::Simple(name) => name,
            _ => {
                store.set(&uuid, String::new());
                update_player(&player, None);
                sender.send_message(TextComponent::text("Nickname cleared."));
                return Ok(0);
            }
        };

        let trimmed = nickname.trim();
        if trimmed.is_empty() {
            store.set(&uuid, String::new());
            update_player(&player, None);
            sender.send_message(TextComponent::text("Nickname cleared."));
        } else {
            store.set(&uuid, trimmed.to_string());
            update_player(&player, Some(trimmed));
            sender.send_message(TextComponent::text(&format!(
                "Nickname updated to: {}",
                trimmed
            )));
        }

        Ok(0)
    }
}

impl EventHandler<PlayerJoinEvent> for Nickname {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        if !self.enabled() {
            return event;
        }

        let store = NicknamesStore::load();
        let uuid = format!(
            "{}-{}",
            event.player.get_id().high,
            event.player.get_id().low
        );

        if let Some(nickname) = store.get(&uuid) {
            update_player(&event.player, Some(nickname));
        }

        event
    }
}

/// Applies a nickname to a player's display name and tab list name.
fn update_player(player: &pumpkin_plugin_api::player::Player, nickname: Option<&str>) {
    let display = match nickname {
        Some(name) => TextComponent::text(name),
        None => TextComponent::text(&player.get_name()),
    };

    let tab_list = match nickname {
        Some(name) => TextComponent::text(name),
        None => TextComponent::text(&player.get_name()),
    };

    player.set_display_name(display);
    player.set_tab_list_name(Some(tab_list));
}

/// Configuration for the nickname mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NicknameConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
