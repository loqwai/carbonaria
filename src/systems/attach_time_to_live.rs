use crate::components::{Math, TimeToLive};
use bevy::prelude::*;

pub fn attach_time_to_live(
    mut commands: Commands,
    q_time_to_live: Query<Entity, Without<TimeToLive>>,
    q_time_to_live_powerups: Query<&Parent, With<Math<TimeToLive>>>,
) {
    for parent in q_time_to_live_powerups.iter() {
        if q_time_to_live.get(parent.get()).is_ok() {
            commands
                .entity(parent.get())
                .insert(TimeToLive(isize::MAX / 2));
        }
    }
}
