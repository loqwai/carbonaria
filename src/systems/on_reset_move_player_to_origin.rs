use bevy::prelude::*;
use crate::{events::ResetEvent, components::Player};

pub fn on_reset_move_player_to_origin(
    mut reset_events: EventReader<ResetEvent>,
    mut transforms: Query<&mut Transform, With<Player>>,
) {
    for _ in reset_events.iter() {
        for mut transform in transforms.iter_mut() {
            transform.translation = Vec3::ZERO;
        }
    }
}
