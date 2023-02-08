use std::ops::{Add, AddAssign};

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Powerup<T:Component> {
    pub value: T,
}
#[derive(Component,Debug)]
pub struct RateOfFire(pub usize);

pub fn powerup_adder(
    powerups: Query<(&Parent,&Powerup<RateOfFire>)>,
    mut guns: Query<&mut RateOfFire>,
) {
    println!("powerup_adder");
    powerups.for_each(|(parent, powerup)| {
        if let Ok(mut gun) = guns.get_mut(parent.get()) {
            gun.0 += powerup.value.0;
        }
    });
}

#[test]
fn did_add_rate_of_fire_to_gun_cooldown_rate() {

    let mut app = App::new();

    let pu1 =  Powerup::<RateOfFire> {
        value: RateOfFire(1),
    };
    let pu2 =  Powerup::<RateOfFire> {
        value: RateOfFire(2),
    };
    let laser_gun = app.world.spawn(RateOfFire(0)).with_children(|parent| {
        parent.spawn(pu1);
        parent.spawn(pu2);

    }).id();


    app.add_system(powerup_adder);
    app.update();

    let rate_of_fire = app.world.get::<RateOfFire>(laser_gun).unwrap();
    assert_eq!(rate_of_fire.0, 3);
}
