use bevy::{
    prelude::*,
};

use crate::components::{ActiveAmmo, Aimable, LaserGun, AmmoType};

#[derive(Bundle)]
pub struct LaserGunBundle {
    pub aimable: Aimable,
    pub global_transform: GlobalTransform,
    pub active_ammo: ActiveAmmo,
    pub gun: LaserGun,
    pub name: Name,
    pub transform: Transform,
}

impl LaserGunBundle {
    pub fn new(cooldown: usize) -> LaserGunBundle {
        LaserGunBundle {
            aimable: Aimable,
            active_ammo: ActiveAmmo(AmmoType::Normal),
            global_transform: GlobalTransform::default(),
            gun: LaserGun {
                cooldown: 0,
                cooldown_max: cooldown,
                cooldown_rate: 10,
            },
            name: "laser gun".into(),
            transform: Transform::default(),
        }
    }
}
