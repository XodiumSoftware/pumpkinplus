//! Nickname module — set or remove player nicknames.
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
//! - Nicknames are persisted on the player's entity via `PersistentDataHolder`.
//! - On join, the stored nickname is applied to the player's display name and tab list name.

use crate::utils::command::default_permission;
use crate::utils::text::parse_legacy_text;
use crate::{PLUGIN_ID, config::ConfigManager, mechanics::mechanic::Mechanic};
use pumpkin_plugin_api::{
    Context, PersistentDataHolder, Server,
    command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
    command_wit::{Arg, ArgumentType, StringType},
    commands::CommandHandler,
    events::{EventData, EventHandler, EventPriority, PlayerJoinEvent},
    permission::Permission,
    player::Player,
    text::TextComponent,
};
use serde::{Deserialize, Serialize};

/// Plugin namespace for persistent data keys.
const DATA_NAMESPACE: &str = "pumpkinplus";
/// Persistent data key storing a player's nickname.
const NICKNAME_KEY: &str = "nickname";

/// Handles player nicknames.
#[derive(Default)]
pub struct Nickname;

impl Mechanic for Nickname {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.nickname.enabled)
    }

    fn cmds(&self) -> Vec<Command> {
        let command = Command::new(
            &["nickname".to_string(), "nick".to_string()],
            "Set or remove your nickname",
        )
        // /nickname <name> — sets nickname
        .then(
            CommandNode::argument("name", &ArgumentType::String(StringType::Greedy))
                .execute(NicknameExecutor),
        )
        // /nickname clear — clears nickname
        .then(CommandNode::literal("clear").execute(NicknameExecutor));
        vec![command]
    }

    fn perms(&self) -> Vec<Permission> {
        vec![default_permission(
            PLUGIN_ID,
            "nickname",
            "Allows using the /nickname and /nick commands.",
        )]
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerJoinEvent>(context, EventPriority::Normal, true);
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

        let (Arg::Simple(nickname) | Arg::Msg(nickname)) = args.get_value("name") else {
            player.remove_custom_data(DATA_NAMESPACE, NICKNAME_KEY);
            update_player(&player, None);
            sender.send_message(TextComponent::text("Nickname cleared."));
            return Ok(1);
        };

        let trimmed = nickname.trim();
        if trimmed.is_empty() {
            player.remove_custom_data(DATA_NAMESPACE, NICKNAME_KEY);
            update_player(&player, None);
            sender.send_message(TextComponent::text("Nickname cleared."));
        } else {
            player.set_string(DATA_NAMESPACE, NICKNAME_KEY, trimmed);
            update_player(&player, Some(trimmed));
            sender.send_message(TextComponent::text(&format!(
                "Nickname updated to: {trimmed}"
            )));
        }

        Ok(1)
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

        if let Some(nickname) = event.player.get_string(DATA_NAMESPACE, NICKNAME_KEY) {
            update_player(&event.player, Some(&nickname));
        }

        event
    }
}

/// Applies a nickname to a player's display name and tab list name.
fn update_player(player: &Player, nickname: Option<&str>) {
    let display = nickname.map_or_else(
        || TextComponent::text(&player.get_name()),
        parse_legacy_text,
    );

    let tab_list = nickname.map_or_else(
        || TextComponent::text(&player.get_name()),
        parse_legacy_text,
    );

    player.set_display_name(display);
    player.set_tab_list_name(Some(tab_list));
}

/// Configuration for the nickname mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NicknameConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
