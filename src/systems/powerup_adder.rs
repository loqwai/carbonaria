use std::ops::{AddAssign};

use bevy::prelude::*;

use crate::components::AddPowerup;



pub fn powerup_adder<T: Component + AddAssign + Default + Copy>(
    powerups: Query<(&Parent, &mut AddPowerup<T>)>,
    mut powerup_target: Query<&mut T>,
) {
    powerup_target.for_each_mut(|mut gun| {
        *gun = T::default();
    });
    powerups.for_each(|(parent, powerup)| {
        if let Ok(mut gun) = powerup_target.get_mut(parent.get()) {
            *gun += powerup.0;
        }
    });
}

#[test]
fn did_add_rate_of_fire_to_gun_cooldown_rate() {
    use crate::components::RateOfFire;
    let mut app = App::new();

    let pu1 = AddPowerup(RateOfFire(1));
    let pu2 = AddPowerup(RateOfFire(2));
    let laser_gun = app
        .world
        .spawn(RateOfFire(0))
        .with_children(|parent| {
            parent.spawn(pu1);
            parent.spawn(pu2);
        })
        .id();

    app.add_system(powerup_adder::<RateOfFire>);
    app.update();

    let rate_of_fire = app.world.get::<RateOfFire>(laser_gun).unwrap();
    assert_eq!(rate_of_fire.0, 3);
    app.update();
    let rate_of_fire = app.world.get::<RateOfFire>(laser_gun).unwrap();
    assert_eq!(rate_of_fire.0, 3);
}
