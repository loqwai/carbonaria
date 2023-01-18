use bevy::prelude::*;

use crate::{components::{Health}};

pub fn on_0_health_kill(
    mut commands: Commands,
    things: Query<(Entity, &Health)>,
) {
    for (thing, health) in things.iter() {
        if health.0 > 0 {
            continue;
        }

        commands.entity(thing).despawn_recursive();
    }
}
