use bevy::prelude::*;
use heron::{CollisionShape, Collisions, RigidBody};

use crate::components::Exit;

#[derive(Bundle)]
pub struct ExitBundle {
    pub exit: Exit,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub collisions: Collisions,

    pub sprite_bundle: SpriteBundle,
}

impl ExitBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3) -> ExitBundle {
        ExitBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("exit.png"),
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

impl Default for ExitBundle {
    fn default() -> Self {
        Self {
            exit: Exit {},
            rigid_body: RigidBody::Sensor,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(32.0, 32.0, 0.0),
                border_radius: None,
            },
            collisions: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}
