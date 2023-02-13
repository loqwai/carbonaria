use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Exit;

#[derive(Bundle)]
pub struct ExitBundle {
    pub exit: Exit,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub sensor: Sensor,
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
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(32.0, 32.0),
            sensor: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}
