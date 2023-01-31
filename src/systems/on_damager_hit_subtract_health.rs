use bevy::prelude::*;

use crate::{
    components::{Damage, Health},
    events::DamagerHitEvent,
};

pub fn on_damager_hit_subtract_health(
    mut events: EventReader<DamagerHitEvent>,
    mut targets: Query<&mut Health>,
    damagers: Query<&Damage>,
    mut commands: Commands,
) {
    for event in events.iter() {

        match targets.get_mut(event.target) {
            Err(_) => continue,
            Ok(mut health) => {
                if health.0 == 0 {
                    continue;
                }

                // get the damage from the damager
                match damagers.get(event.damager) {
                    Err(_) => continue,
                    Ok(damage) => {
                        health.0 -= damage.0;
                    }
                }
                if health.0 <= 0 {
                    commands.entity(event.target).despawn_recursive();
                }
                // println!("damager: target: {:?}, health: {}", event.target, health.0);
            }
        }
    }
}
