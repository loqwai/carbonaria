use bevy::{
    math::Vec3,
    prelude::{AssetServer, Bundle, Res},
    sprite::SpriteBundle,
};
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

impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("player.png"),
                ..Default::default()
            },
            ..Default::default()
        }
    }
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
