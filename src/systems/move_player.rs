use bevy::{prelude::*, ecs::query::QuerySingleError};
use bevy_rapier2d::{prelude::Velocity};

use crate::{
    components::{MousePos, Player},
    events::MoveEvent, util::look_at_target,
};

#[derive(Debug, Error)]
enum MovePlayerError {
    QuerySingleError(QuerySingleError),
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    velocity_query: Query<(Entity, &Transform, &mut Velocity), With<Player>>,
    mouse_query: Query<&Transform, With<MousePos>>,
    move_events: EventWriter<MoveEvent>,
) {
    if let Err(e) = fallible_move_player(
        keyboard_input,
        velocity_query,
        mouse_query,
        move_events) {
        println!("Error moving player: {}", e);
    }
}

fn fallible_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocity_query: Query<(Entity, &Transform, &mut Velocity), With<Player>>,
    mouse_query: Query<&Transform, With<MousePos>>,
    mut move_events: EventWriter<MoveEvent>,
) -> Result<(), MovePlayerError> {
    for (entity,player_pos, _) in velocity_query.iter_mut() {
        let mut rotation = Quat::default();
        let mut velocity = Vec3::default();

        match mouse_query.get_single() {
            Ok(mouse_pos) => {
                let (r,_) = look_at_target(player_pos.translation, mouse_pos.translation);
                rotation = r;
            },
            Err(_) => {todo!()},
        }
        if keyboard_input.pressed(KeyCode::A) {
            velocity = Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity = Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            velocity = Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity = Vec3::new(0.0, -1.0, 0.0);
        }
        move_events.send(MoveEvent{
            who: entity,
            velocity,
            rotation,
        });
    }
    Ok(())
}
