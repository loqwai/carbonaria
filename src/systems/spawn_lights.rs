use bevy::prelude::*;

use crate::resources::{CameraType, Config};

const HALF_SIZE: f32 = 1.0;

pub fn spawn_lights(config: Res<Config>, mut commands: Commands) {
    if config.camera_type != CameraType::Camera3d {
        return;
    }

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
