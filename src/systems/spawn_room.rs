use bevy::prelude::*;

use crate::{bundles::WallBundle, components::Room, resources::Config};

pub fn spawn_room(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>, existing_rooms: Query<Entity, With<Room>>) {
    if !existing_rooms.is_empty() {
        return;
    }

    let room = Room::new(config.dimensions);

    commands.spawn().insert(room.clone());

    for (position, wall_type) in room.known_tiles.iter() {
        let wall = commands
            .spawn_bundle(WallBundle::new(&asset_server, wall_type, *position))
            .id();

        for shape in wall_type.collision_shapes() {
            let child = commands.spawn().insert(shape).id();
            commands.entity(wall).push_children(&[child]);
        }
    }
}
