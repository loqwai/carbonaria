use bevy::{prelude::*, sprite::SpriteBundle};
use heron::{RigidBody, CollisionShape, Collisions};

use crate::components::SpeedUp;

#[derive(Bundle)]
pub struct SpeedPowerupBundle {
    pub speed_up: SpeedUp,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub collisions: Collisions,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl SpeedPowerupBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3) -> SpeedPowerupBundle {
        SpeedPowerupBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("powerup-speed.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for SpeedPowerupBundle {
    fn default() -> Self {
        Self {
            speed_up: SpeedUp,
            sprite_bundle: Default::default(),
            rigid_body: RigidBody::Sensor,
            collision_shape: CollisionShape::Sphere { radius: 16.0 },
            collisions: Default::default(),
        }
    }
}