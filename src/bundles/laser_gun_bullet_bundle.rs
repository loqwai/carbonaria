use bevy::{
    prelude::*,
    sprite::{ SpriteBundle},
};
use bevy_rapier2d::prelude::*;

use crate::components::{LaserGunBullet, Speed,Damage, Health};

#[derive(Bundle)]
pub struct LaserGunBulletBundle {
    pub name: Name,
    pub tag: LaserGunBullet,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub speed: Speed,
    pub damage: Damage,
    pub health: Health,
}

impl LaserGunBulletBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        transform: &GlobalTransform,
    ) -> LaserGunBulletBundle {
        LaserGunBulletBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("bullet.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                transform: transform.clone().into(),
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
            speed: Speed(10.0),
            health: Health(1),
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
            collider: Collider::ball(64.0),
            damage: Damage(1),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
