use crate::{
    components::MousePos,
    resources::{CameraType, Config},
};
use bevy::prelude::*;
pub fn spawn_crosshairs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    config: Res<Config>,
) {
    if config.camera_type != CameraType::Camera2d {
        return;
    }

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("crosshairs.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(MousePos);
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
}
