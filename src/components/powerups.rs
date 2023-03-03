use std::ops::{AddAssign, MulAssign};

use bevy::prelude::*;

#[derive(Component)]
pub struct Speedup;

#[derive(Clone, Component, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Team(pub usize);

#[derive(Clone, Component, Debug, Reflect, Copy)]
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

#[derive(Clone, Component, Reflect, Copy, Debug)]
#[reflect(Component)]
pub struct Health(pub f32);
impl Default for Health {
    fn default() -> Self {
        Health(Default::default())
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
impl PartialEq for Health {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Component, Debug)]
pub struct RateOfFire(pub f32);
impl AddAssign for RateOfFire {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl Default for RateOfFire {
    fn default() -> Self {
        RateOfFire(0.0)
    }
}
impl MulAssign for RateOfFire {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

#[derive(Clone, Component, Debug)]
pub struct TimeToLive(pub isize);
impl AddAssign for TimeToLive {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl Default for TimeToLive {
    fn default() -> Self {
        TimeToLive(0)
    }
}

impl MulAssign for TimeToLive {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

#[derive(Clone, Component, Debug)]
pub struct Poison(pub f32);
impl AddAssign for Poison {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl MulAssign for Poison {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}
impl Default for Poison {
    fn default() -> Self {
        Poison(Default::default())
    }
}

#[derive(Clone, Component, Debug)]
pub struct Math<T: Component + AddAssign + MulAssign + core::fmt::Debug> {
    pub add: Option<T>,
    pub multiply: Option<T>,
}

impl<T: Component + AddAssign + MulAssign + core::fmt::Debug> Math<T> {
    pub fn add(add: T) -> Math<T> {
        Math {
            add: Some(add),
            multiply: None,
        }
    }

    pub fn multiply(multiply: T) -> Math<T> {
        Math {
            add: None,
            multiply: Some(multiply),
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct AmmoCount(pub isize);

impl AddAssign for AmmoCount {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl MulAssign for AmmoCount {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}
impl PartialEq for AmmoCount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
