use bevy::prelude::*;
use bevy::utils::HashSet;
use heron::Collisions;

use crate::components::{Exit, Player};
use crate::events::ResetEvent;

pub fn detect_exit(
    q_exits: Query<&Collisions, With<Exit>>,
    q_players: Query<Entity, With<Player>>,
    mut reset_event_writer: EventWriter<ResetEvent>,
) {
    let players: HashSet<Entity> = q_players.iter().collect();
    let collisions: HashSet<Entity> = q_exits.iter().flat_map(|cs| cs.entities()).collect();

    if players.is_disjoint(&collisions) {
        return;
    }

    reset_event_writer.send(ResetEvent {});
}
