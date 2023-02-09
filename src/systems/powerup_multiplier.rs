use std::ops::{MulAssign};

use bevy::prelude::*;

use crate::components::MultiplyPowerup;

pub fn powerup_multiplier<T: Component + MulAssign + Clone>(
    powerups: Query<(&Parent, &mut MultiplyPowerup<T>)>,
    mut powerup_target: Query<&mut T>,
) {

    powerups.for_each(|(parent, powerup)| {
        if let Ok(mut target) = powerup_target.get_mut(parent.get()) {
            *target *= powerup.0.clone();
        }
    });
}

#[test]
fn did_multiplier_multiply_correctly() {
    use crate::components::{Speed,AddPowerup};
    let mut app = App::new();

    let pu1 = MultiplyPowerup(Speed(3.0));
    let pu2 = MultiplyPowerup(Speed(4.0));
    let speed_add_powerup = AddPowerup(Speed(2.0));
    let laser_gun = app
        .world
        .spawn(Speed(0.0))
        .with_children(|parent| {
            parent.spawn(speed_add_powerup);
            parent.spawn(pu1);
            parent.spawn(pu2);
        })
        .id();

    app.add_system(super::powerup_defaulter::<Speed>);
    app.add_system(super::powerup_adder::<Speed>.after(super::powerup_defaulter::<Speed>));
    app.add_system(powerup_multiplier::<Speed>.after(super::powerup_adder::<Speed>));
    app.update();

    let speed = app.world.get::<Speed>(laser_gun).unwrap();
    assert_eq!(speed.0, 24.0);
    app.update();
    let speed = app.world.get::<Speed>(laser_gun).unwrap();
    assert_eq!(speed.0, 24.0);
}
