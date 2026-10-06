//! Nickname module — set player nicknames via an interactive dialog.
//!
//! ## Commands
//!
//! | Command              | Aliases | Permission                      | Description              |
//! |----------------------|---------|---------------------------------|--------------------------|
//! | `/nickname`          | `nick`  | `pumpkinplus:command.nickname` | Open nickname dialog     |
//!
//! ## Configuration
//!
//! | Field       | Default | Description                                         |
//! |-------------|---------|-----------------------------------------------------|
//! | `enabled`   | `false` | Whether this module is active                       |
//!
//! ## Mechanics
//!
//! - `/nickname` — opens an interactive dialog to set nickname (Java only).
//! - Nicknames support [MiniMessage](https://docs.advntr.dev/minimessage/format.html)
//!   formatting tags (e.g. `<red><bold>Illyrius`).
//! - Nicknames are persisted on the player's entity via `PersistentDataHolder`.
//! - On join, the stored nickname is applied to the player's display name and tab list name.
//!
//! ## Platform Support
//!
//! The dialog only works for Java Edition players. Bedrock players will
//! receive a chat message explaining dialogs are not supported.

use crate::dialogs::dialog::DialogBuilder;
use crate::dialogs::nickname::NicknameDialog;
use crate::utils::command::default_permission;
use crate::utils::text::parse_minimessage;
use crate::{PLUGIN_ID, config::ConfigManager, mechanics::mechanic::Mechanic};
use pumpkin_plugin_api::{
    Context, PersistentDataHolder, Server,
    command::{Command, CommandError, CommandSender},
    commands::CommandHandler,
    events::{DialogClickActionEvent, EventData, EventHandler, EventPriority, PlayerJoinEvent},
    permission::Permission,
    player::Player,
    text::TextComponent,
};
use serde::{Deserialize, Serialize};

/// Plugin namespace for persistent data keys.
pub const DATA_NAMESPACE: &str = "pumpkinplus";
/// Persistent data key storing a player's nickname.
pub const NICKNAME_KEY: &str = "nickname";

/// Handles player nicknames.
#[derive(Default)]
pub struct Nickname;

impl Mechanic for Nickname {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.nickname.enabled)
    }

    fn cmds(&self) -> Vec<Command> {
        vec![
            Command::new(
                &["nickname".to_string(), "nick".to_string()],
                "Open nickname dialog",
            )
            .execute(NicknameExecutor),
        ]
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
        self.register_event::<DialogClickActionEvent>(context, EventPriority::Normal, true);
    }
}

/// Command executor that shows the nickname dialog.
struct NicknameExecutor;

impl CommandHandler for NicknameExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.as_player().ok_or(CommandError::PermissionDenied)?;
        NicknameDialog.show(&player);
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

impl EventHandler<DialogClickActionEvent> for Nickname {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<DialogClickActionEvent>,
    ) -> EventData<DialogClickActionEvent> {
        if !self.enabled() {
            return event;
        }

        let player = &event.player;
        let action_id = &event.id;

        // Check if this is our nickname dialog save action
        if action_id == crate::dialogs::nickname::NICKNAME_SAVE_ACTION {
            let payload_ref = event.payload.as_deref();
            crate::dialogs::nickname::handle_save_action(player, payload_ref);
            event.cancelled = true;
        }

        event
    }
}

/// Applies a nickname to a player's display name and tab list name.
pub fn update_player(player: &Player, nickname: Option<&str>) {
    let display = nickname.map_or_else(
        || TextComponent::text(&player.get_name()),
        parse_minimessage,
    );

    let tab_list = nickname.map_or_else(
        || TextComponent::text(&player.get_name()),
        parse_minimessage,
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
