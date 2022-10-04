use bevy::prelude::*;
use heron::Collisions;

use crate::components::Exit;
use crate::events::ResetEvent;

pub fn detect_exit(
    q_exits: Query<&Collisions, With<Exit>>,
    mut reset_event_writer: EventWriter<ResetEvent>
) {
    let exit_happened = q_exits.iter().any(|collisions| !collisions.is_empty());

    if !exit_happened {
        return;
    }

    reset_event_writer.send(ResetEvent {})
}
