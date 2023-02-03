use bevy::prelude::*;
use crate::events::DespawnEvent;

pub fn consume_despawn_entity_events(
    mut despawn_events: EventReader<DespawnEvent>,
    mut commands: Commands,
    entities: Query<Entity>,
){
    despawn_events.iter().for_each(|event| {
        match entities.get(event.entity).ok() {
            None => (),
            Some(entity) => commands.entity(entity).despawn_recursive(),
        }
    })
}