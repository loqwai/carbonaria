use bevy::prelude::*;

pub fn remove_all_entities(mut commands: Commands, entities: Query<Entity>) {
    entities.for_each(|e| commands.entity(e).despawn())
}
