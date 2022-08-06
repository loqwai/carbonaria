use bevy::{
    math::Vec3,
    prelude::{AssetServer, Bundle, Res, Transform},
    sprite::SpriteBundle,
};
use heron::{CollisionShape, RigidBody, Velocity};

use crate::components::Mob;

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

type Position = (f32, f32);

impl MobBundle {
    pub fn new(asset_server: Res<AssetServer>, (x, y): Position) -> MobBundle {
        MobBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("mob.png"),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
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
            sprite_bundle: Default::default(),
        }
    }
}
