use crate::{
    components::{Math, TimeToLive},
    events::DespawnEvent,
};
use bevy::prelude::*;

pub fn time_to_live(
    mut commands: Commands,
    mut despawn_events: EventWriter<DespawnEvent>,
    q_time_to_live: Query<(Entity, &TimeToLive)>,
) {
    q_time_to_live.for_each(|(entity, time_to_live)| {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(Math::add(TimeToLive(-1)));
        });
        if time_to_live.0 <= 0 {
            despawn_events.send(DespawnEvent { entity });
        }
    });
}
