use bevy::{prelude::*, sprite::SpriteBundle};
use heron::{RigidBody, CollisionShape, Collisions};

use crate::components::Chest;

#[derive(Bundle)]
pub struct SpeedChestBundle {
    pub chest: Chest,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub collisions: Collisions,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl SpeedChestBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3, contents:Entity) -> SpeedChestBundle {

        SpeedChestBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("powerup-speed.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
            chest: Chest {
                contents: Some(contents),
            },
            ..Default::default()
        }
    }
}

impl Default for SpeedChestBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            rigid_body: RigidBody::Sensor,
            collision_shape: CollisionShape::Sphere { radius: 16.0 },
            collisions: Default::default(),
            chest: Chest::default(),
        }
    }
}
