use bevy::prelude::*;
use crate::{components::TimeToLive, events::DespawnEvent};

pub fn time_to_live(
    mut despawn_events: EventWriter<DespawnEvent>,
    mut q_time_to_live: Query<(&Parent, &mut TimeToLive)>,
) {
    q_time_to_live.for_each_mut(|(parent, mut time_to_live)| {
        time_to_live.0 -= 1;
        if time_to_live.0 <= 0 {
            despawn_events.send(DespawnEvent { entity: parent.get() });
        }
    });
}
