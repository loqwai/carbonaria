use std::ops::{Add, AddAssign};

use bevy::prelude::*;

use crate::{
    components::{RateOfFire, LaserGun},
};

#[derive(Component)]
pub struct Powerup<T:Component> {
    pub value: T,
}


pub fn powerup_adder(
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

#[test]
fn did_add_rate_of_fire_to_gun_cooldown_rate() {

    let mut app = App::new();

    let laser =  LaserGun {
        cooldown: 0,
        cooldown_max: 10,
        cooldown_rate: 1,
    };
    let laser_powerup = app.world.spawn(RateOfFire(1)).id();
    let gun_entity = app.world.spawn(laser).push_children(&[laser_powerup]).id();


    app.update();

    let gun = app.world.get::<LaserGun>(gun_entity).unwrap();
    assert_eq!(gun.cooldown_rate, 2);
}
