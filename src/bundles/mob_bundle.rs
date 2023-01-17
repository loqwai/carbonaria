use bevy::{
    math::Vec3,
    prelude::{AssetServer, Bundle, Res, Transform},
    sprite::SpriteBundle,
};
use bevy_rapier2d::prelude::*;

use crate::components::Mob;

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub sprite_bundle: SpriteBundle,
}

impl MobBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3) -> MobBundle {
        MobBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("mob.png"),
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

impl Default for MobBundle {
    fn default() -> Self {
        Self {
            mob: Mob,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(16.0),
            velocity: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}
