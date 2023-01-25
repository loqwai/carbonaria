use bevy::prelude::*;

use crate::{components::Player, events::MoveEvent};

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    players: Query<Entity, With<Player>>,
    mut move_events: EventWriter<MoveEvent>,
) {
    for player in players.iter() {
        let mut direction = Vec3::default();

        if keyboard_input.pressed(KeyCode::A) {
            direction = Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction = Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction = Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction = Vec3::new(0.0, -1.0, 0.0);
        }

        move_events.send(MoveEvent {
            who: player,
            direction,
        });
    }
}
