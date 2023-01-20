use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Anchor, SpriteBundle},
};

use crate::components::LaserGun;

#[derive(Bundle)]
pub struct LaserGunBundle {
    pub name: Name,
    pub gun: LaserGun,
    pub sprite_bundle: SpriteBundle,
}

impl LaserGunBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
    ) -> LaserGunBundle {
        let name: Name = "laser gun".into();

        LaserGunBundle {
            name: name.clone(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("laser-gun.png"),
                transform: Transform {
                    rotation: Quat::from_rotation_z(-PI),
                    ..Default::default()
                },
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(-20.0 / 32.0, 0.0)),
                    custom_size: Some(Vec2::new(40.0, 20.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for LaserGunBundle {
    fn default() -> Self {
        Self {
            name: "laser-gun".into(),
            gun: LaserGun{
                cooldown: 0,
                cooldown_max: 99,
            },
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
        }
    }
}
