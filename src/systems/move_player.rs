use bevy::{ecs::query::QuerySingleError, input::Input, prelude::*};
use bevy_rapier2d::prelude::Velocity;

use crate::components::{Player, Speed};

#[derive(Debug, Error)]
enum MovePlayerError {
    QuerySingleError(QuerySingleError),
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    velocity_query: Query<(Entity, &mut Velocity), With<Player>>,
    speed_query: Query<(&Speed, &Parent)>,
) {
    if let Err(e) = fallible_move_player(keyboard_input, velocity_query, speed_query) {
        println!("Error moving player: {}", e);
    }
}

fn fallible_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocity_query: Query<(Entity, &mut Velocity), With<Player>>,
    speed_query: Query<(&Speed, &Parent)>,
) -> Result<(), MovePlayerError> {
    for (entity, mut velocity) in velocity_query.iter_mut() {
        let mut entity_speed: f32 = 40.0;

        for (speed, parent) in speed_query.iter() {
            if parent.get() != entity {
                continue;
            }

            entity_speed *= speed.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            velocity.linvel += Vec2::new(-entity_speed, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.linvel += Vec2::new(entity_speed, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            velocity.linvel += Vec2::new(0.0, entity_speed);
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.linvel += Vec2::new(0.0, -entity_speed);
        }
    }
    Ok(())
}
