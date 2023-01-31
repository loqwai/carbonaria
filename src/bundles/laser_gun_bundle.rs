use bevy::{
    prelude::*,
};

use crate::components::{Aimable, LaserGun};

#[derive(Bundle)]
pub struct LaserGunBundle {
    pub aimable: Aimable,
    pub global_transform: GlobalTransform,
    pub gun: LaserGun,
    pub name: Name,
    pub transform: Transform,
}

impl LaserGunBundle {
    pub fn new() -> LaserGunBundle {
        LaserGunBundle {
            aimable: Aimable,
            global_transform: GlobalTransform::default(),
            gun: LaserGun {
                cooldown: 0,
                cooldown_max: 99,
            },
            name: "laser gun".into(),
            transform: Transform::default(),
        }
    }
}