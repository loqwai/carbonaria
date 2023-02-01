use bevy::prelude::*;
use crate::components::{RateOfFire, LaserGun};

pub fn move_rate_of_fire_powerup_to_gun(
    mut commands: Commands,
    rate_of_fires: Query<(Entity, &RateOfFire, &Parent)>,
    childrens: Query<&Children>,
    mut guns: Query<&mut LaserGun>,
) {
    rate_of_fires.for_each(|(entity, rate_of_fire, parent)| {
        for child in childrens.get(parent.get()).unwrap() {
            match guns.get_mut(*child).ok() {
                None => (),
                Some(mut gun) => {
                    gun.cooldown_max /= rate_of_fire.0;
                    gun.cooldown = gun.cooldown.min(gun.cooldown_max);
                }
            }
        }
        commands.entity(entity).despawn_recursive();
    });
}