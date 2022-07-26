use bevy::{ecs::system::QuerySingleError, input::Input, prelude::*};
use heron::Velocity;

use crate::components::Player;

#[derive(Debug, Error)]
enum MovePlayerError {
    QuerySingleError(QuerySingleError),
}

pub fn move_player(keyboard_input: Res<Input<KeyCode>>, query: Query<(&Player, &mut Velocity)>) {
    if let Err(e) = fallible_move_player(keyboard_input, query) {
        panic!("Error moving player: {}", e);
    }
}

fn fallible_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity)>,
) -> Result<(), MovePlayerError> {
    let (_, mut velocity) = query.get_single_mut()?;

    let player_speed = 5.0;

    if keyboard_input.pressed(KeyCode::A) {
        velocity.linear += Vec3::new(-player_speed, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        velocity.linear += Vec3::new(player_speed, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::W) {
        velocity.linear += Vec3::new(0.0, player_speed, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        velocity.linear += Vec3::new(0.0, -player_speed, 0.0);
    }

    Ok(())
}
