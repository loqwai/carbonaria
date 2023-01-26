use bevy::prelude::*;

use crate::{events::ResetEvent, components::Exit};

pub fn on_reset_despawn_all_exits(
  mut commands: Commands,
  mut reset_events: EventReader<ResetEvent>,
  mut exits: Query<Entity, With<Exit>>,
) {
    for _ in reset_events.iter() {
        for exit in exits.iter_mut() {
            commands.entity(exit).despawn_recursive();
        }
    }
}
