//! Condense module — convert items to/from their block forms.
//!
//! Allows players to condense 9 items into their block form (e.g., 9 diamonds
//! → 1 diamond block) and uncondense blocks back into items.
//!
//! ## Commands
//!
//! | Command | Aliases | Description |
//! |---------|---------|-------------|
//! | `/condense` | `cn` | Condense all applicable items into blocks |
//! | `/uncondense` | `ucn` | Uncondense all blocks into items |
//!
//! ## Condensable Items
//!
//! | Item | Block |
//! |------|-------|
//! | Amethyst Shard | Amethyst Block |
//! | Bone Meal | Bone Block |
//! | Coal | Coal Block |
//! | Copper Ingot | Copper Block |
//! | Diamond | Diamond Block |
//! | Dried Kelp | Dried Kelp Block |
//! | Emerald | Emerald Block |
//! | Gold Ingot | Gold Block |
//! | Gold Nugget | Gold Ingot |
//! | Iron Ingot | Iron Block |
//! | Iron Nugget | Iron Ingot |
//! | Lapis Lazuli | Lapis Block |
//! | Melon Slice | Melon |
//! | Nether Wart | Nether Wart Block |
//! | Netherite Ingot | Netherite Block |
//! | Quartz | Quartz Block |
//! | Redstone | Redstone Block |
//! | Slime Ball | Slime Block |
//! | Wheat | Hay Block |
//!
//! ## Status
//!
//! **Partial implementation** — Commands work but overflow handling is limited.
//!
//! ## Missing / Limited APIs
//!
//! | API | Status | Purpose |
//! |-----|--------|---------|
//! | `World.drop_item()` | ❌ Missing | Drop items when inventory is full |
//! | `Player.send_action_bar()` | ⚠️ Partial | Use `send_system_message` instead |
//! | Inventory `add_item()` | ⚠️ Manual | Must manually find empty slots |
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this module is active   |

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use pumpkin_plugin_api::command::{Command, CommandError, CommandSender};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Item, ItemStack, ItemStackExt, Server};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Number of items needed to condense into one block.
const CONDENSE_AMOUNT: u8 = 9;

/// Handles item condensing and uncondensing.
#[derive(Default)]
pub struct Condense;

impl Mechanic for Condense {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.condense.enabled)
    }

    fn cmds(&self) -> Vec<Command> {
        vec![
            Command::new(
                &["condense".to_string(), "cn".to_string()],
                "Condense all applicable items into blocks",
            )
            .execute(CondenseExecutor { reverse: false }),
            Command::new(
                &["uncondense".to_string(), "ucn".to_string()],
                "Uncondense all blocks into items",
            )
            .execute(CondenseExecutor { reverse: true }),
        ]
    }
}

/// Command executor for condense/uncondense.
struct CondenseExecutor {
    reverse: bool,
}

impl CommandHandler for CondenseExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.as_player().ok_or(CommandError::PermissionDenied)?;
        let total = condense_inventory(&player, self.reverse);

        let message = if total > 0 {
            format!(
                "{}Condensed {} item(s){}",
                if self.reverse { "Un" } else { "" },
                total,
                if self.reverse {
                    " from blocks"
                } else {
                    " into blocks"
                }
            )
        } else {
            format!(
                "Nothing to {}",
                if self.reverse {
                    "uncondense"
                } else {
                    "condense"
                }
            )
        };

        player.send_system_message(TextComponent::text(&message), false);
        Ok(1)
    }
}

/// Map of item to block for condensing, or block to item for uncondensing.
fn condensable_map(reverse: bool) -> HashMap<Item, Item> {
    let forward = [
        (Item::AmethystShard, Item::AmethystBlock),
        (Item::BoneMeal, Item::BoneBlock),
        (Item::Coal, Item::CoalBlock),
        (Item::CopperIngot, Item::CopperBlock),
        (Item::Diamond, Item::DiamondBlock),
        (Item::DriedKelp, Item::DriedKelpBlock),
        (Item::Emerald, Item::EmeraldBlock),
        (Item::GoldIngot, Item::GoldBlock),
        (Item::GoldNugget, Item::GoldIngot),
        (Item::IronIngot, Item::IronBlock),
        (Item::IronNugget, Item::IronIngot),
        (Item::LapisLazuli, Item::LapisBlock),
        (Item::MelonSlice, Item::Melon),
        (Item::NetherWart, Item::NetherWartBlock),
        (Item::NetheriteIngot, Item::NetheriteBlock),
        (Item::Quartz, Item::QuartzBlock),
        (Item::Redstone, Item::RedstoneBlock),
        (Item::SlimeBall, Item::SlimeBlock),
        (Item::Wheat, Item::HayBlock),
    ];

    if reverse {
        forward.iter().map(|(k, v)| (*v, *k)).collect()
    } else {
        forward.iter().copied().collect()
    }
}

/// Condenses or uncondenses all applicable items in the player's inventory.
///
/// Returns the total number of items condensed/uncondensed.
fn condense_inventory(player: &Player, reverse: bool) -> u32 {
    let map = condensable_map(reverse);
    let inventory = player.get_inventory().as_inventory();
    let size = inventory.get_size();
    let mut total = 0u32;

    for slot in 0..size {
        let Some(item) = inventory.get_item(slot) else {
            continue;
        };

        let Some(item_type) = item.get_item() else {
            continue;
        };

        let Some(&target) = map.get(&item_type) else {
            continue;
        };

        let count = item.get_count();
        let amount = count / CONDENSE_AMOUNT;
        if amount < 1 {
            continue;
        }

        let remaining = count - (amount * CONDENSE_AMOUNT);
        let produced = u32::from(amount);

        // Update the source slot
        if remaining > 0 {
            item.set_count(remaining);
        } else {
            inventory.set_item(slot, None);
        }

        // Add the target items (manual overflow handling — no drop_item API)
        add_items(&inventory, target, amount);

        total += produced;
    }

    total
}

/// Adds items to the inventory, dropping overflow.
///
/// # Limitations
///
/// Since `World.drop_item()` is not available, overflow items are discarded
/// instead of dropped. This is a limitation of the current Pumpkin API.
fn add_items(inventory: &pumpkin_plugin_api::inventory::Inventory, item: Item, mut amount: u8) {
    let max_stack = 64; // TODO: Get from item type
    let size = inventory.get_size();

    for slot in 0..size {
        if amount == 0 {
            break;
        }

        match inventory.get_item(slot) {
            None => {
                let add = amount.min(max_stack);
                inventory.set_item(slot, Some(ItemStack::new(item.resource_location(), add)));
                amount -= add;
            }
            Some(existing) => {
                if existing.get_item() == Some(item) {
                    let existing_count = existing.get_count();
                    let space = max_stack.saturating_sub(existing_count);
                    if space > 0 {
                        let add = amount.min(space);
                        existing.set_count(existing_count + add);
                        amount -= add;
                    }
                }
            }
        }
    }

    // TODO: Drop remaining items when World.drop_item() is available
    // For now, excess items are silently discarded
}

/// Configuration for the condense mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CondenseConfig {
    /// Whether this module is active.
    pub enabled: bool,
}
