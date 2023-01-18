use bevy::{ecs::query::QuerySingleError, input::Input, prelude::*};
use bevy_rapier2d::prelude::Velocity;

use crate::{components::Player, events::MoveEvent};

#[derive(Debug, Error)]
enum MovePlayerError {
    QuerySingleError(QuerySingleError),
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    velocity_query: Query<(Entity, &mut Velocity), With<Player>>,
    move_events: EventWriter<MoveEvent>,
) {
    if let Err(e) = fallible_move_player(keyboard_input, velocity_query, move_events) {
        println!("Error moving player: {}", e);
    }
}

fn fallible_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocity_query: Query<(Entity, &mut Velocity), With<Player>>,
    mut move_events: EventWriter<MoveEvent>,
) -> Result<(), MovePlayerError> {
    for (entity, _) in velocity_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            move_events.send(MoveEvent {
                who: entity,
                velocity: Vec3::new(-1.0, 0.0, 0.0),
                rotation: Quat::default(),
            });
        }
        if keyboard_input.pressed(KeyCode::D) {
            move_events.send(MoveEvent {
                who: entity,
                velocity: Vec3::new(1.0, 0.0, 0.0),
                rotation: Quat::default(),
            });
        }
        if keyboard_input.pressed(KeyCode::W) {
            move_events.send(MoveEvent {
                who: entity,
                velocity: Vec3::new(0.0, 1.0, 0.0),
                rotation: Quat::default(),
            });
        }
        if keyboard_input.pressed(KeyCode::S) {
            move_events.send(MoveEvent {
                who: entity,
                velocity: Vec3::new(0.0, -1.0, 0.0),
                rotation: Quat::default(),
            });
        }
    }
    Ok(())
}
