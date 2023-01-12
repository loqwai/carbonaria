use crate::components::Speed;
use bevy::prelude::*;

pub fn update_speed_from_speedup(
    mut q_speedup: Query<(&Parent, &Speed)>,
    mut q_thing_to_speedup: Query<&mut Speed>,
) {
    for (parent, _) in q_speedup.iter_mut() {
        if let Ok(mut speed) = q_thing_to_speedup.get_mut(parent.get()) {
            speed.0 = 80.0
        }
    }
}
