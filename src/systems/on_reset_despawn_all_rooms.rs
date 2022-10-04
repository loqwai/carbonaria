use bevy::prelude::*;

use crate::{events::ResetEvent, components::Room, components::WallType};

pub fn on_reset_despawn_all_rooms(
    mut commands: Commands,
    mut reset_events: EventReader<ResetEvent>,
    mut wall_types: Query<Entity, With<WallType>>,
    mut rooms: Query<Entity, With<Room>>,
) {
    for _ in reset_events.iter() {
        for wall_type in wall_types.iter_mut() {
            commands.entity(wall_type).despawn_recursive();
        }

        for room in rooms.iter_mut() {
            commands.entity(room).despawn_recursive();
        }
    }
}
