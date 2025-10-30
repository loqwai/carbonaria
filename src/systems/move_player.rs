use bevy::prelude::*;

use crate::{components::Player, events::MoveEvent};

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    players: Query<Entity, With<Player>>,
    mut move_events: EventWriter<MoveEvent>,
) {
    for player in players.iter() {
        let mut directions: Vec<Vec3> = Vec::new();

        if keyboard_input.pressed(KeyCode::KeyA) {
            directions.push(Vec3::new(-1.0, 0.0, 0.0));
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            directions.push(Vec3::new(1.0, 0.0, 0.0));
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            directions.push(Vec3::new(0.0, 1.0, 0.0));
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            directions.push(Vec3::new(0.0, -1.0, 0.0));
        }

        if directions.is_empty() {
            continue;
        }

        move_events.send(MoveEvent {
            who: player,
            direction: directions.iter().sum(),
        });
    }
}
