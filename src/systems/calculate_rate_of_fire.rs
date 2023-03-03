use crate::components::{LaserGun, RateOfFire};
use bevy::prelude::*;

pub fn calculate_rate_of_fire(
    mut rate_of_fires: Query<&RateOfFire, With<Children>>,
    mut guns: Query<(&mut LaserGun, &Parent)>,
) {
    guns.for_each_mut(|(mut gun, parent)| {
        let Ok(rate_of_fire) = rate_of_fires.get_mut(parent.get()) else { return; };
        gun.cooldown_rate = rate_of_fire.0 as usize;
    });
}
