use bevy::{
    prelude::*,
    sprite::{Anchor, SpriteBundle},
};
use heron::{CollisionShape, RigidBody};

use crate::components::Stick;

#[derive(Bundle)]
pub struct StickBundle {
    pub stick: Stick,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl StickBundle {
    pub fn new(asset_server: Res<AssetServer>, position: Vec3) -> StickBundle {
        StickBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("stick.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(-16.0 / 64.0, 0.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for StickBundle {
    fn default() -> Self {
        Self {
            stick: Stick,
            rigid_body: RigidBody::KinematicPositionBased,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(16.0, 4.0, 0.0),
                border_radius: None,
            },
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
        }
    }
}
