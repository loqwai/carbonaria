use bevy::{
    prelude::*,
    sprite::{ SpriteBundle},
};
use bevy_rapier2d::prelude::*;

use crate::components::{LaserGunBullet, Speed,Damage, Health, TimeToLive};

const RADIUS: f32 = 64.0;

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
    pub time_to_live: TimeToLive,
}

impl LaserGunBulletBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        transform: &GlobalTransform,
        scale: f32,
    ) -> LaserGunBulletBundle {
        LaserGunBulletBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::ball(RADIUS * scale),
            damage: Damage(1),
            health: Health(1),
            name: "laser gun bullet".into(),
            sensor: Sensor,
            speed: Speed(10.0),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("bullet.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    ..Default::default()
                },
                transform: transform.clone().into(),
                ..Default::default()
            },
            tag: LaserGunBullet,
            time_to_live: TimeToLive(100),
        }
    }
}
