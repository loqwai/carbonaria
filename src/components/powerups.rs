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

#[derive(Clone, Component)]
pub struct RateOfFire(pub usize);