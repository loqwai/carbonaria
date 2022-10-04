use bevy::prelude::*;
use heron::Collisions;

use crate::components::Exit;

pub fn on_exit_reset(q_exits: Query<&Collisions, With<Exit>>) {
    let exit_happened = q_exits.iter().any(|collisions| !collisions.is_empty());

    if !exit_happened {
        return;
    }

    panic!("Exit collision")
}
