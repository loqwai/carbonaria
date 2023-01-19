use bevy::{
    prelude::*,
    sprite::{ SpriteBundle},
};

use crate::components::{LaserGunBullet, Speed};

#[derive(Bundle)]
pub struct LaserGunBulletBundle {
    pub name: Name,
    pub tag: LaserGunBullet,
    pub sprite_bundle: SpriteBundle,
    pub speed: Speed,
}

impl LaserGunBulletBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        transform: &GlobalTransform,
    ) -> LaserGunBulletBundle {
        LaserGunBulletBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("stick.png"),
                transform: transform.clone().into(),
                sprite: Sprite {
                    // anchor: Anchor::Custom(Vec2::new(-20.0 / 32.0, 0.0)),
                    custom_size: Some(Vec2::new(40.0, 20.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for LaserGunBulletBundle {
    fn default() -> Self {
        Self {
            name: "laser gun bullet".into(),
            tag: LaserGunBullet,
            speed: Speed(20.0),
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
        }
    }
}
