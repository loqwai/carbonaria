use bevy::prelude::*;
use bevy::math::Vec3Swizzles;

use crate::components::{Compass, Exit, Player};

pub fn update_compass(
  mut compasses: Query<(&mut Transform, &Parent), (With<Compass>, Without<Player>, Without<Exit>)>,
  players: Query<&Transform, With<Player>>,
  exits: Query<&Transform, With<Exit>>,
) {
    match exits.get_single() {
        Err(_) => return,
        Ok(exit) => {
            for (mut compass, parent) in compasses.iter_mut() {
                let player = players.get(**parent).unwrap();

                let compass_forward = (compass.rotation * Vec3::Y).xy();
                let player_location = player.translation;
                let to_exit = (exit.translation - player_location).xy().normalize();

                let forward_dot_exit = compass_forward.dot(to_exit);

                let compass_right = (compass.rotation * Vec3::X).xy();
                let right_dot_exit = compass_right.dot(to_exit);
                let rotation_sign = -f32::copysign(1.0, right_dot_exit);

                let max_angle = forward_dot_exit.clamp(-1.0, 1.0).acos();
                let rotation_angle =  rotation_sign * (0.1 as f32).min(max_angle);
                compass.rotate_z(rotation_angle);
            }
        },
    }
}
