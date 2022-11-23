use bevy::{prelude::*, sprite::SpriteBundle};
use heron::{CollisionShape, Collisions, RigidBody};

use crate::components::{Chest, MousePos};

#[derive(Bundle)]
pub struct ChestBundle {
    pub chest: Chest,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub collisions: Collisions,
    pub mouse_pos: MousePos,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl ChestBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3, contents: Entity) -> ChestBundle {
        ChestBundle {
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

impl Default for ChestBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            rigid_body: RigidBody::Sensor,
            collision_shape: CollisionShape::Sphere { radius: 16.0 },
            collisions: Default::default(),
            chest: Chest::default(),
            mouse_pos: MousePos,
        }
    }
}
