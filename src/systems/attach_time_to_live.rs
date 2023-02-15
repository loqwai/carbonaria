use crate::components::{Math, TimeToLive};
use bevy::prelude::*;

pub fn attach_time_to_live(
    mut commands: Commands,
    q_time_to_live: Query<Without<TimeToLive>>,
    mut q_time_to_live_powerups: Query<&Parent, With<Math<TimeToLive>>>,
) {
    q_time_to_live_powerups.for_each_mut(|parent| {
        if let Ok(_) = q_time_to_live.get(parent.get()) {
            commands
                .entity(parent.get())
                .insert(TimeToLive(isize::MAX / 2));
        }
    });
}
