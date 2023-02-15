use bevy::prelude::*;

use crate::{components::Health, events::DespawnEvent};

pub fn on_0_health_kill(
    mut despawn_events: EventWriter<DespawnEvent>,
    things: Query<(Entity, &Health)>,
) {
    for (entity, health) in things.iter() {
        if health.0 > 0.0 {
            continue;
        }

        despawn_events.send(DespawnEvent { entity });
    }
}
