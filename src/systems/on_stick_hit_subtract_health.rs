use bevy::prelude::*;

use crate::{components::Health, events::StickHitEvent};

pub fn on_stick_hit_subtract_health(
    mut events: EventReader<StickHitEvent>,
    mut q_health: Query<&mut Health>,
) {
    for event in events.iter() {
        match q_health.get_mut(event.target) {
            Err(_) => continue,
            Ok(mut health) => {
                health.0 -= 1;
                println!("health: {}", health.0);
            }
        }
    }
}
