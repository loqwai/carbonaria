use bevy::prelude::*;
use crate::events::DespawnEvent;

pub fn consume_despawn_entity_events(
    mut despawn_events: EventReader<DespawnEvent>,
    mut commands: Commands,
){
    despawn_events.iter().for_each(|event| {
        commands.entity(event.entity).despawn_recursive();
    })
}