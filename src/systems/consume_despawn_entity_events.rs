use crate::events::DespawnEvent;
use bevy::prelude::*;

pub fn consume_despawn_entity_events(
    mut despawn_events: EventReader<DespawnEvent>,
    mut commands: Commands,
    entities: Query<Entity>,
) {
    for event in despawn_events.read() {
        if let Ok(entity) = entities.get(event.entity) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
