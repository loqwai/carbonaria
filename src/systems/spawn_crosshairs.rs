use crate::{
    components::MousePos,
    resources::{CameraType, Config},
};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub fn spawn_crosshairs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_window: Query<&mut Window>,
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

    if let Ok(mut window) = q_window.get_single_mut() {
        window.cursor.visible = false;
    }
}
