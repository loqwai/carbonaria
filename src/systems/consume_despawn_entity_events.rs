use crate::events::DespawnEvent;
use bevy::prelude::*;

pub fn consume_despawn_entity_events(
    mut despawn_events: EventReader<DespawnEvent>,
    mut commands: Commands,
    entities: Query<Entity>,
) {
    despawn_events
        .iter()
        .filter_map(|event| entities.get(event.entity).ok())
        .for_each(|entity| commands.entity(entity).despawn_recursive())
}
