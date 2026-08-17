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

use crate::{PLUGIN_ID, config::ConfigManager, mechanics::mechanic::Mechanic};
use pumpkin_plugin_api::{
    Context, Server,
    command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
    command_wit::{Arg, ArgumentType, StringType},
    commands::CommandHandler,
    events::{EventData, EventHandler, EventPriority, PlayerJoinEvent},
    permission::{Permission, PermissionDefault},
    player::Player,
    text::{NamedColor, TextComponent},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::error;

use std::cell::RefCell;

thread_local! {
    static DATA_FOLDER: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Stores and retrieves player nicknames from `{data_folder}/nicknames.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct NicknamesStore {
    nicknames: HashMap<String, String>,
}

impl NicknamesStore {
    /// Builds the file path for `nicknames.json` inside the given data folder.
    fn path(data_folder: &str) -> PathBuf {
        PathBuf::from(data_folder.trim_start_matches("./")).join("nicknames.json")
    }

    fn load(data_folder: &str) -> Self {
        let path = Self::path(data_folder);
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn save(&self, data_folder: &str) {
        let path = Self::path(data_folder);
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

    fn set(&mut self, data_folder: &str, uuid: &str, nickname: String) {
        if nickname.is_empty() {
            self.nicknames.remove(uuid);
        } else {
            self.nicknames.insert(uuid.to_string(), nickname);
        }
        self.save(data_folder);
    }
}

/// Handles player nicknames.
#[derive(Default)]
pub struct Nickname;

impl Mechanic for Nickname {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.nickname.enabled)
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

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: format!("{PLUGIN_ID}:command.nickname"),
            description: "Allows using the /nickname and /nick commands.".to_string(),
            default: PermissionDefault::Allow,
            children: vec![],
        }]
    }

    fn events(&self, context: &Context) {
        DATA_FOLDER.with(|f| {
            *f.borrow_mut() = Some(context.get_data_folder().clone());
        });

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

        let data_folder = DATA_FOLDER.with(|f| f.borrow().clone().unwrap_or_default());
        let mut store = NicknamesStore::load(&data_folder);

        let (Arg::Simple(nickname) | Arg::Msg(nickname)) = args.get_value("name") else {
            store.set(&data_folder, &uuid, String::new());
            update_player(&player, None);
            sender.send_message(TextComponent::text("Nickname cleared."));
            return Ok(0);
        };

        let trimmed = nickname.trim();
        if trimmed.is_empty() {
            store.set(&data_folder, &uuid, String::new());
            update_player(&player, None);
            sender.send_message(TextComponent::text("Nickname cleared."));
        } else {
            store.set(&data_folder, &uuid, trimmed.to_string());
            update_player(&player, Some(trimmed));
            sender.send_message(TextComponent::text(&format!(
                "Nickname updated to: {trimmed}"
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

        let data_folder = DATA_FOLDER.with(|f| f.borrow().clone().unwrap_or_default());
        let store = NicknamesStore::load(&data_folder);
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

/// Parses a string containing legacy `&` color/formatting codes and returns a
/// `TextComponent` with styled children. A plain text component is returned if
/// no codes are present.
///
/// Supported codes match the standard Minecraft color/formatting codes:
/// `0-9`, `a-f` for colors; `k` obfuscated, `l` bold, `m` strikethrough,
/// `n` underlined, `o` italic, `r` reset.
fn parse_legacy_text(input: &str) -> TextComponent {
    let root = TextComponent::text("");
    let mut current_text = String::new();
    let mut current = TextComponent::text("");
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '&' {
            // Flush accumulated text before applying the new code.
            if !current_text.is_empty() {
                current.add_text(&current_text);
                current_text.clear();
                root.add_child(current);
                current = TextComponent::text("");
            }

            let Some(code) = chars.next() else {
                break;
            };

            let code_lower = code.to_ascii_lowercase();
            match code_lower {
                'r' => {
                    current = TextComponent::text("");
                }
                'k' => {
                    current.obfuscated(true);
                }
                'l' => {
                    current.bold(true);
                }
                'm' => {
                    current.strikethrough(true);
                }
                'n' => {
                    current.underlined(true);
                }
                'o' => {
                    current.italic(true);
                }
                _ => {
                    if let Some(color) = color_from_code(code_lower) {
                        current = TextComponent::text("");
                        current.color_named(color);
                    }
                    // Unknown codes are ignored.
                }
            }
        } else {
            current_text.push(ch);
        }
    }

    if !current_text.is_empty() {
        current.add_text(&current_text);
    }
    // Only add the final child if it has text or styling.
    if !current.get_text().is_empty() || has_style(&current) {
        root.add_child(current);
    }

    root
}

/// Maps a legacy formatting code character to a `NamedColor`.
fn color_from_code(code: char) -> Option<NamedColor> {
    Some(match code {
        '0' => NamedColor::Black,
        '1' => NamedColor::DarkBlue,
        '2' => NamedColor::DarkGreen,
        '3' => NamedColor::DarkAqua,
        '4' => NamedColor::DarkRed,
        '5' => NamedColor::DarkPurple,
        '6' => NamedColor::Gold,
        '7' => NamedColor::Gray,
        '8' => NamedColor::DarkGray,
        '9' => NamedColor::Blue,
        'a' => NamedColor::Green,
        'b' => NamedColor::Aqua,
        'c' => NamedColor::Red,
        'd' => NamedColor::LightPurple,
        'e' => NamedColor::Yellow,
        'f' => NamedColor::White,
        _ => return None,
    })
}

/// Returns true if the text component has any style applied.
fn has_style(component: &TextComponent) -> bool {
    // TextComponent doesn't expose style getters, so we infer it from the
    // encoded NBT length. This is a heuristic; a better API would expose style
    // flags directly.
    component.encode().len() > 1
}

/// Applies a nickname to a player's display name and tab list name.
fn update_player(player: &Player, nickname: Option<&str>) {
    let display = match nickname {
        Some(name) => parse_legacy_text(name),
        None => TextComponent::text(&player.get_name()),
    };

    let tab_list = match nickname {
        Some(name) => parse_legacy_text(name),
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
