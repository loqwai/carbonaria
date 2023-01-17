use bevy::{
    math::Vec3,
    prelude::{AssetServer, Bundle, Res, Transform},
    sprite::SpriteBundle,
};
use heron::{CollisionShape, RigidBody, Velocity};

use crate::components::{Mob, Speed, Pocket};

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub velocity: Velocity,
    pub base_speed: Speed,
    pub pockets: Pocket,

    #[bundle]
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
            collision_shape: CollisionShape::Sphere { radius: 16.0 },
            velocity: Default::default(),
            base_speed: Speed::fast(),
            pockets: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}
