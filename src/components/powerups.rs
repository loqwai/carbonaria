use std::ops::{AddAssign, MulAssign};

use bevy::prelude::*;

#[derive(Component)]
pub struct Speedup;

#[derive(Clone, Component, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Team(pub usize);

#[derive(Clone, Component, Reflect,Copy)]
pub struct Speed(pub f32);
impl Default for Speed {
    fn default() -> Self {
        Speed(0.0)
    }
}
impl AddAssign for Speed {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl MulAssign for Speed {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

#[derive(Clone, Component, Reflect,Copy)]
#[reflect(Component)]
pub struct Health(pub isize);
impl Default for Health {
    fn default() -> Self {
        Health(0)
    }
}
impl AddAssign for Health {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl MulAssign for Health {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}


#[derive(Clone, Component)]
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
impl MulAssign for RateOfFire {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

#[derive(Clone, Component)]
pub struct MathPowerup<T: Component + AddAssign + MulAssign> {
    pub add: Option<T>,
    pub multiply: Option<T>,
}
