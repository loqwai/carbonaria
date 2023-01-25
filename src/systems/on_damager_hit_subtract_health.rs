use bevy::prelude::*;

use crate::{
    components::{Damage, Health},
    events::DamagerHitEvent,
};

pub fn on_damager_hit_subtract_health(
    mut events: EventReader<DamagerHitEvent>,
    mut targets: Query<&mut Health>,
    damagers: Query<&Damage>,
) {
    for event in events.iter() {
        match targets.get_mut(event.target) {
            Err(_) => continue,
            Ok(mut health) => {
                if health.0 == 0 {
                    continue;
                }
                // get the damage from the damager
                let damage = damagers.get(event.damager).unwrap().0;
                health.0 -= damage;
                println!("damager: target: {:?}, health: {}", event.target, health.0);
            }
        }
    }
}
