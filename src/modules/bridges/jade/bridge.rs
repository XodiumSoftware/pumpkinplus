//! Jade bridge — server-side handler for the Jade (WAILA) client mod.
//!
//! Jade lets players point at a block or entity and see extended info in the
//! HUD. By default it only knows what vanilla clients see; this bridge
//! advertises extra *providers* to the client and replies to data requests
//! with NBT payloads the client uses to render extra rows.
//!
//! ## Protocol
//!
//! | Channel                  | Direction       | Purpose                                |
//! |--------------------------|-----------------|----------------------------------------|
//! | `jade:client_handshake`  | client → server | Client announces support               |
//! | `jade:server_handshake`  | server → client | Server lists its provider uids         |
//! | `jade:request_block`     | client → server | Client asks about a targeted block     |
//! | `jade:request_entity`    | client → server | Client asks about a targeted entity    |
//! | `jade:receive_data`      | server → client | Server replies with an NBT compound    |
//!
//! ## Handshake
//!
//! On `jade:client_handshake` we reply with `jade:server_handshake`:
//! `VarInt(0) | VarInt(0) | VarInt(block_count) | block_key* | VarInt(entity_count) | entity_key*`.
//! Client uses these key indices in subsequent requests to ask for specific
//! providers by index.
//!
//! ## Requests
//!
//! Both `request_block` and `request_entity` end with the same trailing
//! shape: `VarInt(indices_len) | VarInt(provider_index)*`. The indices
//! refer to the order of keys we sent in the handshake.
//!
//! For `request_block`, the leading bytes carry `show_details: bool` +
//! `BlockHitResult` (packed `i64` pos + `u8` face + `3×f32` hit-vec +
//! `bool` is-miss) + `ItemStack` + optional accessor NBT. We parse just
//! enough to recover the target block position.
//!
//! For `request_entity`, the leading bytes carry `show_details: bool` +
//! `entity_id: VarInt` + `partIndex: VarInt` + `3×f32 hit_vec` + optional
//! accessor NBT.
//!
//! ## Configuration
//!
//! | Field     | Default | Description                     |
//! |-----------|---------|---------------------------------|
//! | `enabled` | `false` | Whether this bridge is active   |

use crate::config::ConfigManager;
use crate::modules::bridges::bridge::Bridge;
use crate::modules::bridges::jade::buf::{Buf, BufReader};
use crate::modules::bridges::jade::nbt::NbtCompound;
use crate::modules::bridges::jade::providers::{JadeBlockProvider, JadeEntityProvider};
use pumpkin_plugin_api::events::{
    EventData, EventHandler, EventPriority, PlayerCustomPayloadEvent,
};
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::wit::pumpkin::plugin::common::BlockPos;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Channel for client → server "I support Jade".
const CLIENT_HANDSHAKE_CHANNEL: &str = "jade:client_handshake";
/// Channel for server → client provider list reply.
const SERVER_HANDSHAKE_CHANNEL: &str = "jade:server_handshake";
/// Channel for client → server "tell me about this block".
const REQUEST_BLOCK_CHANNEL: &str = "jade:request_block";
/// Channel for client → server "tell me about this entity".
const REQUEST_ENTITY_CHANNEL: &str = "jade:request_entity";
/// Channel for server → client data reply.
const RECEIVE_DATA_CHANNEL: &str = "jade:receive_data";

/// Handles Jade (WAILA) plugin channel synchronization.
#[derive(Default)]
pub struct Jade;

impl Bridge for Jade {
    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.bridges.jade.enabled)
    }

    fn events(&self, context: &Context) {
        self.register_event::<PlayerCustomPayloadEvent>(context, EventPriority::Normal, true);
    }
}

impl Jade {
    /// All block providers in handshake order.
    fn block_providers() -> Vec<Box<dyn JadeBlockProvider>> {
        // TODO: populate as providers are ported. For now this bridge
        // will still handshake with an empty list, which Jade treats as
        // "server provides no block extensions" — a valid working state.
        Vec::new()
    }

