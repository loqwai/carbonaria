use bevy::prelude::*;
use crate::{events::ResetEvent, components::Mob};

pub fn on_reset_despawn_all_mobs(
    mut commands: Commands,
    mut reset_events: EventReader<ResetEvent>,
    mut mobs: Query<Entity, With<Mob>>,
) {
    for _ in reset_events.iter() {
        for mob in mobs.iter_mut() {
            commands.entity(mob).despawn_recursive();
        }
    }
}
