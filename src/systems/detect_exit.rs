use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::components::{Exit, Player};
use crate::events::ResetEvent;

pub fn detect_exit(
    q_exits: Query<Entity, With<Exit>>,
    q_players: Query<Entity, With<Player>>,
    mut exit_collision_event_reader: EventReader<CollisionEvent>,
    mut reset_event_writer: EventWriter<ResetEvent>,
) {
    let collision_happened = exit_collision_event_reader
        .iter()
        .any(|collision| match collision {
            CollisionEvent::Started(a, b, _) => {
                if q_players.contains(*a) && q_exits.contains(*b) {
                    return true;
                }

                if q_players.contains(*b) && q_exits.contains(*b) {
                    return true;
                }

                return false;
            }
            _ => false,
        });

    if collision_happened {
        reset_event_writer.send(ResetEvent {});
    }
}
