use crate::components::{LaserGun, RateOfFire};
use bevy::prelude::*;

pub fn calculate_rate_of_fire(
    rate_of_fires: Query<&RateOfFire, With<Children>>,
    mut guns: Query<(&mut LaserGun, &Parent)>,
) {
    for (mut gun, parent) in guns.iter_mut() {
        let Ok(rate_of_fire) = rate_of_fires.get(parent.get()) else { continue; };
        gun.cooldown_rate = rate_of_fire.0 as usize;
    }
}