    /// All entity providers in handshake order.
    fn entity_providers() -> Vec<Box<dyn JadeEntityProvider>> {
        // TODO: populate as providers are ported.
        Vec::new()
    }

    /// Builds the server handshake payload: empty config + empty shearable
    /// blocks + the supported provider key lists.
    fn handshake_payload() -> Vec<u8> {
        let mut buf = Buf::new();
        buf.write_var_int(0); // server_config map size
        buf.write_var_int(0); // shearable_blocks list size

        let block_providers = Self::block_providers();
        buf.write_var_int(len(block_providers.len()));
        for p in &block_providers {
            buf.write_utf(p.key());
        }

        let entity_providers = Self::entity_providers();
        buf.write_var_int(len(entity_providers.len()));
        for p in &entity_providers {
            buf.write_utf(p.key());
        }

        buf.into_vec()
    }

    /// Sends a raw plugin channel payload to the player.
    fn send(player: &Player, channel: &str, bytes: &[u8]) {
        if let Some(java) = player.as_java() {
            java.send_custom_payload(channel, bytes);
        }
    }

    /// Handles `jade:request_block` by decoding the leading position,
    /// routing to each matching provider, and replying with `receive_data`.
    #[allow(clippy::too_many_lines)]
    fn handle_block_request(player: &Player, bytes: &[u8]) {
        let mut r = BufReader::new(bytes);
        let _show_details = r.read_bool();

        // TODO(jade-codec): The Jade client sends `BlockHitResult` packed as
        // `i64` (long-encoded BlockPos) followed by direction `u8` and a
        // `3×f32` hit-vec, then `bool` miss flag, then an ItemStack in NMS
        // stream-codec form, then optionally accessor NBT.
        //
        // For now we only extract the packed BlockPos and skip the rest —
        // providers only need the target position. If we ever need the
        // leading ItemStack (the "serversideRep" hint), we'll need to
        // implement ItemStack stream-codec decoding, which depends on NMS
        // component serialization. Tracked by the broader `ItemStack` codec
        // TODO.
        let packed_pos = r.read_packed_i64();
        let pos = decode_block_pos(packed_pos);

        // Skip face (u8) + hit-vec (3× f32) + miss flag (u8).
        let _ = r.read_u8();
        let _ = r.read_f32();
        let _ = r.read_f32();
        let _ = r.read_f32();
        let _ = r.read_bool();

        // ItemStack in NMS stream codec — TODO, see above.
        // For now we assume the stack is empty (VarInt count = 0), which is
        // the common case. Real decoding requires ItemStack codec support.
        let stack_count = r.read_var_int();
        if stack_count != 0 {
            // Item present; bail for now until the ItemStack codec is ported.
            return;
        }

        // Optional accessor NBT — skip if present (we don't read it).
        // (First byte 0x0A = TAG_Compound.)
        // TODO(jade-codec): skip compound correctly when accessor data present.

        let count = r.read_var_int();
        let world = player.get_world();
        let providers = Self::block_providers();

        // Build the response lazily on the first provider that writes data;
        // that way if no provider matches we send nothing.
        let mut response: Option<NbtCompound> = None;
        for _ in 0..count.max(0) {
            let idx = r.read_var_int();
            let Ok(idx) = usize::try_from(idx) else {
                continue;
            };
            let Some(provider) = providers.get(idx) else {
                continue;
            };
            let tag = response.get_or_insert_with(|| {
                NbtCompound::new()
                    .put_int("x", pos.x)
                    .put_int("y", pos.y)
                    .put_int("z", pos.z)
                // TODO(block-id): populate "BlockId" from the Block at `pos`
                // once the typed accessor is exposed. Jade uses this for the
                // client-side icon.
            });
            let _wrote = provider.write(&world, pos, tag);
        }

        // Note: we reply only if at least one provider actually wrote
        // something. With no providers registered yet, this is always None.
        if let Some(tag) = response {
            Self::send(player, RECEIVE_DATA_CHANNEL, &tag.encode());
        }
    }

