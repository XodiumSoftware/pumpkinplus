//! Embertread custom enchantment.
//!
//! Feet-slot enchantment that cancels contact and fire damage while worn.
//!
//! ## Configuration
//!
//! | Field        | Default | Description                                      |
//! |--------------|---------|--------------------------------------------------|
//! | `embertread` | `false` | Whether Embertread damage cancellation is active |
//!
//! ## Behavior
//!
//! Registers a level-1 custom enchantment active in the feet slot. When an entity
//! wearing foot armor with Embertread takes contact or fire damage, the damage is
//! cancelled.

use crate::config::ConfigManager;
use crate::modules::enchantments::enchantment::Enchantment;
use crate::namespaced_id;
use pumpkin_plugin_api::DamageType;
use pumpkin_plugin_api::Player;
use pumpkin_plugin_api::enchantment::{AttributeModifierSlot, EnchantmentBuilder};
use pumpkin_plugin_api::events::{EntityDamageEvent, EventData, EventHandler, EventPriority};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};
use tracing::debug;

/// Unique identifier for the Embertread enchantment.
const EMBERTREAD_ID: &str = namespaced_id!("embertread");

/// Handles the Embertread custom enchantment.
#[derive(Default)]
pub struct Embertread;

impl Enchantment for Embertread {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.enchantments.embertread)
    }

    fn enchantment(&self) -> EnchantmentBuilder {
        EnchantmentBuilder::new(EMBERTREAD_ID, TextComponent::text("Embertread"))
            .max_level(1)
            .anvil_cost(2)
            .supported_items("#minecraft:enchantable/foot_armor")
            .weight(2)
            .slots([AttributeModifierSlot::Feet])
    }

    fn events(&self, context: &Context) {
        self.register_event::<EntityDamageEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<EntityDamageEvent> for Embertread {
    fn handle(
        &self,
        server: Server,
        mut event: EventData<EntityDamageEvent>,
    ) -> EventData<EntityDamageEvent> {
        if event.cancelled || !is_contact_or_fire(event.damage_type) {
            return event;
        }

        let Some(player) = find_player_by_entity_id(&server, event.entity_id) else {
            return event;
        };

        let Some(boots) = player.get_inventory().get_boots() else {
            return event;
        };

        if !boots.has_custom_enchantment(EMBERTREAD_ID) {
            return event;
        }

        debug!(
            "[Embertread] Cancelled {:?} damage for {}",
            event.damage_type,
            player.get_name()
        );
        event.cancelled = true;
        event
    }
}

/// Returns `true` if the given damage type is contact or fire damage.
fn is_contact_or_fire(damage_type: DamageType) -> bool {
    matches!(
        damage_type,
        DamageType::Cactus
            | DamageType::Campfire
            | DamageType::HotFloor
            | DamageType::InFire
            | DamageType::Lava
            | DamageType::OnFire
            | DamageType::Stalagmite
            | DamageType::Sting
            | DamageType::SulfurCubeHot
            | DamageType::SweetBerryBush
    )
}

/// Finds an online player by their entity id.
fn find_player_by_entity_id(server: &Server, entity_id: i32) -> Option<Player> {
    let target_id = u32::try_from(entity_id).unwrap_or(u32::MAX);
    server
        .get_all_players()
        .into_iter()
        .find(|player| player.as_entity().get_id() == target_id)
}
