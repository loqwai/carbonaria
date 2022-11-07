use crate::components::{Speed,Speedup};
use bevy::prelude::*;

pub fn update_speed_from_speedup(
    mut q_speedup: Query<(&mut Speed, &Speedup)>,
) {
    for speed_component in q_speedup.iter_mut() {
        // let target_health = q_healths.get(parent.get()).unwrap(); // Maybe Bevy 0.8+ only?
       let mut speed = speed_component.0;
       speed.0 = 800.0;
    }
}
