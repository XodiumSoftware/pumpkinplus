//! Silence module — toggle mob silent state using an amethyst shard.
//!
//! When a player right-clicks a living, non-player, non-monster entity while
//! holding an amethyst shard (or the configured item), the mob's silent state is
//! toggled. A dust particle effect is shown around the target: red when
//! silencing, green when un-silencing. The shard is consumed in survival and
//! adventure gamemodes.
//!
//! ## Implementation notes
//!
//! Pumpkin's high-level `World::spawn_particle` call does not carry dust color
//! data, so colored particles are sent with the raw Java `c-particle` packet.
//! For non-Java players the mechanic falls back to a plain `Dust` particle so
//! the visual still fires, just without the red/green tint.
//!
//! ## Permissions
//!
//! | Node | Default | Description |
//! |------|---------|-------------|
//! | `pumpkinplus:silence.use` | allow | Allows silencing mobs with the configured tool item |
//!
//! ## Configuration
//!
//! | Field       | Default                       | Description                                    |
//! |-------------|-------------------------------|------------------------------------------------|
//! | `enabled`   | `false`                       | Whether this module is active                  |
//! | `tool_item` | `"minecraft:amethyst_shard"`  | Registry key of the item used to toggle silence |
//! | `hand`      | `["Right"]`                   | Hands the tool item can be held in             |
//!
//! `IllyriaPlus` behavior reference:
//! - On `PlayerInteractEntityEvent`: if holding amethyst shard and target is
//!   a living, non-player, non-monster entity, toggle silent state
//! - Show red dust when silencing, green dust when un-silencing
//! - Consume one shard in survival/adventure modes

use pumpkin_plugin_api::common::Position;
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerInteractEntityEvent,
};
use pumpkin_plugin_api::java_packets::{CParticle, ClientboundPacket};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::wit::pumpkin::plugin::event::EntityInteractionAction;
use pumpkin_plugin_api::world::{EntityType, Particle};
use pumpkin_plugin_api::{Context, Item, ItemStackExt, Server};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::config::ConfigManager;
use crate::mechanics::mechanic::Mechanic;
use crate::{GameMode, MirrorHand};

/// Permission node required to silence mobs with the configured tool item.
pub const PERM_SILENCE: &str = concat!(env!("CARGO_PKG_NAME"), ":silence.use");

/// Number of dust particles spawned around the target entity.
const PARTICLE_COUNT: i32 = 10;
/// Maximum speed of the spawned dust particles.
const PARTICLE_MAX_SPEED: f32 = 0.01;
/// Offset of the spawned dust particle cloud.
const PARTICLE_OFFSET: Position = (0.5, 0.6, 0.5);

/// Handles mob silencing with amethyst shards.
#[derive(Default)]
pub struct Silence;

impl Mechanic for Silence {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.mechanics.silence.enabled)
    }

    fn perms(&self) -> Vec<Permission> {
        vec![Permission {
            node: PERM_SILENCE.into(),
            description: "Allows toggling mob silence with the configured tool item.".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        }]
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerInteractEntityEvent>(context, EventPriority::Normal, true);
    }
}

