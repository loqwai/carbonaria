use bevy::prelude::*;
use crate::components::MousePos;
pub fn spawn_crosshairs(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("mob.png"),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    }).insert(MousePos);
}
