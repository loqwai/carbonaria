use bevy::prelude::*;

use crate::components::Player;

pub fn follow_player_with_camera(
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = q_player.get_single().unwrap();
    let mut camera = q_camera.get_single_mut().unwrap();

    camera.translation = player.translation;
}