impl EventHandler<PlayerInteractEntityEvent> for Silence {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerInteractEntityEvent>,
    ) -> EventData<PlayerInteractEntityEvent> {
        let config: SilenceConfig = ConfigManager::get()
            .map(|cm| cm.mechanics.silence)
            .unwrap_or_default();

        if !config.enabled {
            return event;
        }

        if event.action != EntityInteractionAction::Interact {
            return event;
        }

        let Some(tool) = Item::from_registry_key(&config.tool_item) else {
            return event;
        };

        let tool_match = [MirrorHand::Right, MirrorHand::Left]
            .into_iter()
            .filter(|h| h.matches_config(&config.hand))
            .find_map(|h| {
                event
                    .player
                    .get_item_in_hand(h.into())
                    .filter(|stack| stack.is_item(tool))
                    .map(|stack| (h, stack))
            });

        let Some((hand, stack)) = tool_match else {
            return event;
        };

        if !event.player.has_permission(PERM_SILENCE) {
            return event;
        }

        let Ok(target_id) = u32::try_from(event.entity_id) else {
            return event;
        };

        let world = event.player.get_world();
        let Some(entity) = world
            .get_entities()
            .into_iter()
            .find(|e| e.get_id() == target_id)
        else {
            return event;
        };

        if !entity.is_living()
            || entity.get_type() == EntityType::Player
            || is_monster(entity.get_type())
        {
            return event;
        }

        let now_silent = !entity.is_silent();
        entity.set_silent(now_silent);

        let pos = entity.get_position();

        if let Some(java) = event.player.as_java() {
            // Java clients get a properly colored dust particle via the raw
            // c-particle packet; the high-level WIT spawn_particle call cannot
            // carry dust option bytes.
            let (red, green, blue) = if now_silent {
                (1.0_f32, 0.0_f32, 0.0_f32)
            } else {
                (0.0_f32, 1.0_f32, 0.0_f32)
            };
            let mut data = Vec::with_capacity(16);
            data.extend_from_slice(&red.to_be_bytes());
            data.extend_from_slice(&green.to_be_bytes());
            data.extend_from_slice(&blue.to_be_bytes());
            data.extend_from_slice(&1.0_f32.to_be_bytes());

            let packet = ClientboundPacket::CParticle(CParticle {
                force_spawn: false,
                important: true,
                position: pos,
                offset: PARTICLE_OFFSET,
                max_speed: PARTICLE_MAX_SPEED,
                particle_count: PARTICLE_COUNT,
                particle_id: i32::from(Particle::Dust as u8),
                data,
            });
            java.send_packet(&packet);
        } else {
            // Non-Java players see a generic dust cloud without color data.
            event.player.spawn_particles(
                Particle::Dust,
                pos,
                PARTICLE_COUNT as u32,
                PARTICLE_OFFSET,
                PARTICLE_MAX_SPEED,
            );
        }

        let gamemode = GameMode::from(event.player.get_gamemode());
        if matches!(gamemode, GameMode::Survival | GameMode::Adventure) {
            let count = stack.get_count();
            if count > 1 {
                stack.set_count(count - 1);
                event.player.set_item_in_hand(hand.into(), Some(stack));
            } else {
                event.player.set_item_in_hand(hand.into(), None);
            }
        }

        event.cancelled = true;

        debug!(
            "Silence: player {} toggled silent={} for entity {} ({:?})",
            event.player.get_name(),
            now_silent,
            event.entity_id,
            entity.get_type(),
        );

        event
    }
}

/// Returns `true` for hostile monster entity types that should not be silenced.
///
/// Pumpkin does not expose a `Monster` marker trait, so we match the vanilla
/// hostile mob set directly. Neutral or passive mobs (wolves, iron golems, bees,
/// etc.) are left silenceable.
fn is_monster(entity_type: EntityType) -> bool {
    matches!(
        entity_type,
        EntityType::Blaze
            | EntityType::Bogged
            | EntityType::Breeze
            | EntityType::CamelHusk
            | EntityType::CaveSpider
            | EntityType::Creeper
            | EntityType::Drowned
            | EntityType::ElderGuardian
            | EntityType::EnderDragon
            | EntityType::Enderman
            | EntityType::Endermite
            | EntityType::Evoker
            | EntityType::EvokerFangs
            | EntityType::Ghast
            | EntityType::Giant
            | EntityType::Guardian
            | EntityType::Hoglin
            | EntityType::Husk
            | EntityType::Illusioner
            | EntityType::MagmaCube
            | EntityType::Phantom
            | EntityType::Piglin
            | EntityType::PiglinBrute
            | EntityType::Pillager
            | EntityType::PolarBear
            | EntityType::Ravager
            | EntityType::Shulker
            | EntityType::Silverfish
            | EntityType::Skeleton
            | EntityType::Slime
            | EntityType::Stray
            | EntityType::Strider
            | EntityType::Vex
            | EntityType::Vindicator
            | EntityType::Warden
            | EntityType::Witch
            | EntityType::Wither
            | EntityType::WitherSkeleton
            | EntityType::Zoglin
            | EntityType::Zombie
            | EntityType::ZombieHorse
            | EntityType::ZombieNautilus
            | EntityType::ZombieVillager
            | EntityType::ZombifiedPiglin
    )
}

/// Configuration for the silence mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceConfig {
    /// Whether this module is active.
    pub enabled: bool,
    /// Registry key of the item held to trigger the silence toggle.
    /// Defaults to `minecraft:amethyst_shard`.
    pub tool_item: String,
    /// Which hands the tool item can be held in to trigger the toggle.
    /// Use variant names like "Left" or "Right". Leave empty to allow either hand.
    pub hand: Vec<MirrorHand>,
}

impl Default for SilenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            tool_item: "minecraft:amethyst_shard".to_string(),
            hand: vec![MirrorHand::Right],
        }
    }
}
