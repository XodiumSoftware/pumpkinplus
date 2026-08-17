//! Entity lookup helpers.

use crate::EntityType;
use pumpkin_plugin_api::Server;

/// Looks up an entity's mirrored type by its runtime ID across all loaded worlds.
#[must_use]
pub fn entity_type_by_id(server: &Server, id: u32) -> Option<EntityType> {
    for world in server.get_all_worlds() {
        for entity in world.get_entities() {
            if entity.get_id() == id {
                return Some(EntityType::from(entity.get_type()));
            }
        }
    }
    None
}
