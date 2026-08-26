//! Entity lookup helpers.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::world::EntityType;

/// Looks up an entity's type by its runtime ID across all loaded worlds.
#[must_use]
pub fn entity_type_by_id(server: &Server, id: u32) -> Option<EntityType> {
    for world in server.get_all_worlds() {
        for entity in world.get_entities() {
            if entity.get_id() == id {
                return Some(entity.get_type());
            }
        }
    }
    None
}
