use bevy::prelude::*;

pub fn remove_all_entities(mut commands: Commands, entities: Query<Entity>) {
    for e in entities.iter() {
        commands.entity(e).despawn();
    }
}
