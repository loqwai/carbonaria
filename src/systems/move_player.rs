use bevy::{input::Input, prelude::*};

use crate::resources::PlayerResource;

#[derive(Debug, Error)]
enum MovePlayerError {
    CouldNotFindPlayerEntityError,
    QueryError(bevy::ecs::query::QueryEntityError),
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    player: Res<PlayerResource>,
    transforms: Query<&mut Transform>,
) {
    if let Err(e) = fallible_move_player(keyboard_input, player, transforms) {
        panic!("Error moving player: {}", e);
    }
}

fn fallible_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    player: Res<PlayerResource>,
    mut transforms: Query<&mut Transform>,
) -> Result<(), MovePlayerError> {
    let entity = player
        .entity
        .ok_or(MovePlayerError::CouldNotFindPlayerEntityError)?;
    let mut transform = transforms.get_mut(entity)?;

    let player_speed = 5.0;

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation[0] -= player_speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation[0] += player_speed;
    }

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation[1] += player_speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation[1] -= player_speed;
    }

    Ok(())
}
