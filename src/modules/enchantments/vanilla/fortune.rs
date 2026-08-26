//! Fortune enchantment behavior override.
//!
//! Auto-replants mature ageable crops when they are broken with a hoe that has
//! Fortune at least 2. The crop is reset to age 0 after a short delay,
//! preserving the natural drops from the break.
//!
//! ## Configuration
//!
//! | Field     | Default | Description                                  |
//! |-----------|---------|----------------------------------------------|
//! | `fortune` | `false` | Whether auto-replanting via Fortune is active |

use crate::config::ConfigManager;
use crate::modules::enchantments::enchantment::Enchantment;
use pumpkin_plugin_api::Enchantment as VanillaEnchantment;
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::events::{BlockBreakEvent, EventData, EventHandler, EventPriority};
use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::world::{BlockFlags, BlockPos, World};
use pumpkin_plugin_api::{Context, Server};
use tracing::info;

/// Minimum Fortune level required to trigger auto-replanting.
const MIN_FORTUNE_LEVEL: u32 = 2;

/// Handles Fortune-based crop auto-replanting.
#[derive(Default)]
pub struct Fortune;

impl Enchantment for Fortune {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.enchantments.fortune)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<BlockBreakEvent, _>(Fortune, EventPriority::Normal, true)
            .expect("failed to register Fortune enchantment event handler");
    }
}

impl EventHandler<BlockBreakEvent> for Fortune {
    fn handle(
        &self,
        server: Server,
        event: EventData<BlockBreakEvent>,
    ) -> EventData<BlockBreakEvent> {
        if !self.enabled() || event.cancelled {
            return event;
        }

        let Some(ref player) = event.player else {
            return event;
        };

        let Some(item) = player.get_item_in_hand(Hand::Right) else {
            return event;
        };

        if !is_hoe(&item) {
            return event;
        }

        let fortune_level = item
            .get_enchantments()
            .iter()
            .find(|e| e.enchantment == VanillaEnchantment::Fortune)
            .map_or(0, |e| e.level);
        if fortune_level < MIN_FORTUNE_LEVEL {
            return event;
        }

        let world = player.get_world();
        let pos = event.block_pos;

        let Some(ageable) = try_parse_ageable(&world, pos) else {
            return event;
        };
        if ageable.age < ageable.max_age {
            return event;
        }

        let base_state = i32::from(ageable.base_state_id);
        let replant_state =
            u16::try_from(base_state + ageable.age_zero_offset).unwrap_or(ageable.base_state_id);

        info!(
            "[Fortune] Auto-replanting crop at {:?} (Fortune {})",
            pos, fortune_level
        );

        server.schedule_delayed_task(2, move |_server| {
            let flags = BlockFlags::NOTIFY_NEIGHBORS | BlockFlags::NOTIFY_LISTENERS;
            world.set_block_state(pos, replant_state, flags);
        });

        event
    }
}

/// Quick check for hoe-like item registry keys.
fn is_hoe(item: &pumpkin_plugin_api::ItemStack) -> bool {
    let id = item.get_registry_key();
    id == "minecraft:wooden_hoe"
        || id == "minecraft:stone_hoe"
        || id == "minecraft:iron_hoe"
        || id == "minecraft:golden_hoe"
        || id == "minecraft:diamond_hoe"
        || id == "minecraft:netherite_hoe"
}

/// Parsed ageable crop state information.
struct AgeableInfo {
    /// Block state ID of the crop at its current age.
    base_state_id: u16,
    /// Current age of the crop.
    age: u16,
    /// Maximum age the crop can reach.
    max_age: u16,
    /// Offset to add to the base state ID to reach age 0.
    age_zero_offset: i32,
}

/// Attempts to parse an ageable crop from the block state at `pos`.
///
/// Returns `Some(AgeableInfo)` if the block is a supported crop with an `age`
/// property, or `None` otherwise.
fn try_parse_ageable(world: &World, pos: BlockPos) -> Option<AgeableInfo> {
    let state_id = world.get_block_state_id(pos);
    let info = pumpkin_plugin_api::world::block_state_to_info(state_id)?;

    // Supported crops and their vanilla maximum ages.
    let max_age = match info.name.as_str() {
        "wheat" | "carrots" | "potatoes" | "beetroots" => 7,
        "nether_wart" => 3,
        "torchflower_crop" | "pitcher_crop" => 0, // handled differently; ignore
        _ => return None,
    };

    // Extract the `age` property from the block state's properties map.
    let age = info
        .properties
        .iter()
        .find(|(k, _)| k == "age")
        .and_then(|(_, v)| v.parse::<u16>().ok())?;

    // The state ID for age 0 is assumed to be `state_id - age` because vanilla
    // ageable block states are laid out contiguously by age.
    let age_zero_offset = -i32::from(age);

    Some(AgeableInfo {
        base_state_id: state_id,
        age,
        max_age,
        age_zero_offset,
    })
}