    /// Handles `jade:request_entity` by decoding the entity id, looking it
    /// up in the world, and routing to each matching provider.
    fn handle_entity_request(player: &Player, bytes: &[u8]) {
        let mut r = BufReader::new(bytes);
        let _show_details = r.read_bool();

        // Leading: entity_id VarInt + partIndex VarInt + 3× f32 hit_vec.
        let entity_id = r.read_var_int();
        let _part_index = r.read_var_int();
        let _hx = r.read_f32();
        let _hy = r.read_f32();
        let _hz = r.read_f32();

        // Optional accessor NBT — TODO, same as block path.

        let count = r.read_var_int();
        // Pumpkin: no direct get_entity(entity_id), so walk the world's
        // entities and find the matching one. O(n) per request — fine for
        // the rare "player is pointing at something" case.
        //
        // TODO(pumpkin-api): If/when Pumpkin adds `World::get_entity(id)`,
        // switch this to a single call.
        let world = player.get_world();
        let Some(entity) = world
            .get_entities()
            .into_iter()
            .find(|e| i32::try_from(e.get_id()).ok() == Some(entity_id))
        else {
            return;
        };

        let providers = Self::entity_providers();
        let mut response: Option<NbtCompound> = None;
        for _ in 0..count.max(0) {
            let idx = r.read_var_int();
            let Ok(idx) = usize::try_from(idx) else {
                continue;
            };
            let Some(provider) = providers.get(idx) else {
                continue;
            };
            let tag =
                response.get_or_insert_with(|| NbtCompound::new().put_int("EntityId", entity_id));
            let _wrote = provider.write(&entity, tag);
        }

        if let Some(tag) = response {
            Self::send(player, RECEIVE_DATA_CHANNEL, &tag.encode());
        }
    }
}

impl EventHandler<PlayerCustomPayloadEvent> for Jade {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerCustomPayloadEvent>,
    ) -> EventData<PlayerCustomPayloadEvent> {
        match event.channel.as_str() {
            CLIENT_HANDSHAKE_CHANNEL => {
                Self::send(
                    &event.player,
                    SERVER_HANDSHAKE_CHANNEL,
                    &Self::handshake_payload(),
                );
            }
            REQUEST_BLOCK_CHANNEL => {
                Self::handle_block_request(&event.player, &event.data);
            }
            REQUEST_ENTITY_CHANNEL => {
                Self::handle_entity_request(&event.player, &event.data);
            }
            _ => {}
        }
        event
    }
}

/// Decodes a packed `i64` `BlockPos` (the form Jade uses on the wire).
///
/// Packed format: `x: 26 bits | z: 26 bits | y: 12 bits`, sign-extended.
fn decode_block_pos(packed: i64) -> BlockPos {
    // X is the high 26 bits; shift right arithmetic to sign-extend.
    #[allow(clippy::cast_possible_truncation)]
    let x = (packed >> 38) as i32;
    // Y is the low 12 bits, sign-extended.
    #[allow(clippy::cast_possible_truncation)]
    let y = ((packed << 52) >> 52) as i32;
    // Z is the middle 26 bits, sign-extended.
    #[allow(clippy::cast_possible_truncation)]
    let z = ((packed << 26) >> 38) as i32;
    BlockPos { x, y, z }
}

/// Saturating `usize` → `i32` conversion for `VarInt` write calls.
///
/// Provider counts are tiny; this is only to satisfy clippy's
/// pedantic `cast_possible_truncation`/`cast_possible_wrap` lints.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn len(n: usize) -> i32 {
    n.min(i32::MAX as usize) as i32
}

/// Configuration for the Jade bridge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JadeConfig {
    /// Whether this bridge is active.
    pub enabled: bool,
}
