use bevy::prelude::*;

use crate::{components::Player, resources::Config};

pub fn follow_player_with_camera(
    config: Res<Config>,
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = q_player.get_single().unwrap();
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
