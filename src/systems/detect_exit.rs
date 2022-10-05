use bevy::prelude::*;
use heron::Collisions;

use crate::components::{Exit, Player};
use crate::events::ResetEvent;

pub fn detect_exit(
    q_exits: Query<&Collisions, With<Exit>>,
    q_players: Query<Entity, With<Player>>,
    mut reset_event_writer: EventWriter<ResetEvent>,
) {
    match q_players.get_single() {
        Err(_) => return,
        Ok(player) => {
            let exit_happened = q_exits.iter().any(|collisions| collisions.contains(&player));

            if !exit_happened {
                return;
            }

            reset_event_writer.send(ResetEvent {});
        }
    }
}
