use std::ops::AddAssign;

use bevy::prelude::*;

#[derive(Component)]
pub struct Speedup;

#[derive(Clone, Component, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Team(pub usize);

#[derive(Clone, Component, Reflect)]
pub struct Speed(pub f32);


#[derive(Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub usize);
impl Default for Health {
    fn default() -> Self {
        Health(10)
    }
}

#[derive(Clone, Component,Copy)]
pub struct RateOfFire(pub usize);
impl AddAssign for RateOfFire {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl Default for RateOfFire {
    fn default() -> Self {
        RateOfFire(0)
    }
}
#[derive(Component, Debug)]
pub struct AddPowerup<T: Component + AddAssign + Default>(pub T);
