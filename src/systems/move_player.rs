use bevy::{ecs::query::QuerySingleError, input::Input, prelude::*};
use heron::Velocity;

use crate::{components::{Player, Speed}, events::MoveEvent};

#[derive(Debug, Error)]
enum MovePlayerError {
    QuerySingleError(QuerySingleError),
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    velocity_query: Query<(Entity, &mut Velocity), With<Player>>,
    speed_query: Query<(&Speed, &Parent)>,
    move_events: EventWriter<MoveEvent>,
) {
    if let Err(e) = fallible_move_player(keyboard_input, velocity_query, speed_query, move_events) {
        println!("Error moving player: {}", e);
    }
}

fn fallible_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocity_query: Query<(Entity, &mut Velocity), With<Player>>,
    speed_query: Query<(&Speed, &Parent)>,
    mut move_events: EventWriter<MoveEvent>,
) -> Result<(), MovePlayerError> {
    for (entity, _) in velocity_query.iter_mut() {
        let mut entity_speed: f32 = 40.0;

        for (speed, parent) in speed_query.iter() {
            if parent.id() != entity.id() { continue }

            entity_speed *= speed.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            move_events.send(MoveEvent{
                target: entity,
                velocity: Vec3::new(-entity_speed, 0.0, 0.0),
            });
        }
        if keyboard_input.pressed(KeyCode::D) {
            move_events.send(MoveEvent{
                target: entity,
                velocity: Vec3::new(entity_speed, 0.0, 0.0),
            });
        }
        if keyboard_input.pressed(KeyCode::W) {
            move_events.send(MoveEvent{
                target: entity,
                velocity: Vec3::new(0.0, entity_speed, 0.0),
            });
        }
        if keyboard_input.pressed(KeyCode::S) {
            move_events.send(MoveEvent{
                target: entity,
                velocity: Vec3::new(0.0, -entity_speed, 0.0),
            });
        }
    }
    Ok(())
}
