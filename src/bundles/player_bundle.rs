use bevy::{math::Vec3, prelude::Bundle, sprite::SpriteBundle};
use heron::{CollisionShape, RigidBody, Velocity};

use crate::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(32.0, 32.0, 0.0),
                border_radius: None,
            },
            velocity: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}
