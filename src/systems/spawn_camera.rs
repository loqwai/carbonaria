use bevy::prelude::*;

use crate::resources::{CameraType, Config};

pub fn spawn_camera(mut commands: Commands, config: Res<Config>) {
    match config.camera_type {
        CameraType::Camera2d => commands.spawn(Camera2dBundle::new_with_far(100.0)),
        CameraType::Camera3d => commands.spawn(Camera3dBundle::default()),
    };
}
