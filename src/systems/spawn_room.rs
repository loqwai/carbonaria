use bevy::prelude::*;

use crate::{bundles::WallBundle, utils::generate_room};

pub fn spawn_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    let room = generate_room();

    for (position, tile) in room.iter() {
        let x: f32 = f32::from(position.0 .0);
        let y: f32 = f32::from(position.0 .1);

        commands.spawn_bundle(WallBundle::new(
            &asset_server,
            tile,
            (x, y),
            Vec3::new(16.0, 16.0, 0.0),
        ));
    }
}
