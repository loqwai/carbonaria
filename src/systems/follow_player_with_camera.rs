use bevy::prelude::*;

use crate::{
    components::Player,
    resources::{CameraType::*, Config},
};

pub fn follow_player_with_camera(
    config: Res<Config>,
    q_player: Query<&Transform, With<Player>>,
    q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    match config.camera_type {
        Camera2d => follow_player_with_camera_2d(config, q_player, q_camera),
        Camera3d => follow_player_with_camera_3d(config, q_player, q_camera),
    }
}

fn follow_player_with_camera_2d(
    config: Res<Config>,
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    match q_player.get_single() {
        Err(_) => return,
        Ok(player) => {
            let mut camera = q_camera.get_single_mut().unwrap();

            let target_translation = Vec3::new(
                player.translation.x,
                player.translation.y,
                camera.translation.z,
            );

            camera.translation = camera
                .translation
                .lerp(target_translation, config.camera_follow_interpolation);
        }
    }
}

fn follow_player_with_camera_3d(
    config: Res<Config>,
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    match q_player.get_single() {
        Err(_) => return,
        Ok(player) => {
            let mut camera = q_camera.get_single_mut().unwrap();

            let target_translation = Vec3::new(
                player.translation.x,
                player.translation.y - 700.,
                camera.translation.z,
            );

            camera.translation = camera
                .translation
                .lerp(target_translation, config.camera_follow_interpolation);
        }
    }
}
