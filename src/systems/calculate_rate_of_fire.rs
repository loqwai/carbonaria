use crate::{
    components::{LaserGun, RateOfFire},
};
use bevy::prelude::*;

pub fn calculate_rate_of_fire(
    rate_of_fires: Query<&RateOfFire>,
    childrens: Query<&Children>,
    mut guns: Query<(&Parent, &mut LaserGun)>,
) {
    guns.for_each_mut(|(parent, mut gun)| {

        if let Some(children) = childrens.get(parent.get()).ok() {
            let cooldown_rate = children
                .iter()
                .filter_map(|child| rate_of_fires.get(*child).ok())
                .map(|rate_of_fire| rate_of_fire.0)
                .sum();
            gun.cooldown_rate = cooldown_rate;
        }

    });
}
