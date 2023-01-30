use bevy::prelude::*;

#[derive(Component)]
pub struct Speedup;

#[derive(Clone, Component, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Team(pub usize);

#[derive(Clone, Component, Reflect)]
pub struct Speed(pub f32);
impl Speed {
    pub fn fast() -> Speed {
        Speed(1.1)
    }

    pub fn slow() -> Speed {
        Speed(0.5)
    }
}

impl Default for Speed {
    fn default() -> Self {
        Speed(4.0)
    }
}


#[derive(Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub usize);
impl Default for Health {
    fn default() -> Self {
        Health(10)
    }
}
