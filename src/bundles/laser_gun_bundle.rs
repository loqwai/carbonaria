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
    pub fn new(cooldown: usize) -> LaserGunBundle {
        LaserGunBundle {
            aimable: Aimable,
            global_transform: GlobalTransform::default(),
            gun: LaserGun {
                cooldown: 0,
                cooldown_max: cooldown,
            },
            name: "laser gun".into(),
            transform: Transform::default(),
        }
    }
}