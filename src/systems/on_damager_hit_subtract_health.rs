use bevy::prelude::*;

use crate::{
    components::{Damage, Health},
    events::DamagerHitEvent,
};

pub fn on_damager_hit_subtract_health(
    mut hit_events: EventReader<DamagerHitEvent>,
    mut targets: Query<&mut Health, Without<Damage>>,
    mut damagers: Query<(&Damage, &mut Health)>,
) {
    for event in hit_events.iter() {
        match targets.get_mut(event.target) {
            Err(_) => continue,
            Ok(mut health) => {
                if health.0 == 0 {
                    continue;
                }

                // get the damage from the damager
                match damagers.get_mut(event.damager) {
                    Err(_) => continue,
                    Ok((damage, mut damager_health)) => {
                        health.0 = health.0.max(damage.0) - damage.0;
                        damager_health.0 = damager_health.0.max(damage.0) - damage.0;
                    }
                }
            }
        }
    }
}
